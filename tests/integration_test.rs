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

use assert_cmd::Command;
#[test]
fn test_async_no_args_no_output() {
    Command::cargo_bin("examples/async")
        .expect("Failed to find example executable")
        .assert()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn test_async_help() {
    Command::cargo_bin("examples/async")
        .expect("Failed to find example executable")
        .args(["--help"])
        .assert()
        .code(0)
        .stdout(
            "Usage: async [options]

Options:
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

"
            .to_owned(),
        )
        .stderr("");
}

#[test]
fn test_async_version() {
    Command::cargo_bin("examples/async")
        .expect("Failed to find example executable")
        .args(["--version"])
        .assert()
        .code(0)
        .stdout(format!("async {}\n", env!("CARGO_PKG_VERSION")))
        .stderr("");
}

#[test]
fn test_async_too_many_args_args_no_output() {
    Command::cargo_bin("examples/async")
        .expect("Failed to find example executable")
        .args(["foo"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Too many arguments
Type `async --help` for more information
"
            .to_owned(),
        );
}

#[test]
fn test_minimal_no_args_no_output() {
    Command::cargo_bin("examples/minimal")
        .expect("Failed to find example executable")
        .assert()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn test_minimal_help() {
    Command::cargo_bin("examples/minimal")
        .expect("Failed to find example executable")
        .args(["--help"])
        .assert()
        .code(0)
        .stdout(
            "Usage: minimal [options]

Options:
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

"
            .to_owned(),
        )
        .stderr("");
}

#[test]
fn test_minimal_version() {
    Command::cargo_bin("examples/minimal")
        .expect("Failed to find example executable")
        .args(["--version"])
        .assert()
        .code(0)
        .stdout(format!("minimal {}\n", env!("CARGO_PKG_VERSION")))
        .stderr("");
}

#[test]
fn test_minimal_too_many_args_args_no_output() {
    Command::cargo_bin("examples/minimal")
        .expect("Failed to find example executable")
        .args(["foo"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Too many arguments
Type `minimal --help` for more information
"
            .to_owned(),
        );
}

#[test]
fn test_commands_dispatch() {
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["--verbose", "echo", "--uppercase", "hello"])
        .assert()
        .code(0)
        .stdout("verbose\nHELLO\n".to_owned())
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["status"])
        .assert()
        .code(0)
        .stdout("ready\n".to_owned())
        .stderr("");
}

#[test]
fn test_commands_help_and_version() {
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help"])
        .assert()
        .code(0)
        .stdout(
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
        )
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["--help"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Unrecognized option: 'help'
Type `commands help` for more information
"
            .to_owned(),
        );
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["--version"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Unrecognized option: 'version'
Type `commands help` for more information
"
            .to_owned(),
        );
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["echo", "--help"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Unrecognized option: 'help'
Type `commands help` for more information
"
            .to_owned(),
        );
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help", "echo"])
        .assert()
        .code(0)
        .stdout(
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
        )
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help", "status"])
        .assert()
        .code(0)
        .stdout(
            "Usage: commands [options] status
\nReport bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        )
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help", "help"])
        .assert()
        .code(0)
        .stdout(
            "Usage: commands [options] help [command]

Arguments:
    [command]           command to show help for

Report bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        )
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help", "version"])
        .assert()
        .code(0)
        .stdout(
            "Usage: commands [options] version
\nReport bugs to: https://example.com/commands/issues/
commands home page: https://commands.example.com/
"
            .to_owned(),
        )
        .stderr("");
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["version"])
        .assert()
        .code(0)
        .stdout(format!("commands {}\n", env!("CARGO_PKG_VERSION")))
        .stderr("");
}

#[test]
fn test_commands_errors() {
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: No command provided
Type `commands help` for more information
"
            .to_owned(),
        );
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["unknown"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Unknown command `unknown`
Type `commands help` for more information
"
            .to_owned(),
        );
    Command::cargo_bin("examples/commands")
        .expect("Failed to find example executable")
        .args(["help", "unknown"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            "Usage error: Unknown command `unknown`
Type `commands help` for more information
"
            .to_owned(),
        );
}

#[test]
fn test_async_commands_dispatch() {
    Command::cargo_bin("examples/async_commands")
        .expect("Failed to find example executable")
        .args(["greet", "Ada"])
        .assert()
        .code(0)
        .stdout("hello Ada\n".to_owned())
        .stderr("");
    Command::cargo_bin("examples/async_commands")
        .expect("Failed to find example executable")
        .args(["status"])
        .assert()
        .code(0)
        .stdout("ready\n".to_owned())
        .stderr("");
}

#[test]
fn test_async_commands_help() {
    Command::cargo_bin("examples/async_commands")
        .expect("Failed to find example executable")
        .args(["help"])
        .assert()
        .code(0)
        .stdout(
            "Usage: async_commands command [arg...]

Commands:
    greet               greet a user
    help                show command-line usage information
    status              print application status
    version             show version information

"
            .to_owned(),
        )
        .stderr("");
    Command::cargo_bin("examples/async_commands")
        .expect("Failed to find example executable")
        .args(["help", "status"])
        .assert()
        .code(0)
        .stdout(
            "Usage: async_commands status

"
            .to_owned(),
        )
        .stderr("");
}

#[test]
fn test_everything_no_args_no_output() {
    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["a", "b", "c", "d"])
        .assert()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn test_everything_custom_flag() {
    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["-p", "abc", "de fg", "h", "f1"])
        .assert()
        .code(42)
        .stdout(
            r"First arg: abc
Second arg: de fg
Third arg: h
File name: f1
"
            .to_owned(),
        )
        .stderr("");

    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["--print-args", "abc", "de fg", "h", "f1", "f2", "f3"])
        .assert()
        .code(42)
        .stdout(
            r"First arg: abc
Second arg: de fg
Third arg: h
File name: f1
File name: f2
File name: f3
"
            .to_owned(),
        )
        .stderr("");
}

#[test]
fn test_everything_raise_error() {
    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["--raise-error", "p1", "p2", "p3"])
        .assert()
        .code(2)
        .stdout("")
        .stderr(
            r"Usage error: Found raise-error flag
Type `everything --help` or `man 8 the-everything` for more information
"
            .to_owned(),
        );
}

#[test]
fn test_everything_raise_chain() {
    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["--raise-chain", "p1", "p2", "p3"])
        .assert()
        .code(1)
        .stdout("")
        .stderr(
            "everything: Outermost problem: Intermediate problem: Innermost problem\n".to_owned(),
        );
}

#[test]
fn test_everything_help() {
    Command::cargo_bin("examples/everything")
        .expect("Failed to find example executable")
        .args(["--help"])
        .assert()
        .code(0)
        .stdout(
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
        )
        .stderr("");
}

#[test]
fn test_imperative_no_args_no_output() {
    Command::cargo_bin("examples/imperative")
        .expect("Failed to find example executable")
        .args(["a"])
        .assert()
        .code(0)
        .stdout("")
        .stderr("");
}

#[test]
fn test_imperative_custom_flag() {
    Command::cargo_bin("examples/imperative")
        .expect("Failed to find example executable")
        .args(["--print-args", "abc", "de fg"])
        .assert()
        .code(0)
        .stdout(
            r"Free argument: abc
Free argument: de fg
"
            .to_owned(),
        )
        .stderr("");
}

#[test]
fn test_imperative_help() {
    Command::cargo_bin("examples/imperative")
        .expect("Failed to find example executable")
        .args(["--help"])
        .assert()
        .code(0)
        .stdout(
            "Usage: imperative [options] [trail1 .. trailN]

Options:
        --print-args    print free arguments
    -h, --help          show command-line usage information and exit
        --version       show version information and exit

Arguments:
    [trail1 .. trailN]  free arguments

"
            .to_owned(),
        )
        .stderr("");
}
