/*
** src/utils.rs
*/

use std::str::FromStr;

pub fn input_to_lines(input: &'static str) -> impl Iterator<Item = &str> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
}

pub fn input_to_int_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input_to_lines(input)
        .map(|s| s.parse::<T>().unwrap())
}
