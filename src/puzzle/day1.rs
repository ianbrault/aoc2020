/*
** src/puzzle/day1.rs
*/

use std::collections::BTreeSet;

use crate::puzzle::*;
use crate::utils::input_to_int_lines;

const INPUT: &'static str = include_str!("../../input/1.input");

pub struct Day1 {
    entries: BTreeSet<i64>,
}

impl Day1 {
    pub fn new() -> Self {
        let entries = input_to_int_lines(INPUT).collect();
        Day1 { entries }
    }
}

impl Puzzle for Day1 {
    // Find the two entries that sum to 2020; what do you get if you multiply
    // them together?
    fn part1(&self) -> Result<Solution> {
        // solution: put all entries into a BTree; for each number N, check if
        // 2020 - N is in the BTree; this gives us O(n log n) vs. brute force
        // O(n^2), the pre-processing is probably overkill for input this small
        // but I want to get creative!

        let mut solution = Err(PuzzleError::NoSolution);
        for entry in self.entries.iter() {
            let pair = 2020 - entry;
            if pair > 0 && self.entries.contains(&pair) {
                solution = Ok((entry * pair).into());
            }
        }

        Ok(solution?)
    }

    // What is the product of the three entries that sum to 2020?
    fn part2(&self) -> Result<Solution> {
        // solution: same as above but use a nested loop

        let mut solution = Err(PuzzleError::NoSolution);
        for entry_a in self.entries.iter() {
            for entry_b in self.entries.iter() {
                let partner = 2020 - entry_a - entry_b;
                if partner > 0 && self.entries.contains(&partner) {
                    solution = Ok((entry_a * entry_b * partner).into());
                }
            }
        }

        Ok(solution?)
    }
}
