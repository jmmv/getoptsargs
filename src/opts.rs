// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Accessors for getopts.

use crate::{Builder, Matches};
use getopts::{HasArg, Occur};
use std::iter::IntoIterator;
use std::str::FromStr;

impl Builder {
    /// Trivial wrapper over `getopts::Options::opt`.
    #[inline(always)]
    pub fn opt(
        mut self,
        short_name: &str,
        long_name: &str,
        desc: &str,
        hint: &str,
        hasarg: HasArg,
        occur: Occur,
    ) -> Self {
        self.opts.opt(short_name, long_name, desc, hint, hasarg, occur);
        self
    }

    /// Trivial wrapper over `getopts::Options::optflag`.
    #[inline(always)]
    pub fn optflag(mut self, short_name: &str, long_name: &str, desc: &str) -> Self {
        self.opts.optflag(short_name, long_name, desc);
        self
    }

    /// Trivial wrapper over `getopts::Options::optflagmulti`.
    #[inline(always)]
    pub fn optflagmulti(mut self, short_name: &str, long_name: &str, desc: &str) -> Self {
        self.opts.optflagmulti(short_name, long_name, desc);
        self
    }

    /// Trivial wrapper over `getopts::Options::optflagopt`.
    #[inline(always)]
    pub fn optflagopt(mut self, short_name: &str, long_name: &str, desc: &str, hint: &str) -> Self {
        self.opts.optflagopt(short_name, long_name, desc, hint);
        self
    }

    /// Trivial wrapper over `getopts::Options::optmulti`.
    #[inline(always)]
    pub fn optmulti(mut self, short_name: &str, long_name: &str, desc: &str, hint: &str) -> Self {
        self.opts.optmulti(short_name, long_name, desc, hint);
        self
    }

    /// Trivial wrapper over `getopts::Options::optopt`.
    #[inline(always)]
    pub fn optopt(mut self, short_name: &str, long_name: &str, desc: &str, hint: &str) -> Self {
        self.opts.optopt(short_name, long_name, desc, hint);
        self
    }

    /// Trivial wrapper over `getopts::Options::reqopt`.
    #[inline(always)]
    pub fn reqopt(mut self, short_name: &str, long_name: &str, desc: &str, hint: &str) -> Self {
        self.opts.reqopt(short_name, long_name, desc, hint);
        self
    }
}

impl Matches {
    /// Trivial wrapper over `getopts::Matches::opt_defined`.
    #[inline(always)]
    pub fn opt_defined(&self, name: &str) -> bool {
        self.opts.opt_defined(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_present`.
    #[inline(always)]
    pub fn opt_present(&self, name: &str) -> bool {
        self.opts.opt_present(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_count`.
    #[inline(always)]
    pub fn opt_count(&self, name: &str) -> usize {
        self.opts.opt_count(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_positions`.
    #[inline(always)]
    pub fn opt_positions(&self, name: &str) -> Vec<usize> {
        self.opts.opt_positions(name)
    }

    /// Trivial wrapper over `getopts::Matches::opts_present`.
    #[inline(always)]
    pub fn opts_present(&self, names: &[String]) -> bool {
        self.opts.opts_present(names)
    }

    /// Trivial wrapper over `getopts::Matches::opts_present_any`.
    #[inline(always)]
    pub fn opts_present_any<C: IntoIterator>(&self, names: C) -> bool
    where
        C::Item: AsRef<str>,
    {
        self.opts.opts_present_any(names)
    }

    /// Trivial wrapper over `getopts::Matches::opts_str`.
    #[inline(always)]
    pub fn opts_str(&self, names: &[String]) -> Option<String> {
        self.opts.opts_str(names)
    }

    /// Trivial wrapper over `getopts::Matches::opts_str_first`.
    #[inline(always)]
    pub fn opts_str_first<C: IntoIterator>(&self, names: C) -> Option<String>
    where
        C::Item: AsRef<str>,
    {
        self.opts.opts_str_first(names)
    }

    /// Trivial wrapper over `getopts::Matches::opt_strs`.
    #[inline(always)]
    pub fn opt_strs(&self, name: &str) -> Vec<String> {
        self.opts.opt_strs(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_strs_pos`.
    #[inline(always)]
    pub fn opt_strs_pos(&self, name: &str) -> Vec<(usize, String)> {
        self.opts.opt_strs_pos(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_str`.
    #[inline(always)]
    pub fn opt_str(&self, name: &str) -> Option<String> {
        self.opts.opt_str(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_default`.
    #[inline(always)]
    pub fn opt_default(&self, name: &str, def: &str) -> Option<String> {
        self.opts.opt_default(name, def)
    }

    /// Trivial wrapper over `getopts::Matches::opt_get`.
    #[inline(always)]
    pub fn opt_get<T>(&self, name: &str) -> Result<Option<T>, T::Err>
    where
        T: FromStr,
    {
        self.opts.opt_get(name)
    }

    /// Trivial wrapper over `getopts::Matches::opt_get_default`.
    #[inline(always)]
    pub fn opt_get_default<T>(&self, name: &str, def: T) -> Result<T, T::Err>
    where
        T: FromStr,
    {
        self.opts.opt_get_default(name, def)
    }
}
