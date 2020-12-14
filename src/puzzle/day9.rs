/*
** src/puzzle/day9.rs
** https://adventofcode.com/2020/day/9
*/

use std::collections::BTreeSet;

use crate::puzzle::*;
use crate::utils::{input_to_parsed_lines, MinMax};

const INPUT: &str = include_str!("../../input/9.input");

pub struct Day9 {
    numbers: Vec<u64>,
}

impl Day9 {
    pub fn new() -> Self {
        Self {
            numbers: input_to_parsed_lines(INPUT).collect(),
        }
    }
}

impl Puzzle for Day9 {
    // Find the first number in the list (after the preamble) which is not the
    // sum of two of the 25 numbers before it
    fn part1(&self) -> Result<Solution> {
        // grab the first 25 numbers for the preamble and store in a set
        let mut preamble = self.numbers.iter().take(25).collect::<BTreeSet<_>>();

        // iterate thru the remaining numbers to search for the solution
        let mut solution = Err(PuzzleError::NoSolution);
        for (i, number) in self.numbers.iter().skip(25).enumerate() {
            // check if the number is the sum of anything in the preamble
            let mut number_is_sum = false;
            for &&n in preamble.iter() {
                // first condition is necessary to avoid u64 underflow
                // second condition ensures that the 2 numbers are disjoint
                if (*number > n) && (n * 2 != *number) && preamble.contains(&(number - n)) {
                    number_is_sum = true;
                    break;
                }
            }

            if !number_is_sum {
                solution = Ok(*number);
            }

            // remove the oldest preamble entry and replace it with the current
            // note: we enumerate after .skip(25) so i starts at 0 and thus
            // tracks the oldest preamble entry
            preamble.remove(&self.numbers[i]);
            preamble.insert(number);
        }

        Ok(solution?.into())
    }

    // Find a contiguous set of at least two numbers in your list which sum to
    // the invalid number from step 1. To find the encryption weakness, sum the
    // smallest and largest number in this contiguous range. What is the
    // encryption weakness in your XMAS-encrypted list of numbers?
    fn part2(&self) -> Result<Solution> {
        // NOTE: this is the answer from part 1
        let target = 23278925;

        let mut solution = Err(PuzzleError::NoSolution);

        // check a sequence of sliding sums
        // bump up the lower end once the sum is greater than the target
        let mut lower = 0;
        let mut upper;
        let mut sum;
        while lower < self.numbers.len() - 1 {
            upper = lower + 1;
            sum = self.numbers[lower];

            while sum < target {
                sum += self.numbers[upper];
                upper += 1;
            }

            if sum == target {
                // find the min and max in the range
                // FIXME: add a min_max iterator adaptor
                let (min, max) = self.numbers[lower..upper].iter().min_max().unwrap();
                solution = Ok(min + max);
                break;
            } else {
                lower += 1;
            }
        }

        Ok(solution?.into())
    }
}
