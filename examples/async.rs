// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! An example app to show the use of an async `main` with the `tokio` runtime.

use getoptsargs::prelude::*;

fn app_setup(builder: Builder) -> Builder {
    builder
}

async fn app_main(_matches: Matches) -> Result<i32> {
    Ok(0)
}

tokio_app!("async", app_setup, app_main);
