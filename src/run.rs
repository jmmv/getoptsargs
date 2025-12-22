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

use crate::{App, Arguments, License, Matches, UsageError};
use anyhow::Result;
use getopts::Options;
use std::env;
use std::error::Error;
use std::io;
use std::path::Path;

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

/// Prints usage information for program `name` with `opts` following the GNU Standards format.
pub(crate) fn help(
    stylized_name: &str,
    program_name: &str,
    bugs: Option<&str>,
    homepage: Option<&str>,
    extra_help: Option<fn(&mut dyn io::Write) -> io::Result<()>>,
    opts: &Options,
    args: &Arguments,
) {
    let mut brief = format!("Usage: {} [options]", program_name);
    let args_usage = args.brief();
    if !args_usage.is_empty() {
        brief.push(' ');
        brief.push_str(&args_usage);
    }

    println!("{}", opts.usage(&brief));
    if !args_usage.is_empty() {
        println!("{}", args.usage());
    }

    if let Some(extra_help) = extra_help {
        let _ = extra_help(&mut io::stdout().lock());
        println!();
    }

    if let Some(bugs) = bugs {
        println!("Report bugs to: {}", bugs);
    }
    if let Some(homepage) = homepage {
        println!("{} home page: {}", stylized_name, homepage);
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

/// Handles non-configurable options before program start (such as `--help` and `--version`).
pub(crate) fn pre_run(
    app: &App,
    opts: Options,
    args: Arguments,
    env_args: env::Args,
) -> Result<Option<Matches>> {
    let mut opt_matches = opts.parse(env_args)?;

    if opt_matches.opt_present("help") {
        help(
            app.stylized_name,
            &app.program_name,
            app.bugs,
            app.homepage,
            app.extra_help,
            &opts,
            &args,
        );
        return Ok(None);
    }

    if opt_matches.opt_present("version") {
        version(app.stylized_name, app.version, app.copyright, app.license);
        return Ok(None);
    }

    let arg_matches = args.parse(opt_matches.free.split_off(0))?;

    #[cfg(feature = "env_logger")]
    if app.init_env_logger {
        init_env_logger(&app.program_name);
    }

    Ok(Some(Matches {
        program_name: app.program_name.clone(),
        opts: opt_matches,
        args: arg_matches,
    }))
}

/// Prints a usage error `e` to stderr.
///
/// Usage errors should only display the error message and a very brief mention on how to request
/// help.  This does not print the full help message by design as that would be distracting, yet
/// that's what other option parsing libraries like to do.
pub(crate) fn print_usage_error<E: Error>(app: &App, e: E) {
    eprintln!("Usage error: {}", e);
    match app.manpage {
        Some((page, section)) => eprintln!(
            "Type `{} --help` or `man {} {}` for more information",
            app.program_name, section, page
        ),
        None => eprintln!("Type `{} --help` for more information", app.program_name),
    }
}

/// Handles the error returned from the app's main function, printing it to the console in the
/// correct form and transforming it to the exit status to return to the user.
pub(crate) fn handle_error(app: &App, e: anyhow::Error) -> i32 {
    if let Some(e) = e.downcast_ref::<UsageError>() {
        print_usage_error(app, e);
        2
    } else if let Some(e) = e.downcast_ref::<getopts::Fail>() {
        print_usage_error(app, e);
        2
    } else {
        eprintln!("{}: {}", app.program_name, e);
        1
    }
}
