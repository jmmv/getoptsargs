// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Integration tests for the examples.

#![warn(anonymous_parameters, bad_style, missing_docs)]
#![warn(unused, unused_extern_crates, unused_import_braces, unused_qualifications)]
#![warn(unsafe_code)]

use getoptsargs::testutils::*;

#[test]
fn test_async_no_args_no_output() {
    check(bin_path("examples/async"), &[], 0, Behavior::Null, Behavior::Null);
}

#[test]
fn test_async_help() {
    check(
        bin_path("examples/async"),
        &["--help"],
        0,
        Behavior::Inline(
            "Usage: async [options]

Options:
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_async_version() {
    check(
        bin_path("examples/async"),
        &["--version"],
        0,
        Behavior::Inline(format!("async {}\n", env!("CARGO_PKG_VERSION"))),
        Behavior::Null,
    );
}

#[test]
fn test_async_too_many_args_args_no_output() {
    check(
        bin_path("examples/async"),
        &["foo"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Too many arguments
Type `async --help` for more information
"
            .to_owned(),
        ),
    );
}

#[test]
fn test_minimal_no_args_no_output() {
    check(bin_path("examples/minimal"), &[], 0, Behavior::Null, Behavior::Null);
}

#[test]
fn test_minimal_help() {
    check(
        bin_path("examples/minimal"),
        &["--help"],
        0,
        Behavior::Inline(
            "Usage: minimal [options]

Options:
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_minimal_version() {
    check(
        bin_path("examples/minimal"),
        &["--version"],
        0,
        Behavior::Inline(format!("minimal {}\n", env!("CARGO_PKG_VERSION"))),
        Behavior::Null,
    );
}

#[test]
fn test_minimal_too_many_args_args_no_output() {
    check(
        bin_path("examples/minimal"),
        &["foo"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Too many arguments
Type `minimal --help` for more information
"
            .to_owned(),
        ),
    );
}

#[test]
fn test_commands_dispatch() {
    check(
        bin_path("examples/commands"),
        &["--verbose", "echo", "--uppercase", "hello"],
        0,
        Behavior::Inline("verbose\nHELLO\n".to_owned()),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["status"],
        0,
        Behavior::Inline("ready\n".to_owned()),
        Behavior::Null,
    );
}

#[test]
fn test_commands_help_and_version() {
    check(
        bin_path("examples/commands"),
        &["help"],
        0,
        Behavior::Inline(
            "Usage: commands [options] command [arg...]

Options:
    -v, --verbose       print additional information

Commands:
    echo                print a message
    help                show command-line usage information
    status              print application status
    version             show version information

Report bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["--help"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Unrecognized option: 'help'
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
    check(
        bin_path("examples/commands"),
        &["--version"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Unrecognized option: 'version'
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
    check(
        bin_path("examples/commands"),
        &["echo", "--help"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Unrecognized option: 'help'
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
    check(
        bin_path("examples/commands"),
        &["help", "echo"],
        0,
        Behavior::Inline(
            "Usage: commands [options] echo [command options] message

Options:
    -u, --uppercase     print the message in uppercase

Arguments:
    message             message to print

The message is written to standard output.

Report bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["help", "status"],
        0,
        Behavior::Inline(
            "Usage: commands [options] status
\nReport bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["help", "help"],
        0,
        Behavior::Inline(
            "Usage: commands [options] help [command]

Arguments:
    [command]           command to show help for

Report bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["help", "version"],
        0,
        Behavior::Inline(
            "Usage: commands [options] version
\nReport bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/commands"),
        &["version"],
        0,
        Behavior::Inline(format!("commands {}\n", env!("CARGO_PKG_VERSION"))),
        Behavior::Null,
    );
}

#[test]
fn test_commands_errors() {
    check(
        bin_path("examples/commands"),
        &[],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: No command provided
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
    check(
        bin_path("examples/commands"),
        &["unknown"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Unknown command `unknown`
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
    check(
        bin_path("examples/commands"),
        &["help", "unknown"],
        2,
        Behavior::Null,
        Behavior::Inline(
            "Usage error: Unknown command `unknown`
Type `commands help` for more information
"
            .to_owned(),
        ),
    );
}

#[test]
fn test_async_commands_dispatch() {
    check(
        bin_path("examples/async_commands"),
        &["greet", "Ada"],
        0,
        Behavior::Inline("hello Ada\n".to_owned()),
        Behavior::Null,
    );
    check(
        bin_path("examples/async_commands"),
        &["status"],
        0,
        Behavior::Inline("ready\n".to_owned()),
        Behavior::Null,
    );
}

#[test]
fn test_async_commands_help() {
    check(
        bin_path("examples/async_commands"),
        &["help"],
        0,
        Behavior::Inline(
            "Usage: async_commands command [arg...]

Commands:
    greet               greet a user
    help                show command-line usage information
    status              print application status
    version             show version information

"
            .to_owned(),
        ),
        Behavior::Null,
    );
    check(
        bin_path("examples/async_commands"),
        &["help", "status"],
        0,
        Behavior::Inline(
            "Usage: async_commands status

"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_everything_no_args_no_output() {
    check(
        bin_path("examples/everything"),
        &["a", "b", "c", "d"],
        0,
        Behavior::Null,
        Behavior::Null,
    );
}

#[test]
fn test_everything_custom_flag() {
    check(
        bin_path("examples/everything"),
        &["-p", "abc", "de fg", "h", "f1"],
        42,
        Behavior::Inline(
            r"First arg: abc
Second arg: de fg
Third arg: h
File name: f1
"
            .to_owned(),
        ),
        Behavior::Null,
    );

    check(
        bin_path("examples/everything"),
        &["--print-args", "abc", "de fg", "h", "f1", "f2", "f3"],
        42,
        Behavior::Inline(
            r"First arg: abc
Second arg: de fg
Third arg: h
File name: f1
File name: f2
File name: f3
"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_everything_raise_error() {
    check(
        bin_path("examples/everything"),
        &["--raise-error", "p1", "p2", "p3"],
        2,
        Behavior::Null,
        Behavior::Inline(
            r"Usage error: Found raise-error flag
Type `everything --help` or `man 8 the-everything` for more information
"
            .to_owned(),
        ),
    );
}

#[test]
fn test_everything_raise_chain() {
    check(
        bin_path("examples/everything"),
        &["--raise-chain", "p1", "p2", "p3"],
        1,
        Behavior::Null,
        Behavior::Inline(
            "everything: Outermost problem: Intermediate problem: Innermost problem\n".to_owned(),
        ),
    );
}

#[test]
fn test_everything_help() {
    check(
        bin_path("examples/everything"),
        &["--help"],
        0,
        Behavior::Inline(
            "Usage: everything [options] first second third_has_a_very_long_name [name1 .. nameN]

Options:
    -p, --print-args    print free arguments
        --raise-chain   raises an error with causes
        --raise-error   raises an explicit usage error
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

Arguments:
    first               this is the first required argument and contains a
                        very long description
    second              short description
    third_has_a_very_long_name
                        and a short description
    [name1 .. nameN]    file names

This is an extra help message.

Report bugs to: https://example.com/everything/issues/
Everything home page: https://everything.example.com/
"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_imperative_no_args_no_output() {
    check(bin_path("examples/imperative"), &["a"], 0, Behavior::Null, Behavior::Null);
}

#[test]
fn test_imperative_custom_flag() {
    check(
        bin_path("examples/imperative"),
        &["--print-args", "abc", "de fg"],
        0,
        Behavior::Inline(
            r"Free argument: abc
Free argument: de fg
"
            .to_owned(),
        ),
        Behavior::Null,
    );
}

#[test]
fn test_imperative_help() {
    check(
        bin_path("examples/imperative"),
        &["--help"],
        0,
        Behavior::Inline(
            "Usage: imperative [options] [trail1 .. trailN]

Options:
        --print-args    print free arguments
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

Arguments:
    [trail1 .. trailN]  free arguments

"
            .to_owned(),
        ),
        Behavior::Null,
    );
}
