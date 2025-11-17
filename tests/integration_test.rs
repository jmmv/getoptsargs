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
fn test_everything_help() {
    check(
        bin_path("examples/everything"),
        &["--help"],
        0,
        Behavior::Inline(
            "Usage: everything [options] first second third_has_a_very_long_name [name1 .. nameN]

Options:
    -h, --help          show command-line usage information and exit
        --version       show version information and exit
    -p, --print-args    print free arguments

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
