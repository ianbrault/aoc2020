/*
** src/puzzle/day6.rs
** https://adventofcode.com/2020/day/6
*/

use std::collections::BTreeSet;

use crate::puzzle::{self, Puzzle, Solution};
use crate::types::Counter;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/6.input");

pub struct Day6 {
    groups: Vec<&'static str>,
}

impl Day6 {
    pub fn new() -> Self {
        let groups = INPUT
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .collect();

        Self { groups }
    }
}

impl Puzzle for Day6 {
    // What is the sum of the number of unique questions answered "yes" to in
    // each group?
    fn part1(&self) -> puzzle::Result<Solution> {
        let mut sum = 0;

        for group in self.groups.iter() {
            // store the unique "yes" answers for each group in a set
            let mut unique_answers = BTreeSet::new();

            for line in input_to_lines(group) {
                for c in line.chars() {
                    unique_answers.insert(c);
                }
            }

            sum += unique_answers.len() as u64;
        }

        Ok(sum.into())
    }

    // For each group, count the number of questions to which everyone answered
    // "yes". What is the sum of those counts?
    fn part2(&self) -> puzzle::Result<Solution> {
        let mut sum = 0;

        // for each group, track the number of respondents, and the frequency
        // of each answer; the number of questions to which everyone answered
        // yes is each entry where the count == number of respondents
        for group in self.groups.iter() {
            let mut counter = Counter::new();

            let n_answers = input_to_lines(group).count();
            for response in input_to_lines(group) {
                counter.extend(response.chars());
            }

            let all_answered = counter
                .into_iter()
                .filter(|(_, count)| *count == n_answers)
                .count();

            sum += all_answered as u64;
        }

        Ok(sum.into())
    }
}
