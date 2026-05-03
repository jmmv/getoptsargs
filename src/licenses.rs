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
    AGPL3Only,
    AGPL3OrLater,
    Apache2,
    BSD3Clause,
    GPL2Only,
    GPL2OrLater,
    GPL3Only,
    GPL3OrLater,
    MIT,
}

impl License {
    /// Maps a Cargo-provided license to a `License`.  This is best-effort.
    pub(crate) fn from_cargo() -> Option<Self> {
        // TODO(jmmv): It'd be nice to support "OR" as well and not silently ignore unknown values.
        match env!("CARGO_PKG_LICENSE") {
            "AGPL-3.0-only" => Some(Self::AGPL3Only),
            "AGPL-3.0-or-later" => Some(Self::AGPL3OrLater),
            "Apache-2.0" => Some(Self::Apache2),
            "BSD-3-Clause" => Some(Self::BSD3Clause),
            "GPL-2.0-only" => Some(Self::GPL2Only),
            "GPL-2.0-or-later" => Some(Self::GPL2OrLater),
            "GPL-3.0-only" => Some(Self::GPL3Only),
            "GPL-3.0-or-later" => Some(Self::GPL3OrLater),
            "MIT" => Some(Self::MIT),
            _ => None,
        }
    }
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::AGPL3Only => {
                "GNU Affero General Public License v3.0 only <https://www.gnu.org/licenses/agpl-3.0.html>"
            }
            Self::AGPL3OrLater => {
                "GNU Affero General Public License v3.0 or later <https://www.gnu.org/licenses/agpl-3.0.html>"
            }
            Self::Apache2 => "Apache Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0>",
            Self::BSD3Clause => "BSD 3-Claus <https://opensource.org/license/bsd-3-clause>",
            Self::GPL2Only => {
                "GNU General Public License v2.0 only <https://www.gnu.org/licenses/gpl-2.0.html>"
            }
            Self::GPL2OrLater => {
                "GNU General Public License v2.0 or later <https://www.gnu.org/licenses/gpl-2.0.html>"
            }
            Self::GPL3Only => {
                "GNU General Public License v3.0 only <https://www.gnu.org/licenses/gpl-3.0.html>"
            }
            Self::GPL3OrLater => {
                "GNU General Public License v3.0 or later <https://www.gnu.org/licenses/gpl-3.0.html>"
            }
            Self::MIT => "MIT <https://opensource.org/license/mit>",
        };
        write!(f, "{}", text)
    }
}
