// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Error types for this library.

use std::error::Error;
use std::fmt;

/// Errors caused by the user's invocation of the program.
#[derive(Debug, PartialEq)]
pub struct UsageError {
    /// The error message to show to the user.
    pub message: String,
}

impl fmt::Display for UsageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for UsageError {}

/// Constructs an `UsageError` from a format string.
#[macro_export]
macro_rules! __bad_usage__ {
    ($fmt:expr) => {
        $crate::UsageError { message: ($fmt).into() }
    };

    ($fmt:expr, $($args:tt)*) => {
        $crate::UsageError { message: format!($fmt, $($args)*) }
    };
}

pub use __bad_usage__ as bad_usage;
