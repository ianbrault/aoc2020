/*
** src/puzzle/mod.rs
*/

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// variant to cover various solution types
#[derive(Debug)]
pub enum Solution {
    Int(i64),
    UInt(u64),
}

impl From<i64> for Solution {
    fn from(n: i64) -> Self {
        Self::Int(n)
    }
}

impl From<u64> for Solution {
    fn from(n: u64) -> Self {
        Self::UInt(n)
    }
}

impl From<usize> for Solution {
    fn from(n: usize) -> Self {
        Self::UInt(n as u64)
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::UInt(u) => write!(f, "{}", u),
        }
    }
}

// puzzles for each day are trait objects which conform to the following interface
pub trait Puzzle {
    fn part1(&self) -> Result<Solution>;
    fn part2(&self) -> Result<Solution>;
}

pub fn all_puzzles() -> Result<Vec<Box<dyn Puzzle>>> {
    Ok(vec![
        Box::new(day1::Day1::new()),
        Box::new(day2::Day2::new()),
        Box::new(day3::Day3::new()),
        Box::new(day4::Day4::new()),
        Box::new(day5::Day5::new()),
        Box::new(day6::Day6::new()),
        Box::new(day7::Day7::new()),
        Box::new(day8::Day8::new()),
        Box::new(day9::Day9::new()),
        Box::new(day10::Day10::new()),
        Box::new(day11::Day11::new()),
        Box::new(day12::Day12::new()),
        Box::new(day13::Day13::new()),
        Box::new(day14::Day14::new()),
        Box::new(day15::Day15::new()),
        Box::new(day16::Day16::new()),
        Box::new(day17::Day17::new()),
        Box::new(day18::Day18::new()),
    ])
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
