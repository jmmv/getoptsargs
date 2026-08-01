#! /bin/sh
# getoptsargs
# Copyright 2026 Julio Merino.
# All rights reserved.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option.  This file may not be copied, modified, or distributed
# except according to those terms.

set -eu

cargo clippy --all-features --all-targets -- -D warnings
cargo fmt -- --check
