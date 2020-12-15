/*
** src/puzzle/day10.rs
** https://adventofcode.com/2020/day/10
*/

use crate::puzzle::*;
use crate::utils::{input_to_parsed_lines, PairWith};

const INPUT: &str = include_str!("../../input/10.input");

pub struct Day10 {
    joltages: Vec<u8>,
}

impl Day10 {
    pub fn new() -> Self {
        // parse the adapter joltage ratings and sort
        // note: include both the charging outlet (0-jolt) and the device's
        // build-in adapter (max-jolt + 3)
        let mut joltages = vec![0];
        joltages.extend(input_to_parsed_lines::<u8>(INPUT));

        joltages.sort();
        // doing the push after the sort ensures that we grab the max
        joltages.push(joltages[joltages.len() - 1] + 3);

        Self { joltages }
    }

    fn at(&self, i: usize) -> u8 {
        self.joltages[i]
    }

    fn diff(&self, i: usize, j: usize) -> u8 {
        self.at(j) - self.at(i)
    }
}

impl Puzzle for Day10 {
    // Find a chain that uses all of your adapters to connect the charging
    // outlet to your device's built-in adapter and count the joltage
    // differences between the charging outlet, the adapters, and your device.
    // What is the number of 1-jolt differences multiplied by the number of
    // 3-jolt differences?
    fn part1(&self) -> Result<Solution> {
        let mut one_jolts: u64 = 0;
        let mut three_jolts: u64 = 0;

        // adapter joltages are already sorted, just count the differences
        // for jolt_diff in PairWithDiff::from(self.joltages.iter()) {
        for jolt_diff in self.joltages.iter().pair_with(|&x, &y| y - x) {
            match jolt_diff {
                1 => one_jolts += 1,
                3 => three_jolts += 1,
                // sanity check
                x if x > 3 => unreachable!(),
                _ => {}
            }
        }

        Ok((one_jolts * three_jolts).into())
    }

    // What is the total number of distinct ways you can arrange the adapters
    // to connect the charging outlet to your device?
    fn part2(&self) -> Result<Solution> {
        // we can treat the sorted joltages as a DAG, where vertices are
        // connected by an edge if their differences are <= 3; the solution
        // becomes count the number of paths from the first to last vertex
        let n = self.joltages.len();

        // search in reverse-order and memoize results
        let mut memo = vec![0u64; n];
        // the end should have a value of 1, a little un-intuitive but it makes
        // the math work out
        memo[n - 1] = 1;
        for i in (0..(n - 1)).rev() {
            // the current item could connect to the next 3 items, depending on
            // their separation (no 2 items are separated by more than 3; if
            // not, part 1 would trigger an unreachable! panic)
            if i + 1 < n && self.diff(i, i + 1) <= 3 {
                memo[i] += memo[i + 1];
            }
            // note: simplify branching logic by not nesting the if's
            if i + 2 < n && self.diff(i, i + 2) <= 3 {
                memo[i] += memo[i + 2];
            }
            if i + 3 < n && self.diff(i, i + 3) <= 3 {
                memo[i] += memo[i + 3];
            }
        }

        Ok(memo[0].into())
    }
}
