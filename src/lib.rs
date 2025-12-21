// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Simple options and arguments processor that builds on getopts.
//!
//! # Usage
//!
//! The basic usage of getoptsargs is to define the full `main` method of your
//! application as shown in the skeleton below.
//!
//! ```rust
//! use getoptsargs::prelude::*;
//!
//! fn app_setup(builder: Builder) -> Builder {
//!     builder
//! }
//!
//! fn app_main(_matches: Matches) -> Result<i32> {
//!     Ok(0)
//! }
//!
//! app!("Stylized App Name", app_setup, app_main);
//! ```
//!
//! The `app_setup` function uses the `builder` argument to define the options and
//! arguments that the application exposes.  The `app_main` function contains the
//! application's main program and is executed _after_ the command line arguments
//! have been processed.

#![allow(clippy::collapsible_else_if)]
#![warn(anonymous_parameters, bad_style, missing_docs)]
#![warn(unused, unused_extern_crates, unused_import_braces, unused_qualifications)]
#![warn(unsafe_code)]

use anyhow::Result;
use getopts::Matches as OptionMatches;
use getopts::Options;
use std::env;
use std::io;

mod args;
use args::{Arguments, Matches as ArgumentMatches};

mod errors;
use errors::UsageError;

mod licenses;
use licenses::License;

mod opts;

pub mod prelude;

mod run;

pub mod testutils;

/// Contains the result of options and arguments parsing.
#[derive(Debug)]
pub struct Matches {
    /// The program name.
    pub program_name: String,

    /// The option matches.
    opts: OptionMatches,

    /// The argument matches.
    args: ArgumentMatches,
}

/// Container for the metadata about the user-defined application.
struct App {
    stylized_name: &'static str,
    version: &'static str,
    program_name: String,
    copyright: Option<&'static str>,
    license: Option<License>,
    manpage: Option<(&'static str, &'static str)>,
    homepage: Option<&'static str>,
    bugs: Option<&'static str>,
    extra_help: Option<fn(&mut dyn io::Write) -> io::Result<()>>,
    #[cfg(feature = "env_logger")]
    init_env_logger: bool,
}

/// Builder for the user-defined application.
#[must_use]
pub struct Builder {
    app: App,
    env_args: env::Args,
    opts: Options,
    args: Arguments,
}

impl Builder {
    /// Creates a new application.
    ///
    /// `stylized_name` specifies the name of the program, irrespective of how it was called.  This
    /// is used in `--version` output and other places where needed.  This is also used (in its
    /// lowercase form) as the actual program name if program name auto-determination fails.
    ///
    /// `version` specifies the version of the program, and is typically the value of
    /// `env!("CARGO_PKG_VERSION")`.
    ///
    /// `copyright` is a user-provided string that must start with `Copyright` and that provides an
    /// informational copyright message for the user.
    ///
    /// `args` should always be `env::args()`.
    pub fn new(stylized_name: &'static str, version: &'static str, env_args: env::Args) -> Self {
        let (program_name, env_args) = run::program_name(env_args, stylized_name.to_lowercase());

        let mut opts = Options::new();
        opts.optflag("h", "help", "show command-line usage information and exit");
        opts.optflag("", "version", "show version information and exit");

        let license = License::from_cargo();

        let app = App {
            stylized_name,
            version,
            program_name,
            copyright: None,
            license,
            manpage: None,
            homepage: None,
            bugs: None,
            extra_help: None,
            #[cfg(feature = "env_logger")]
            init_env_logger: true,
        };

        Self { app, env_args, opts, args: Arguments::default() }
    }

    /// Sets the bug reporting URL of the application to `bugs`.
    pub fn bugs(mut self, bugs: &'static str) -> Self {
        self.app.bugs = Some(bugs);
        self
    }

    /// Sets the copyright of the application to `copyright`.  The provided string must be prefixed
    /// with `Copyright `.
    pub fn copyright(mut self, copyright: &'static str) -> Self {
        assert!(copyright.starts_with("Copyright "));
        self.app.copyright = Some(copyright);
        self
    }

    /// Registers a function that prints additional help when `--help` is requested.
    pub fn extra_help(mut self, extra_help: fn(&mut dyn io::Write) -> io::Result<()>) -> Self {
        self.app.extra_help = Some(extra_help);
        self
    }

    /// Sets the homepage of the application to `homepage`.
    pub fn homepage(mut self, homepage: &'static str) -> Self {
        self.app.homepage = Some(homepage);
        self
    }

    /// Sets the license of the application to `license`.
    pub fn license(mut self, license: License) -> Self {
        self.app.license = Some(license);
        self
    }

    /// Sets the manual page for the application to `page` in `section`.
    pub fn manpage(mut self, page: &'static str, section: &'static str) -> Self {
        self.app.manpage = Some((page, section));
        self
    }

    /// Tells the runtime to _not_ init the env logger so that the caller can do so at the
    /// best moment (e.g. if the caller is daemonizing and only wants to start console
    /// logging once a log file has been opened) via an explicit call to `init_env_logger`.
    #[cfg(feature = "env_logger")]
    pub fn disable_init_env_logger(mut self) -> Self {
        self.app.init_env_logger = false;
        self
    }

    /// Processes arguments as previously configured and handles standard non-configurable
    /// options like `--help` or `--version`.
    ///
    /// Returns `None` if the application should exit immediately _without_ error because one
    /// of the standard options was processed.  Otherwise, returns a `Matches` object with the
    /// results of the argument parsing.
    ///
    /// Prefer to use the `run` method in conjunction with the `app!` macro to hide the details
    /// of handling the complex return semantics of the returned type.  This function exists
    /// only to let you implement a completely imperative program without any flow control
    /// redirections.
    pub fn start(self) -> Result<Option<Matches>> {
        run::pre_run(&self.app, self.opts, self.args, self.env_args)
    }

    /// Starts the application delegating execution to `main`.
    ///
    /// Returns the exit code that the caller must propagate to the caller via `process::exit`.
    pub fn run(self, main: fn(Matches) -> Result<i32>) -> i32 {
        match run::pre_run(&self.app, self.opts, self.args, self.env_args) {
            Ok(None) => 0,
            Ok(Some(matches)) => match main(matches) {
                Ok(code) => code,
                Err(e) => run::handle_error(&self.app, e),
            },
            Err(e) => run::handle_error(&self.app, e),
        }
    }

    /// Async version of `run`.
    pub async fn run_async<F: Future<Output = Result<i32>>>(self, main: fn(Matches) -> F) -> i32 {
        match run::pre_run(&self.app, self.opts, self.args, self.env_args) {
            Ok(None) => 0,
            Ok(Some(matches)) => match main(matches).await {
                Ok(code) => code,
                Err(e) => run::handle_error(&self.app, e),
            },
            Err(e) => run::handle_error(&self.app, e),
        }
    }
}

/// Defines the `main` entry point for a new app.
#[macro_export]
macro_rules! app {
    ( $name:literal, $builder:ident, $main:ident ) => {
        fn main() {
            let mut builder =
                $crate::Builder::new($name, env!("CARGO_PKG_VERSION"), std::env::args());
            builder = $builder(builder);
            let exit_code = builder.run($main);
            std::process::exit(exit_code);
        }
    };
}

/// Defines the `main` entry point for a new async app using the tokio runtime.
#[macro_export]
macro_rules! tokio_app {
    ( $name:literal, $builder:ident, $main:ident ) => {
        #[tokio::main]
        async fn main() {
            let mut builder =
                $crate::Builder::new($name, env!("CARGO_PKG_VERSION"), std::env::args());
            builder = $builder(builder);
            let exit_code = builder.run_async($main).await;
            std::process::exit(exit_code);
        }
    };
}
