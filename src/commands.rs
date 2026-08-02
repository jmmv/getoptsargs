// getoptsargs
// Copyright 2026 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://www.opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Subcommand registration and dispatch.

use crate::{AppMetadata, Arguments, Builder, Matches, UsageError, run};
use anyhow::{Result, anyhow};
use getopts::Options;
use std::collections::BTreeMap;
use std::future::Future;
use std::io;
use std::pin::Pin;

/// Type of an async command's `main` function.
type AsyncMain = Box<dyn Fn(Matches, Matches) -> Pin<Box<dyn Future<Output = Result<i32>>>>>;

/// Type of an sync command's `main` function.
type SyncMain = fn(Matches, Matches) -> Result<i32>;

/// A command handler implementation.
enum Main {
    /// An asynchronous command handler.
    Async(AsyncMain),

    /// A synchronous command handler.
    Sync(SyncMain),
}

/// A registered command definition.
pub(crate) struct Command {
    /// The command name.
    name: &'static str,

    /// A short command description.
    description: &'static str,

    /// The function that configures the command.
    setup: fn(CommandBuilder) -> CommandBuilder,

    /// The function that executes the command.
    main: Main,
}

/// Context retained to dispatch a command after application parsing.
pub(crate) struct CommandContext {
    /// The application metadata.
    pub(crate) metadata: AppMetadata,

    /// The registered commands indexed by name.
    pub(crate) commands: BTreeMap<&'static str, Command>,

    /// The application option definitions.
    pub(crate) opts: Options,
}

/// Builder for a user-defined command.
#[must_use]
pub struct CommandBuilder {
    /// The command option definitions.
    pub(crate) opts: Options,

    /// The command argument definitions.
    pub(crate) args: Arguments,

    /// Additional help for this command.
    pub(crate) extra_help: Option<fn(&mut dyn io::Write) -> io::Result<()>>,
}

impl CommandBuilder {
    /// Creates a new command.
    pub fn new() -> Self {
        Self { opts: Options::new(), args: Arguments::default(), extra_help: None }
    }

    /// Registers a function that prints additional help for this command.
    pub fn extra_help(mut self, extra_help: fn(&mut dyn io::Write) -> io::Result<()>) -> Self {
        self.extra_help = Some(extra_help);
        self
    }
}

impl Default for CommandBuilder {
    /// Creates a default command builder.
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Registers a synchronous command.
    pub fn cmd(
        mut self,
        name: &'static str,
        description: &'static str,
        setup: fn(CommandBuilder) -> CommandBuilder,
        main: SyncMain,
    ) -> Self {
        self.register_command(name, description, setup, Main::Sync(main));
        self
    }

    /// Registers an asynchronous command.
    pub fn cmd_async<F, Fut>(
        mut self,
        name: &'static str,
        description: &'static str,
        setup: fn(CommandBuilder) -> CommandBuilder,
        main: F,
    ) -> Self
    where
        F: Fn(Matches, Matches) -> Fut + 'static,
        Fut: Future<Output = Result<i32>> + 'static,
    {
        let main = Box::new(move |app_matches, command_matches| {
            Box::pin(main(app_matches, command_matches)) as _
        });
        self.register_command(name, description, setup, Main::Async(main));
        self
    }

    /// Validates and stores a command definition.
    fn register_command(
        &mut self,
        name: &'static str,
        description: &'static str,
        setup: fn(CommandBuilder) -> CommandBuilder,
        main: Main,
    ) {
        assert!(self.args.is_empty(), "Cannot register commands with root arguments");
        assert!(!name.is_empty(), "Command name cannot be empty");
        assert!(name != "help" && name != "version", "Command name is reserved");
        assert!(!self.commands.contains_key(name), "Command name is already registered");
        self.app.has_commands = true;
        self.commands.insert(name, Command { name, description, setup, main });
    }
}

/// Returns user-defined and built-in command descriptions in name order.
fn command_descriptions(
    commands: &BTreeMap<&'static str, Command>,
) -> Vec<(&'static str, &'static str)> {
    let mut descriptions = commands
        .values()
        .map(|command| (command.name, command.description))
        .collect::<BTreeMap<_, _>>();
    descriptions.insert("help", "show command-line usage information");
    descriptions.insert("version", "show version information");
    descriptions.into_iter().collect()
}

/// Prints usage information for a command.
fn command_help(metadata: AppMetadata, program_name: &str, app_opts: &Options, command: &Command) {
    let builder = (command.setup)(CommandBuilder::new());
    let app_has_options = run::options_defined(app_opts);
    let program_name = if app_has_options {
        format!("{} [options] {}", program_name, command.name)
    } else {
        format!("{} {}", program_name, command.name)
    };
    let options_placeholder = if app_has_options { "[command options]" } else { "[options]" };
    run::help(
        &program_name,
        run::HelpConfig {
            metadata,
            opts: &builder.opts,
            args: &builder.args,
            commands: &[],
            extra_help: builder.extra_help,
            options_placeholder,
        },
    );
}

/// Prints usage information for a built-in command.
fn built_in_command_help(
    metadata: AppMetadata,
    program_name: &str,
    app_opts: &Options,
    name: &str,
) {
    let app_has_options = run::options_defined(app_opts);
    let program_name = if app_has_options {
        format!("{} [options] {}", program_name, name)
    } else {
        format!("{} {}", program_name, name)
    };
    let opts = Options::new();
    let mut args = Arguments::default();
    if name == "help" {
        args.trailing("command", 0, 1, "command to show help for");
    }
    run::help(
        &program_name,
        run::HelpConfig {
            metadata,
            opts: &opts,
            args: &args,
            commands: &[],
            extra_help: None,
            options_placeholder: "[options]",
        },
    );
}

/// Handles the built-in help command.
fn help_command(
    metadata: AppMetadata,
    program_name: &str,
    opts: &Options,
    commands: &BTreeMap<&'static str, Command>,
    args: &[String],
) -> Result<()> {
    match args {
        [] => {
            run::help(
                program_name,
                run::HelpConfig {
                    metadata,
                    opts,
                    args: &Arguments::default(),
                    commands: &command_descriptions(commands),
                    extra_help: None,
                    options_placeholder: "[options]",
                },
            );
            Ok(())
        }
        [name] => {
            if name == "help" || name == "version" {
                built_in_command_help(metadata, program_name, opts, name);
                return Ok(());
            }
            let Some(command) = commands.get(name.as_str()) else {
                return Err(UsageError { message: format!("Unknown command `{}`", name) }.into());
            };
            command_help(metadata, program_name, opts, command);
            Ok(())
        }
        _ => Err(UsageError { message: "Too many arguments".to_owned() }.into()),
    }
}

/// Extracts the selected command and its matches from application matches.
fn prepare_dispatch(mut app_matches: Matches) -> Result<Option<(Matches, Matches, Main)>> {
    let Some(CommandContext { metadata, mut commands, opts }) = app_matches.cmd_ctx.take() else {
        return Err(anyhow!("No commands registered"));
    };
    let mut command_args = app_matches.take_arg_trail().into_iter();
    let Some(command_name) = command_args.next() else {
        return Err(UsageError { message: "No command provided".to_owned() }.into());
    };
    let command_args = command_args.collect::<Vec<_>>();

    match command_name.as_str() {
        "help" => {
            help_command(metadata, &app_matches.program_name, &opts, &commands, &command_args)?;
            Ok(None)
        }

        "version" => {
            if !command_args.is_empty() {
                return Err(UsageError { message: "Too many arguments".to_owned() }.into());
            }
            run::version(
                metadata.stylized_name,
                metadata.version,
                metadata.copyright,
                metadata.license,
            );
            Ok(None)
        }

        command_name => {
            let Some(command) = commands.remove(command_name) else {
                return Err(
                    UsageError { message: format!("Unknown command `{command_name}`") }.into()
                );
            };

            let builder = (command.setup)(CommandBuilder::new());
            let mut opts = builder.opts.parse(command_args)?;
            let args = builder.args.parse(opts.free.split_off(0))?;
            let matches = Matches {
                program_name: format!("{} {}", app_matches.program_name, command.name),
                opts,
                args,
                cmd_ctx: None,
            };

            Ok(Some((app_matches, matches, command.main)))
        }
    }
}

/// Dispatches a synchronous command application.
pub fn command_dispatcher(matches: Matches) -> Result<i32> {
    let Some((app_matches, command_matches, main)) = prepare_dispatch(matches)? else {
        return Ok(0);
    };
    match main {
        Main::Sync(main) => main(app_matches, command_matches),
        Main::Async(_) => unreachable!("Async command requires tokio_app!"),
    }
}

/// Dispatches a command application from a Tokio runtime.
pub async fn tokio_command_dispatcher(matches: Matches) -> Result<i32> {
    let Some((app_matches, command_matches, main)) = prepare_dispatch(matches)? else {
        return Ok(0);
    };
    match main {
        Main::Async(main) => main(app_matches, command_matches).await,
        Main::Sync(main) => main(app_matches, command_matches),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;

    /// Creates an application configured for command dispatch tests.
    fn app() -> App {
        App {
            metadata: AppMetadata {
                stylized_name: "Test",
                version: "0",
                copyright: None,
                license: None,
                homepage: None,
                bugs: None,
                extra_help: None,
            },
            program_name: "test".to_owned(),
            manpage: None,
            has_commands: true,
            #[cfg(feature = "env_logger")]
            init_env_logger: false,
        }
    }

    /// Configures a command that accepts one positional argument.
    fn command_setup(builder: CommandBuilder) -> CommandBuilder {
        builder.posarg("argument", "irrelevant")
    }

    /// Verifies the parsed matches supplied to a command handler.
    fn command_main(app_matches: Matches, command_matches: Matches) -> Result<i32> {
        assert!(app_matches.arg_trail().is_empty());
        assert_eq!("value", command_matches.arg_pos("argument"));
        Ok(0)
    }

    #[test]
    fn test_dispatch_reads_and_consumes_trailing_arguments() {
        let mut args = Arguments::default();
        args.trailing("_COMMAND_ARGS", 0, usize::MAX, "");
        let args = args.parse(vec!["command".to_owned(), "value".to_owned()]).unwrap();

        let mut commands = BTreeMap::new();
        commands.insert(
            "command",
            Command {
                name: "command",
                description: "irrelevant",
                setup: command_setup,
                main: Main::Sync(command_main),
            },
        );

        let matches = Matches {
            program_name: "test".to_owned(),
            opts: Options::new().parse(Vec::<String>::new()).unwrap(),
            args,
            cmd_ctx: Some(CommandContext {
                metadata: app().metadata,
                commands,
                opts: Options::new(),
            }),
        };

        assert_eq!(0, command_dispatcher(matches).unwrap());
    }
}
