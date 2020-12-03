/*
** src/utils.rs
*/

use std::str::FromStr;

pub fn input_to_int_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<T>().unwrap())
}
