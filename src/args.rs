// getoptsargs
// Copyright 2025 Julio Merino.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

//! Positional argument parsing.

use crate::errors::{UsageError, bad_usage};
use std::collections::HashMap;

/// Start of the second column in usage messages.  Matches `getopts`.
const COL2_START: usize = 24;
/// Max lenth of the second column in usage messages.  Matches `getopts`.
const COL2_WIDTH: usize = 54;

/// Given an `unwrapped` text, reformats it to fit within `max_width`.  Every generated line
/// _except_ the first one is prefixed with `pad_width` spaces.
fn wrap(unwrapped: &str, pad_width: usize, max_width: usize) -> String {
    let mut text = String::new();
    let mut len = 0;
    for word in unwrapped.split(' ') {
        if len == 0 {
            text.push_str(word);
            len += word.len();
            continue;
        }

        if len + word.len() + 1 > max_width {
            text.push('\n');
            text.push_str(&" ".repeat(pad_width));
            len = 0;
        } else {
            text.push(' ');
            len += 1;
        }
        text.push_str(word);
        len += word.len();
    }
    text
}

/// Given two strings, formats them as two colums.  The second column starts at `start2` and will
/// be wrapped to `width2` characters.  The resulting string may have newlines in it.
fn format_two_columns(col1: String, col2: &str, start2: usize, width2: usize) -> String {
    let mut text = col1;
    if text.len() < start2 {
        while text.len() < start2 {
            text.push(' ');
        }
    } else {
        text.push('\n');
        text.push_str(&" ".repeat(start2));
    }
    text.push_str(&wrap(col2, start2, width2));
    text
}

/// Representation of parsed free arguments.
#[derive(Debug)]
pub struct Matches {
    /// Mapping of named positional arguments to their values.
    positional: HashMap<&'static str, String>,

    /// Trailing optional or repeated arguments.
    trailing: Vec<String>,
}

/// Returns the brief specification for the trailing argument `name`.
fn trailing_brief(name: &str, min: usize, max: usize) -> String {
    match (min, max) {
        (0, 1) => format!("[{}]", name),
        (0, usize::MAX) => format!("[{}1 .. {}N]", name, name),
        (1, usize::MAX) => format!("{}1 [.. {}N]", name, name),
        (_, _) => format!("{}{} .. {}{}", name, min, name, max),
    }
}

/// Representation of expected free arguments.
///
/// The interface of this struct is supposed to mimic `getopts::Options`.
#[derive(Default)]
pub struct Arguments {
    positional_spec: Vec<(&'static str, &'static str)>,
    trailing_spec: Option<(&'static str, usize, usize, &'static str)>,
}

impl Arguments {
    /// Registers the next positional argumet with `name` and `description`.
    pub fn positional(&mut self, name: &'static str, description: &'static str) {
        assert!(
            self.trailing_spec.is_none(),
            "Cannot register positional arguments after setting the trailing spec"
        );
        self.positional_spec.push((name, description));
    }

    /// Registers the remaining trailing arguments with a base name of `name` and a `description`.
    ///
    /// `min` and `max` specify the number of occurrences required for this argument.  The common
    /// cases are either 0 and 1 to represent an optional argument; 0 and `usize::MAX` to represent
    /// an optional list of arguments; or 1 and `usize::MAX` to represent a list of arguments with
    /// at least one entry.
    pub fn trailing(
        &mut self,
        name: &'static str,
        min: usize,
        max: usize,
        description: &'static str,
    ) {
        assert!(self.trailing_spec.is_none(), "Cannot register trailing arguments more than once");
        self.trailing_spec = Some((name, min, max, description));
    }

    /// Generates a brief description of the arguments specification to be used in usage summaries.
    pub(crate) fn brief(&self) -> String {
        let mut spec = self.positional_spec.iter().map(|s| s.0).collect::<Vec<_>>().join(" ");
        if let Some((name, min, max, _description)) = self.trailing_spec {
            if !spec.is_empty() {
                spec.push(' ');
            }
            spec += &trailing_brief(name, min, max);
        }
        spec
    }

    /// Generates a multi-line usage message with the details of all arguments.
    ///
    /// The output matches the format of `getopts::Matches::usage` and should be used to extend its
    /// return value.
    pub(crate) fn usage(&self) -> String {
        if self.positional_spec.is_empty() && self.trailing_spec.is_none() {
            return String::new();
        }

        let mut text = String::from("Arguments:\n");
        for (name, description) in &self.positional_spec {
            text +=
                &format_two_columns(format!("    {}", name), description, COL2_START, COL2_WIDTH);
            text.push('\n');
        }
        if let Some((name, min, max, description)) = self.trailing_spec {
            let brief = trailing_brief(name, min, max);
            text +=
                &format_two_columns(format!("    {}", brief), description, COL2_START, COL2_WIDTH);
            text.push('\n');
        }
        text
    }

    /// Parses a collection of free-form arguments and returns a `Matches` object if they are valid
    /// according to the arguments specification.
    pub(crate) fn parse(&self, free: Vec<String>) -> Result<Matches, UsageError> {
        let mut iter = free.into_iter();

        let mut positional = HashMap::with_capacity(self.positional_spec.len());
        for (name, _description) in &self.positional_spec {
            let value = match iter.next() {
                Some(value) => value,
                None => return Err(bad_usage!("Required argument `{}` not provided", name)),
            };
            let previous = positional.insert(*name, value);
            assert!(previous.is_none());
        }

        let mut trailing = vec![];
        if let Some((name, min, max, _description)) = self.trailing_spec {
            trailing = iter.collect::<Vec<String>>();
            if trailing.len() < min {
                if min == 1 {
                    return Err(bad_usage!(
                        "Trailing argument `{}` requires at least 1 value",
                        name
                    ));
                } else {
                    return Err(bad_usage!(
                        "Trailing argument `{}` requires at least {} values",
                        name,
                        min
                    ));
                }
            }
            if trailing.len() > max {
                return Err(bad_usage!("Too many arguments"));
            }
        } else {
            if iter.next().is_some() {
                return Err(bad_usage!("Too many arguments"));
            }
        }

        Ok(Matches { positional, trailing })
    }
}

impl super::Builder {
    /// Trivial wrapper over `Arguments::positional`.
    pub fn posarg(mut self, name: &'static str, description: &'static str) -> Self {
        self.args.positional(name, description);
        self
    }

    /// Trivial wrapper over `Arguments::trailing`.
    pub fn trailarg(
        mut self,
        name: &'static str,
        min: usize,
        max: usize,
        description: &'static str,
    ) -> Self {
        self.args.trailing(name, min, max, description);
        self
    }
}

impl super::Matches {
    /// Returns the positional name identified by `name`.
    ///
    /// # Panics
    ///
    /// This function will panic if the argument name has not been correctly processed.
    #[inline(always)]
    pub fn arg_pos(&self, name: &str) -> &str {
        self.args.positional.get(name).unwrap()
    }

    /// Returns the trailing arguments after all registered positional arguments.
    #[inline(always)]
    pub fn arg_trail(&self) -> &[String] {
        self.args.trailing.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_fits_ok() {
        assert_eq!("foo bar", wrap("foo bar", 4, 7));
        assert_eq!("foo bar", wrap("foo bar", 4, 10));
    }

    #[test]
    fn test_wrap_does_not_fit_no_padding() {
        assert_eq!("foo\nbar\nvery-long-word\na b", wrap("foo bar very-long-word a b", 0, 5));
    }

    #[test]
    fn test_wrap_does_not_fit_with_padding() {
        assert_eq!(
            "foo\n    bar\n    very-long-word\n    a b",
            wrap("foo bar very-long-word a b", 4, 5)
        );
    }

    #[test]
    fn test_format_two_columns_one_line() {
        assert_eq!("    foo   bar", format_two_columns("    foo".into(), "bar", 10, 5));
        assert_eq!("    fooxy bar", format_two_columns("    fooxy".into(), "bar", 10, 5));
    }

    #[test]
    fn test_format_two_columns_first_column_too_long() {
        assert_eq!(
            "    fooxyz\n          bar",
            format_two_columns("    fooxyz".into(), "bar", 10, 5)
        );
    }

    #[test]
    fn test_format_two_columns_second_column_too_long() {
        assert_eq!(
            "    foo   bar\n          baz",
            format_two_columns("    foo".into(), "bar baz", 10, 5)
        );
    }

    #[test]
    fn test_arguments_brief_none() {
        let args = Arguments::default();
        assert_eq!("", args.brief());
    }

    #[test]
    fn test_arguments_brief_only_positional() {
        let mut args = Arguments::default();
        args.positional("one", "irrelevant");
        assert_eq!("one", args.brief());

        args.positional("two", "irrelevant");
        assert_eq!("one two", args.brief());
    }

    #[test]
    fn test_arguments_brief_only_trailing() {
        let mut args = Arguments::default();
        args.trailing("name", 0, usize::MAX, "irrelevant");
        assert_eq!("[name1 .. nameN]", args.brief());

        let mut args = Arguments::default();
        args.trailing("name", 1, usize::MAX, "irrelevant");
        assert_eq!("name1 [.. nameN]", args.brief());
    }

    #[test]
    fn test_arguments_brief_positional_and_trailing() {
        let mut args = Arguments::default();
        args.positional("one", "irrelevant");
        args.trailing("name", 0, usize::MAX, "irrelevant");
        assert_eq!("one [name1 .. nameN]", args.brief());
    }

    #[test]
    fn test_arguments_usage_none() {
        let args = Arguments::default();
        assert_eq!("", args.usage());
    }

    #[test]
    fn test_arguments_usage_only_positional() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        assert_eq!(
            r"Arguments:
    one                 flag one
",
            args.usage()
        );

        args.positional("two", "flag two");
        assert_eq!(
            r"Arguments:
    one                 flag one
    two                 flag two
",
            args.usage()
        );
    }

    #[test]
    fn test_arguments_usage_only_trailing() {
        let mut args = Arguments::default();
        args.trailing("name", 0, usize::MAX, "list of names");
        assert_eq!(
            r"Arguments:
    [name1 .. nameN]    list of names
",
            args.usage()
        );

        let mut args = Arguments::default();
        args.trailing("name", 1, usize::MAX, "list of names");
        assert_eq!(
            r"Arguments:
    name1 [.. nameN]    list of names
",
            args.usage()
        );
    }

    #[test]
    fn test_arguments_usage_positional_and_trailing() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        args.trailing("name", 0, usize::MAX, "list of names");
        assert_eq!(
            r"Arguments:
    one                 flag one
    [name1 .. nameN]    list of names
",
            args.usage()
        )
    }

    #[test]
    fn test_arguments_parse_none() {
        let args = Arguments::default();
        let matches = args.parse(vec![]).unwrap();
        assert!(matches.positional.is_empty());
        assert!(matches.trailing.is_empty());
    }

    #[test]
    fn test_arguments_parse_only_positional_ok() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        args.positional("two", "flag two");
        let matches = args.parse(vec!["foo".to_owned(), "bar".to_owned()]).unwrap();
        assert_eq!("foo", matches.positional["one"]);
        assert_eq!("bar", matches.positional["two"]);
        assert!(matches.trailing.is_empty());
    }

    #[test]
    fn test_arguments_parse_only_positional_not_enough() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        args.positional("two", "flag two");
        let err = args.parse(vec![]).unwrap_err();
        assert_eq!(bad_usage!("Required argument `one` not provided"), err);
        let err = args.parse(vec!["foo".to_owned()]).unwrap_err();
        assert_eq!(bad_usage!("Required argument `two` not provided"), err);
    }

    #[test]
    fn test_arguments_parse_only_positional_too_many() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        args.positional("two", "flag two");
        let err =
            args.parse(vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()]).unwrap_err();
        assert_eq!(bad_usage!("Too many arguments"), err);
    }

    #[test]
    fn test_arguments_parse_only_trailing_not_required() {
        let mut args = Arguments::default();
        args.trailing("name", 0, usize::MAX, "list of names");

        let matches = args.parse(vec![]).unwrap();
        assert!(matches.positional.is_empty());
        assert!(matches.trailing.is_empty());

        let matches = args.parse(vec!["a".to_owned()]).unwrap();
        assert!(matches.positional.is_empty());
        assert_eq!(vec!["a"], matches.trailing);

        let matches = args.parse(vec!["a".to_owned(), "b".to_owned()]).unwrap();
        assert!(matches.positional.is_empty());
        assert_eq!(vec!["a", "b"], matches.trailing);
    }

    #[test]
    fn test_arguments_parse_only_trailing_required() {
        let mut args = Arguments::default();
        args.trailing("name", 1, usize::MAX, "list of names");

        let matches = args.parse(vec!["a".to_owned()]).unwrap();
        assert!(matches.positional.is_empty());
        assert_eq!(vec!["a"], matches.trailing);

        let matches = args.parse(vec!["a".to_owned(), "b".to_owned()]).unwrap();
        assert!(matches.positional.is_empty());
        assert_eq!(vec!["a", "b"], matches.trailing);
    }

    #[test]
    fn test_arguments_parse_only_trailing_required_not_enough() {
        let mut args = Arguments::default();
        args.trailing("name", 1, usize::MAX, "list of names");
        let err = args.parse(vec![]).unwrap_err();
        assert_eq!(bad_usage!("Trailing argument `name` requires at least 1 value"), err);
    }

    #[test]
    fn test_arguments_parse_positional_and_trailing() {
        let mut args = Arguments::default();
        args.positional("one", "flag one");
        args.trailing("name", 0, usize::MAX, "list of names");
        let matches = args.parse(vec!["a".to_owned(), "b".to_owned()]).unwrap();
        assert_eq!("a", matches.positional["one"]);
        assert_eq!(vec!["b"], matches.trailing);
    }
}
