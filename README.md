# getoptsargs

[![Crates.io](https://img.shields.io/crates/v/getoptsargs.svg)](https://crates.io/crates/getoptsargs/)
[![Docs.rs](https://docs.rs/getoptsargs/badge.svg)](https://docs.rs/getoptsargs/)

getoptsargs is a simple Rust library to process options and arguments in
command-line applications.  It is intended to be a small wrapper over getopts,
an "ancient" library still used by no other than rustc, and to supplement it
with argument handling.

getoptsargs is free software under the [Apache 2.0 License](LICENSE-APACHE) or
the [MIT License](LICENSE-MIT) at your discretion.

## Highlights

*   getoptsargs wants to offer and end-to-end implementation of `main`, not
    just argument parsing, by letting you write your `main` program to return a
    `Result<i32>`.  This is to make it easier to propagate errors _and_ make it
    clear that exit codes are an important communication mechanism with the
    calling program.

*   The visual interface exposed by getoptsargs tries to adhere to the [GNU
    Coding Standards for command line
    interfaces](https://www.gnu.org/prep/standards/html_node/Command_002dLine-Interfaces.html)
    as much as possible.  Application using getoptsargs will feel "right at
    home" with other traditional tools in a Unix-like system.

*   The API in getoptsargs is imperative, not derivative as is found in the
    more-popular `clap` or `argh` crates.  This makes the code less magical,
    albeit more verbose at times.

*   getoptsargs provides auxiliary test utilities to perform end-to-end testing
    of the _executable_, not just of internal APIs.  These help ensure that the
    standard out and standard error of the program behave exactly as expected,
    which is important in offering high-quality, detail-oriented command line
    tools.

## Lowlights

*   The interface exposed by getoptsargs intentionally mimics the getopts it
    inherits from, and getopts is not the cleanest Rust API.  You'll notice some
    rough edges and terse names---but that's intentional to keep things simple
    and consistent.

*   There is no support for subcommand-based interfaces.  This is currently out
    of scope for this library, but if there was interest, it could be added.

## Usage

The basic structure of a getoptsargs application looks like this:

``` rust
use getoptsargs::prelude::*;

fn app_setup(builder: Builder) -> Builder {
    builder
}

fn app_main(_matches: Matches) -> Result<i32> {
    Ok(0)
}

app!("Stylized App Name", app_setup, app_main);
```

The `app_setup` function is meant to use the `builder` to register supported
options and arguments whereas the `app_main` function is the entry point
that gets executed _after_ parsing the user-supplied command line according
to what was specified in `app_setup`.

Consult the documentation at <https://docs.rs/getoptsargs/> for the full API
reference.

For functional sample programs, see the files under the [`examples`](examples)
directory.  These will tell you:

*   how to write sync and async applications,
*   how to use the `builder` to define options and arguments, and
*   how to use the corresponding `matches` to access them after processing.
