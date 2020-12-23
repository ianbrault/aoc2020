/*
** src/utils.rs
*/

use std::iter::Peekable;
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

// takes an iterator and transforms it into a new iterator which combines the
// current and next elements with the provided function
// difference between the current and next elements
pub struct PairWithIter<I, F>
where
    I: Iterator,
{
    inner: Peekable<I>,
    combinator: F,
}

impl<'a, I, N, F> PairWithIter<I, F>
where
    N: 'a,
    I: Iterator<Item = &'a N>,
    F: Fn(&'a N, &'a N) -> N,
{
    pub fn new(iter: I, combinator: F) -> Self {
        Self {
            inner: iter.peekable(),
            combinator,
        }
    }
}

impl<'a, I, N, F> Iterator for PairWithIter<I, F>
where
    N: 'a,
    I: Iterator<Item = &'a N>,
    F: Fn(&'a N, &'a N) -> N,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        // get the next item
        if let Some(curr) = self.inner.next() {
            // peek the following item
            if let Some(after) = self.inner.peek() {
                Some((self.combinator)(curr, after))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// iterator extension for PairWithIter
pub trait PairWith<'a, N, F>: Iterator<Item = &'a N>
where
    Self: Sized,
    N: 'a,
    F: Fn(&'a N, &'a N) -> N,
{
    fn pair_with(self, combinator: F) -> PairWithIter<Self, F> {
        PairWithIter::new(self, combinator)
    }
}

impl<'a, N, F, I> PairWith<'a, N, F> for I
where
    N: 'a,
    I: Iterator<Item = &'a N>,
    F: Fn(&'a N, &'a N) -> N,
{
}
