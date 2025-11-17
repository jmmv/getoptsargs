// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Licenses that the user can choose from.

use std::fmt;

/// List of recognized licenses for informational messages.
#[allow(missing_docs)]
#[derive(Clone, Copy)]
pub enum License {
    Apache2,
    BSD3Clause,
    MIT,
}

impl License {
    /// Maps a Cargo-provided license to a `License`.  This is best-effort.
    pub(crate) fn from_cargo() -> Option<Self> {
        // TODO(jmmv): It'd be nice to support "OR" as well and not silently ignore unknown values.
        match env!("CARGO_PKG_LICENSE") {
            "Apache-2.0" => Some(Self::Apache2),
            "BSD-3-Clause" => Some(Self::BSD3Clause),
            "MIT" => Some(Self::MIT),
            _ => None,
        }
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Apache2 => "Apache Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>",
            Self::BSD3Clause => "BSD 3-Claus <https://opensource.org/license/bsd-3-clause>",
            Self::MIT => "MIT <https://opensource.org/license/mit>",
        };
        write!(f, "{}", text)
    }
}
