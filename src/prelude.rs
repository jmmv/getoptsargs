// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Public types required to implement any app.  Import wholesale.

pub use crate::errors::UsageError;
pub use crate::licenses::License;
#[cfg(feature = "env_logger")]
pub use crate::run::init_env_logger;
pub use crate::{Builder, Matches, app, tokio_app};
pub use anyhow::{Result, anyhow, bail};
