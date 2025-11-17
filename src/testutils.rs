// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Helper functions to implement integration tests that use golden input and output files.

use std::env;
use std::path::{Path, PathBuf};
use std::process;

/// Computes the path to the directory where this test's binary lives.
pub fn self_dir() -> PathBuf {
    let self_exe = env::current_exe().expect("Cannot get self's executable path");
    let dir = self_exe.parent().expect("Cannot get self's directory");
    assert!(dir.ends_with("target/debug/deps") || dir.ends_with("target/release/deps"));
    dir.to_owned()
}

/// Computes the path to the built binary `name`.
pub fn bin_path<P: AsRef<Path>>(name: P) -> PathBuf {
    let test_dir = self_dir();
    let debug_or_release_dir = test_dir.parent().expect("Failed to get parent directory");
    debug_or_release_dir.join(name).with_extension(env::consts::EXE_EXTENSION)
}

/// Describes the behavior for one of the output streams (stdout, stderr) connected to a
/// program.
pub enum Behavior {
    /// Ensure the stream is silent.
    Null,

    /// Expect the contents of the stream to match this string.
    Inline(String),
}

/// Runs `bin` with arguments `args` and checks its behavior against expectations.
///
/// `exp_code` is the expected error code from the program.  `stdout_behavior` and `stderr_behavior`
/// indicate what to expect from the program's textual output.
pub fn check<P: AsRef<Path>>(
    bin: P,
    args: &[&str],
    exp_code: i32,
    stdout_behavior: Behavior,
    stderr_behavior: Behavior,
) {
    let exp_stdout = match stdout_behavior {
        Behavior::Null => "".to_owned(),
        Behavior::Inline(s) => s,
    };

    let exp_stderr = match stderr_behavior {
        Behavior::Null => "".to_owned(),
        Behavior::Inline(s) => s,
    };

    let result = process::Command::new(bin.as_ref())
        .args(args)
        .output()
        .expect("Failed to execute subprocess");
    let code = result.status.code().expect("Subprocess didn't exit cleanly");
    let stdout = String::from_utf8(result.stdout).expect("Stdout not is not valid UTF-8");
    let stderr = String::from_utf8(result.stderr).expect("Stderr not is not valid UTF-8");

    if exp_code != code || exp_stdout != stdout || exp_stderr != stderr {
        eprintln!("Exit code: {}", code);
        eprintln!("stdout:\n{}", stdout);
        eprintln!("stderr:\n{}", stderr);
        assert_eq!(exp_code, code);
        assert_eq!(exp_stdout, stdout);
        assert_eq!(exp_stderr, stderr);
    }
}
