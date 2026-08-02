// getoptsargs
// Copyright 2026 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// at your option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! An example app with synchronous subcommands.

use getoptsargs::prelude::*;

fn echo_extra_help(output: &mut dyn std::io::Write) -> std::io::Result<()> {
    writeln!(output, "The message is written to standard output.")
}

fn echo_setup(builder: CommandBuilder) -> CommandBuilder {
    builder
        .extra_help(echo_extra_help)
        .optflag("u", "uppercase", "print the message in uppercase")
        .posarg("message", "message to print")
}

fn echo_main(app_matches: Matches, command_matches: Matches) -> Result<i32> {
    let message = command_matches.arg_pos("message");
    if app_matches.opt_present("verbose") {
        println!("verbose");
    }
    if command_matches.opt_present("uppercase") {
        println!("{}", message.to_uppercase());
    } else {
        println!("{}", message);
    }
    Ok(0)
}

fn status_setup(builder: CommandBuilder) -> CommandBuilder {
    builder
}

fn status_main(_app_matches: Matches, _command_matches: Matches) -> Result<i32> {
    println!("ready");
    Ok(0)
}

fn app_setup(builder: Builder) -> Builder {
    builder
        .bugs("https://example.com/commands/issues/")
        .homepage("https://commands.example.com/")
        .optflag("v", "verbose", "print additional information")
        .cmd("status", "print application status", status_setup, status_main)
        .cmd("echo", "print a message", echo_setup, echo_main)
}

app!("commands", app_setup, command_dispatcher);
