/*
** src/puzzle/day3.rs
** https://adventofcode.com/2020/day/3
*/

use crate::puzzle::{self, Puzzle, Solution};
use crate::types::Bitfield;

const INPUT: &str = include_str!("../../input/3.input");

// terrain map which indicates the locations of trees
pub struct TreeMap {
    // each row is stored as a bitfield, where a bit is set if there is a tree
    map: Vec<Bitfield>,
    pub width: usize,
    pub height: usize,
}

impl TreeMap {
    pub fn at(&self, x: usize, y: usize) -> bool {
        if y >= self.height {
            false
        } else {
            self.map[y].at(x % self.width)
        }
    }

    pub fn traverse(&self, dy: u8, dx: u8) -> TreeMapTraverser {
        TreeMapTraverser::new(self, dy, dx)
    }

    fn parse_map_row(s: &str) -> Bitfield {
        if s.len() > 32 {
            // NOTE: need to use a larger bitfield if this panic! is ever hit
            unreachable!(format!("map row \"{}\" is too long", s))
        } else {
            Bitfield::from(s.chars().map(|c| c == '#'))
        }
    }
}

impl From<&str> for TreeMap {
    fn from(s: &str) -> Self {
        let mut map = vec![];

        // get the width of the first line
        let width = s.split('\n').next().map_or(0, |ss| ss.len());

        for line in s.split('\n').filter(|ss| !ss.is_empty()) {
            map.push(Self::parse_map_row(line));
        }

        let height = map.len();

        Self {
            map,
            width,
            height,
        }
    }
}

// used to traverse a TreeMap at a given slope, as an iterator
pub struct TreeMapTraverser<'a> {
    tree_map: &'a TreeMap,
    dy: u8,
    dx: u8,
    pos: (usize, usize),
}

impl<'a> TreeMapTraverser<'a> {
    fn new(tree_map: &'a TreeMap, dy: u8, dx: u8) -> Self {
        Self {
            tree_map,
            dy,
            dx,
            pos: (0, 0),
        }
    }
}

impl<'a> Iterator for TreeMapTraverser<'a> {
    // each iteration returns whether or not there is a tree at the new position
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.pos;
        x += self.dx as usize;
        y += self.dy as usize;

        let res = if y >= self.tree_map.height {
            // reached the bottom, done iterating
            None
        } else {
            Some(self.tree_map.at(x, y))
        };

        self.pos = (x, y);
        res
    }
}

pub struct Day3 {
    map: TreeMap,
}

impl Day3 {
    pub fn new() -> Self {
        Self {
            map: TreeMap::from(INPUT),
        }
    }
}

impl Puzzle for Day3 {
    // Starting at the top-left corner of your map and following a slope of
    // right 3 and down 1, how many trees would you encounter?
    fn part1(&self) -> puzzle::Result<Solution> {
        // traverse the tree map, counting encountered trees
        let n_trees = self.map.traverse(1, 3).filter(|b| *b).count();
        Ok((n_trees as u64).into())
    }

    // What do you get if you multiply together the number of trees encountered
    // on each of the listed slopes?
    fn part2(&self) -> puzzle::Result<Solution> {
        let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

        // traverse the tree map for each given slope
        let mut n_trees = 1;
        for (dy, dx) in slopes {
            let n = self.map.traverse(dy, dx).filter(|b| *b).count();
            n_trees *= n;
        }

        Ok((n_trees as u64).into())
    }
}
