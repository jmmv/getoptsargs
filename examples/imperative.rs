// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! An example app that does _not_ use the `app!` macro to show how
//! to use this crate in a fully imperative manner.

use getoptsargs::prelude::*;

fn main() {
    let result = Builder::new("imperative", env!("CARGO_PKG_VERSION"), std::env::args())
        .optflag("", "print-args", "print free arguments")
        .trailarg("trail", 0, usize::MAX, "free arguments")
        .start();

    let matches = match result {
        Ok(Some(matches)) => matches,

        Ok(None) => {
            // The user provided a standard terminal flag such as `--help` or
            // `--version` which was already handled by `start`.  Exit immediately
            // without error (this is important).
            std::process::exit(0);
        }

        Err(e) => {
            eprintln!("Argument parsing error: {}", e);
            std::process::exit(1);
        }
    };

    // matches now contains the application-specific flags and can be handled as desired.

    if matches.opt_present("print-args") {
        for arg in matches.arg_trail() {
            println!("Free argument: {}", arg);
        }
    }
}
