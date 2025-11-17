// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! An example app that uses all optional hooks to define custom options, arguments,
//! and help messages.

use getoptsargs::prelude::*;

fn app_extra_help() {
    println!("This is an extra help message.");
}

fn app_setup(builder: Builder) -> Builder {
    builder
        .copyright("2025 Julio Merino")
        .bugs("https://example.com/everything/issues/")
        .homepage("https://everything.example.com/")
        .manpage("the-everything", "8")
        .extra_help(app_extra_help)
        // Configure option processing.
        .optflag("p", "print-args", "print free arguments")
        // Configure argument processing.
        .posarg("first", "this is the first required argument and contains a very long description")
        .posarg("second", "short description")
        .posarg("third_has_a_very_long_name", "and a short description")
        .trailarg("name", 0, usize::MAX, "file names")
}

fn app_main(matches: Matches) -> Result<i32> {
    if matches.opt_present("print-args") {
        println!("First arg: {}", matches.arg_pos("first"));
        println!("Second arg: {}", matches.arg_pos("second"));
        println!("Third arg: {}", matches.arg_pos("third_has_a_very_long_name"));

        for name in matches.arg_trail() {
            println!("File name: {}", name);
        }

        Ok(42)
    } else {
        Ok(0)
    }
}

app!("Everything", app_setup, app_main);
