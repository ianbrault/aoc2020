 /*
** src/puzzle/day3.rs
*/

use std::convert::TryFrom;

use crate::puzzle::*;
use crate::types::TreeMap;

const INPUT: &'static str = include_str!("../../input/3.input");

pub struct Day3 {
    map: TreeMap,
}

impl Day3 {
    pub fn new() -> Result<Self> {
        let map = TreeMap::try_from(INPUT)?;
        Ok(Day3 { map })
    }
}

impl Puzzle for Day3 {
    // Starting at the top-left corner of your map and following a slope of
    // right 3 and down 1, how many trees would you encounter?
    fn part1(&self) -> Result<Solution> {
        // traverse the tree map, counting encountered trees
        let n_trees = self.map.traverse(1, 3).filter(|b| *b).count();
        Ok((n_trees as i64).into())
    }

    // What do you get if you multiply together the number of trees encountered
    // on each of the listed slopes?
    fn part2(&self) -> Result<Solution> {
        let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

        // traverse the tree map for each given slope
        let mut n_trees = 1;
        for (dy, dx) in slopes {
            let n = self.map.traverse(dy, dx).filter(|b| *b).count();
            n_trees *= n;
        }

        Ok((n_trees as i64).into())
    }
}
