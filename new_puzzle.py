#!/usr/bin/env python3

import os
import sys

template = """\
/*
** src/puzzle/day<D>.rs
*/

use crate::puzzle::*;

const INPUT: &str = include_str!("../../input/<D>.input");

pub struct Day<D> {}

impl Day<D> {
    pub fn new() -> Self {
        Day<D> {}
    }
}

impl Puzzle for Day<D> {
    // <QUESTION>
    fn part1(&self) -> Result<Solution> {
        unimplemented!()
    }

    // <QUESTION>
    fn part2(&self) -> Result<Solution> {
        unimplemented!()
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

    current_dir = os.path.dirname(os.path.abspath(__file__))
    puzzle_dir = os.path.join(current_dir, "src", "puzzle")

    # write the puzzle source file
    with open(os.path.join(puzzle_dir, 'day%d.rs' % n), 'w') as puzzle_file:
        puzzle_file.write(template.replace('<D>', str(n)))
