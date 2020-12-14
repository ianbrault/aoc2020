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

// splits input into non-empty lines
pub fn input_to_lines(input: &'static str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|s| !s.is_empty())
}

// splits input into non-empty lines, and parses a type from each line
pub fn input_to_parsed_lines<T>(input: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input_to_lines(input).map(|s| s.parse::<T>().unwrap())
}

// iterator extension to find both the minimum and maximum elements of an iterator
pub trait MinMax<'a, N>: Iterator<Item = &'a N>
where
    Self: Sized,
    N: PartialOrd + 'a,
{
    fn min_max(self) -> Option<(&'a N, &'a N)> {
        let mut min = None;
        let mut max = None;

        for el in self {
            // compare minimum
            if let Some(m) = min {
                if el < m {
                    min = Some(el);
                }
            } else {
                min = Some(el);
            }

            // compare maximum
            if let Some(m) = max {
                if el > m {
                    max = Some(el);
                }
            } else {
                max = Some(el);
            }
        }

        // if min is set, max is guaranteed to be set as well
        min.map(|m| (m, max.unwrap()))
    }
}

impl<'a, I, N> MinMax<'a, N> for I
where
    I: Iterator<Item = &'a N>,
    N: PartialOrd + 'a,
{
}
