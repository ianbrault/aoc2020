/*
** src/puzzle/day4.rs
*/

use std::convert::TryFrom;

use crate::puzzle::*;
use crate::types::Passport;

const INPUT: &'static str = include_str!("../../input/4.input");

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }
}

impl Puzzle for Day4 {
    // In your batch file, how many passports are valid?
    // note: does not include field validation
    fn part1(&self) -> Result<Solution> {
        let n_valid = INPUT
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .map(|batch| Passport::has_fields(batch))
            .filter(|&b| b)
            .count();

        Ok((n_valid as u64).into())
    }

    // In your batch file, how many passports are valid?
    // note: includes field validation
    fn part2(&self) -> Result<Solution> {
        let mut passports = vec![];

        // parse passports from the fields in the batch file
        for batch in INPUT.split("\n\n").filter(|s| !s.is_empty()) {
            if let Ok(passport) = Passport::try_from(batch) {
                passports.push(passport);
            }
        }

        Ok((passports.len() as u64).into())
    }
}
