/*
** src/puzzle/day15.rs
** https://adventofcode.com/2020/day/15
*/

use crate::puzzle::*;

const N_GIVEN: u32 = 7;
const INPUT: [u32; N_GIVEN as usize] = [0, 8, 15, 2, 12, 1, 4];

struct MemoryGame;

impl MemoryGame {
    fn run_for(n_turns: u32) -> u64 {
        let mut previous;
        // stores the last turn when a number was spoken
        // for n_turns=30000000 this is huge (56+ MiB) but the cache misses are
        // amortized by avoiding the hashing and reallocation of HashMap
        let mut numbers = vec![0; n_turns as usize];

        // the first N_GIVEN numbers come directly from the puzzle input
        let mut i = 0;
        while i < N_GIVEN {
            numbers[INPUT[i as usize] as usize] = i + 1;
            i += 1;
        }
        previous = INPUT[(N_GIVEN - 1) as usize];

        while i < n_turns {
            // the next number is the number of turns since the previously-
            // spoken number was spoken; if it is not tracked, the previous
            // turn was the first time it was spoken
            // note: insert the previous number instead of the current number
            let last_turn = &mut numbers[previous as usize];
            if *last_turn == 0 {
                *last_turn = i;
            }
            previous = i - *last_turn;
            *last_turn = i;
            i += 1;
        }

        previous as u64
    }
}

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day15 {
    // What will be the 2020th number spoken?
    fn part1(&self) -> Result<Solution> {
        let number = MemoryGame::run_for(2020);
        Ok(number.into())
    }

    // Given your starting numbers, what will be the 30000000th number spoken?
    fn part2(&self) -> Result<Solution> {
        let number = MemoryGame::run_for(30000000);
        Ok(number.into())
    }
}
