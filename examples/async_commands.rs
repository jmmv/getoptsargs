// getoptsargs
// Copyright 2026 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// at your option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! An example app with asynchronous subcommands.

use getoptsargs::prelude::*;

fn greet_setup(builder: CommandBuilder) -> CommandBuilder {
    builder.posarg("name", "name to greet")
}

async fn greet_main(_app_matches: Matches, command_matches: Matches) -> Result<i32> {
    println!("hello {}", command_matches.arg_pos("name"));
    Ok(0)
}

fn status_setup(builder: CommandBuilder) -> CommandBuilder {
    builder
}

async fn status_main(_app_matches: Matches, _command_matches: Matches) -> Result<i32> {
    println!("ready");
    Ok(0)
}

fn app_setup(builder: Builder) -> Builder {
    builder.cmd_async("greet", "greet a user", greet_setup, greet_main).cmd_async(
        "status",
        "print application status",
        status_setup,
        status_main,
    )
}

tokio_app!("async-commands", app_setup, tokio_command_dispatcher);
