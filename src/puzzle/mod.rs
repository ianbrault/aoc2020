/*
** src/puzzle/mod.rs
*/

mod day1;

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
}

impl From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
        }
    }
}

// puzzles for each day are trait objects which conform to the following interface
pub trait Puzzle {
    fn part1(&self) -> Result<Solution>;
    fn part2(&self) -> Result<Solution>;
}

pub fn all_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(day1::Day1::new())]
}

#[derive(Debug)]
pub enum PuzzleError {
    NoSolution,
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSolution => write!(f, "no solution found for the puzzle"),
        }
    }
}

impl error::Error for PuzzleError {}
