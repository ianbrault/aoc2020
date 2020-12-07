/*
** src/puzzle/day5.rs
** https://adventofcode.com/2020/day/5
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/5.input");

struct BoardingPass {
    id: u64,
}

impl BoardingPass {
    // performs the binary search to find a row/column based on the boarding
    // pass's string representation
    fn binary_partition(slice: &str, min: u8, max: u8, cmin: char, cmax: char) -> u8 {
        let mut mmin = min;
        let mut mmax = max;

        for c in slice.chars() {
            let delta = (mmax - mmin) / 2;
            match c {
                _ if c == cmin => {
                    mmax -= delta;
                },
                _ if c == cmax => {
                    mmin += delta;
                },
                _ => unreachable!(),
            }
        }

        mmin
    }

    // converts the boarding pass string representation into a row and column
    fn str_to_row_col<S>(s: S) -> (u8, u8)
    where S: AsRef<str>
    {
        let row = Self::binary_partition(&s.as_ref()[0..7], 0, 128, 'F', 'B');
        let col = Self::binary_partition(&s.as_ref()[7..10], 0, 8, 'L', 'R');
        (row, col)
    }
}

impl<S> From<S> for BoardingPass
where S: AsRef<str>
{
    fn from(s: S) -> Self {
        let (row, col) = Self::str_to_row_col(s);
        Self {
            id: ((row as u64) * 8) + (col as u64),
        }
    }
}

pub struct Day5 {
    boarding_passes: Vec<BoardingPass>,
}

impl Day5 {
    pub fn new() -> Self {
        let boarding_passes = input_to_lines(INPUT)
            .map(BoardingPass::from)
            .collect();

        Self {
            boarding_passes
        }
    }
}

impl Puzzle for Day5 {
    // What is the highest seat ID on a boarding pass?
    fn part1(&self) -> Result<Solution> {
        let max_id = self.boarding_passes
            .iter()
            .map(|bp| bp.id)
            .max().unwrap();

        Ok(max_id.into())
    }

    // What is the ID of your seat?
    fn part2(&self) -> Result<Solution> {
        // collect boarding pass IDs and sort
        let mut bp_ids = self.boarding_passes
            .iter()
            .map(|bp| bp.id)
            .collect::<Vec<u64>>();
        bp_ids.sort();

        // find boarding pass IDs which have a gap of 1
        let mut my_id = Err(PuzzleError::NoSolution);
        for i in 0..(bp_ids.len() - 1) {
            if bp_ids[i + 1] - bp_ids[i] == 2 {
                my_id = Ok(((bp_ids[i] + 1) as u64).into());
            }
        }

        Ok(my_id?)
    }
}
