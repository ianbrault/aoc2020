/*
** src/utils.rs
*/

use std::str::FromStr;

// a macro for a split-and-match pattern which is used frequently
// the Pattern struct is nightly-only, so we cannot use a Rust function
macro_rules! split {
    ($string:ident, $splitter:expr) => {
        $string.split($splitter).collect::<Vec<&str>>().as_slice()
    };
}

// split input into non-empty lines
pub fn input_to_lines(input: &'static str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|s| !s.is_empty())
}

// split input into non-empty lines, and parse a type from each line
pub fn input_to_parsed_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input_to_lines(input).map(|s| s.parse::<T>().unwrap())
}
