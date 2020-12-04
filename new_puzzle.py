#!/usr/bin/env python

import argparse
import os
import sys

template = """ \
/*
** src/puzzle/day{day}.rs
*/

use crate::puzzle::*;

const INPUT: &'static str = include_str!("../../input/{day}.input");

pub struct Day{day} {}

impl Day{day} {
    pub fn new() -> Self {
        Day{day} {}
    }
}

impl Puzzle for Day{day} {
    // <QUESTION>
    fn part1(&self) -> Result<Solution> {
        Ok((-1).into())
    }

    // <QUESTION>
    fn part2(&self) -> Result<Solution> {
        Ok((-1).into())
    }
}
"""


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit("error: missing argument DAY")

    n = sys.argv[1]
    try:
        n = int(n)
    except ValueError:
        sys.exit("error: invalid argument DAY: %r" % n)

    puzzle_dir = os.path.join(os.path.abspath(__file__), "src", "puzzle")
    with open(os.path.join(puzzle_dir, 'day%d.rs' % n), 'w') as puzzle_file:
        puzzle_file.write(template.format(day=n))
