// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Execution logic.

use crate::{App, AppMetadata, Arguments, Command, CommandContext, License, Matches, UsageError};
use anyhow::Result;
use getopts::{Options, ParsingStyle};
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::io;
use std::path::Path;

/// Configuration for rendering usage information.
pub(crate) struct HelpConfig<'a> {
    /// The application metadata.
    pub(crate) metadata: AppMetadata,

    /// Positional and trailing argument definitions.
    pub(crate) args: &'a Arguments,

    /// Command names and descriptions.
    pub(crate) commands: &'a [(&'a str, &'a str)],

    /// Additional help specific to this invocation.
    pub(crate) extra_help: Option<fn(&mut dyn io::Write) -> io::Result<()>>,

    /// Option definitions.
    pub(crate) opts: &'a Options,

    /// Usage placeholder for the option definitions.
    pub(crate) options_placeholder: &'static str,
}

/// Returns whether `opts` contains any option definitions.
pub(crate) fn options_defined(opts: &Options) -> bool {
    opts.usage("").trim() != "Options:"
}

/// Consumes and returns the program name from `env::Args`.
///
/// If the program name cannot be obtained, return `default_name` instead.
pub(crate) fn program_name<S: Into<String>>(
    mut args: env::Args,
    default_name: S,
) -> (String, env::Args) {
    let name = match args.next() {
        Some(arg0) => match Path::new(&arg0).file_stem() {
            Some(basename) => match basename.to_str() {
                Some(s) => s.to_owned(),
                None => default_name.into(),
            },
            None => default_name.into(),
        },
        None => default_name.into(),
    };
    (name, args)
}

/// Prints usage information for `program_name` with `config` following the GNU Standards format.
pub(crate) fn help(program_name: &str, config: HelpConfig<'_>) {
    let mut brief = format!("Usage: {}", program_name);
    let has_options = options_defined(config.opts);
    if has_options {
        brief.push(' ');
        brief.push_str(config.options_placeholder);
    }
    let args_usage = config.args.brief();
    if !args_usage.is_empty() {
        brief.push(' ');
        brief.push_str(&args_usage);
    }
    if !config.commands.is_empty() {
        brief.push_str(" command [arg...]");
    }

    if has_options {
        println!("{}", config.opts.usage(&brief));
    } else {
        println!("{}", brief);
        println!();
    }
    if !args_usage.is_empty() {
        println!("{}", config.args.usage());
    }

    if !config.commands.is_empty() {
        println!("Commands:");
        for (name, description) in config.commands {
            println!("    {:<20}{}", name, description);
        }
        println!();
    }
    if let Some(extra_help) = config.metadata.extra_help {
        let _ = extra_help(&mut io::stdout().lock());
        println!();
    }
    if let Some(extra_help) = config.extra_help {
        let _ = extra_help(&mut io::stdout().lock());
        println!();
    }

    if let Some(bugs) = config.metadata.bugs {
        println!("Report bugs to: {}", bugs);
    }
    if let Some(homepage) = config.metadata.homepage {
        println!("{} home page: {}", config.metadata.stylized_name, homepage);
    }
}

/// Prints version information following the GNU Standards format.
pub(crate) fn version(
    stylized_name: &str,
    version: &str,
    copyright: Option<&str>,
    license: Option<License>,
) {
    println!("{} {}", stylized_name, version);
    if let Some(copyright) = copyright {
        println!("{}", copyright);
    }
    if let Some(license) = license {
        println!("License {}", license)
    }
}

/// Initialize the logger with settings that match traditional apps.
///
/// This allows apps to use the `log::error`, `log::warn`, and `log::info` macros (by
/// default, but other levels can be used too) for progress reporting and makes the messages
/// "blend" with other command line apps.  The default logger configuration generates lines
/// more suited for log files, not human consumption.
#[cfg(feature = "env_logger")]
pub fn init_env_logger<P: Into<String>>(program_name: P) {
    use std::io::Write;
    let mut builder =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    {
        let program_name = program_name.into();
        builder.format(move |buf, record| {
            writeln!(buf, "{}: {}: {}", program_name, record.level(), record.args())
        });
    }
    builder.init()
}

/// Handles standard options and prepares application matches.
pub(crate) fn pre_run<I>(
    app: &App,
    mut opts: Options,
    mut args: Arguments,
    env_args: I,
    commands: BTreeMap<&'static str, Command>,
) -> Result<Option<Matches>>
where
    I: IntoIterator<Item = String>,
{
    let program_name = app.program_name.clone();

    #[cfg(feature = "env_logger")]
    if app.init_env_logger {
        init_env_logger(&program_name);
    }

    if commands.is_empty() {
        opts.optflag("h", "help", "show command-line usage information and exit");
        opts.optflag("", "version", "show version information and exit");
    } else {
        assert!(args.is_empty(), "Cannot register commands with root arguments");
        args.trailing("_COMMAND_ARGS", 0, usize::MAX, "");
        opts.parsing_style(ParsingStyle::StopAtFirstFree);
    }
    let mut opt_matches = opts.parse(env_args)?;

    let context = if commands.is_empty() {
        if opt_matches.opt_present("help") {
            help(
                &app.program_name,
                HelpConfig {
                    metadata: app.metadata,
                    opts: &opts,
                    args: &args,
                    commands: &[],
                    extra_help: None,
                    options_placeholder: "[options]",
                },
            );
            return Ok(None);
        }

        if opt_matches.opt_present("version") {
            version(
                app.metadata.stylized_name,
                app.metadata.version,
                app.metadata.copyright,
                app.metadata.license,
            );
            return Ok(None);
        }

        None
    } else {
        Some(CommandContext { metadata: app.metadata, commands, opts })
    };

    let arg_matches = args.parse(opt_matches.free.split_off(0))?;

    Ok(Some(Matches { program_name, opts: opt_matches, args: arg_matches, cmd_ctx: context }))
}

/// Prints a usage error `e` to stderr.
///
/// Usage errors should only display the error message and a very brief mention on how to request
/// help.  This does not print the full help message by design as that would be distracting, yet
/// that's what other option parsing libraries like to do.
pub(crate) fn print_usage_error<E: Error>(app: &App, e: E) {
    eprintln!("Usage error: {}", e);
    let help = if app.has_commands {
        format!("{} help", app.program_name)
    } else {
        format!("{} --help", app.program_name)
    };
    match app.manpage {
        Some((page, section)) => {
            eprintln!("Type `{}` or `man {} {}` for more information", help, section, page)
        }
        None => eprintln!("Type `{}` for more information", help),
    }
}

/// Handles the error returned from the app's main function, printing it to the console in the
/// correct form and transforming it to the exit status to return to the user.
///
/// Errors that are not usage errors are printed with all of their causes, separated by colons,
/// because the outermost message alone rarely says why anything failed: an app that reports
/// "cannot open the configuration file" is only useful when it also says that the file is not
/// there.  Usage errors have no causes to print, being about what the user typed.
pub(crate) fn handle_error(app: &App, e: anyhow::Error) -> i32 {
    if let Some(e) = e.downcast_ref::<UsageError>() {
        print_usage_error(app, e);
        2
    } else if let Some(e) = e.downcast_ref::<getopts::Fail>() {
        print_usage_error(app, e);
        2
    } else {
        eprintln!("{}: {:#}", app.program_name, e);
        1
    }
}
