/*
** src/puzzle/day17.rs
** https://adventofcode.com/2020/day/17
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/17.input");

struct CubeAutomaton3D {
    initial_size: usize,
    // active cube sets are double-buffered so that we can do "simultaneous"
    // updates reading from one and writing to the other
    active_cubes_a: HashSet<(i64, i64, i64)>,
    active_cubes_b: HashSet<(i64, i64, i64)>,
    active_set: usize,
}

impl CubeAutomaton3D {
    fn active_cubes(&self) -> usize {
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .len()
    }

    fn is_active(&self, x: i64, y: i64, z: i64) -> bool {
        // check the current set
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .contains(&(x, y, z))
    }

    fn add_cube(&mut self, x: i64, y: i64, z: i64) {
        // add the cube to the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .insert((x, y, z));
    }

    fn remove_cube(&mut self, x: i64, y: i64, z: i64) {
        // remove the cube from the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .remove(&(x, y, z));
    }

    fn active_neighbors(&self, x: i64, y: i64, z: i64) -> usize {
        let active = itertools::iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1), (z - 1)..=(z + 1))
            .filter(|(dx, dy, dz)| self.is_active(*dx, *dy, *dz))
            .count();
        // exclude the given point
        if self.is_active(x, y, z) {
            active - 1
        } else {
            active
        }
    }

    fn run_cycle(&mut self, cycle: i64) {
        // clear the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .clear();

        let x_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let y_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let z_range = (-cycle - 1)..=(cycle + 1);

        for (x, y, z) in itertools::iproduct!(x_range, y_range, z_range) {
            let active_neighbors = self.active_neighbors(x, y, z);
            if self.is_active(x, y, z) {
                if active_neighbors != 2 && active_neighbors != 3 {
                    self.remove_cube(x, y, z);
                } else {
                    self.add_cube(x, y, z);
                }
            } else if active_neighbors == 3 {
                self.add_cube(x, y, z);
            }
        }

        self.active_set = (self.active_set + 1) % 2;
    }

    fn run_to_completion(&mut self, cycles: usize) {
        for n in 0..cycles {
            self.run_cycle(n as i64);
        }
    }
}

impl From<&'static str> for CubeAutomaton3D {
    fn from(s: &'static str) -> Self {
        let mut active_cubes_a = HashSet::new();
        let mut active_cubes_b = HashSet::new();
        let mut initial_size = None;

        for (row, line) in input_to_lines(s).enumerate() {
            initial_size = Some(line.len());
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes_a.insert((col as i64, row as i64, 0));
                    active_cubes_b.insert((col as i64, row as i64, 0));
                }
            }
        }

        Self {
            initial_size: initial_size.unwrap(),
            active_cubes_a,
            active_cubes_b,
            active_set: 0,
        }
    }
}

struct CubeAutomaton4D {
    initial_size: usize,
    // active cube sets are double-buffered so that we can do "simultaneous"
    // updates reading from one and writing to the other
    active_cubes_a: HashSet<(i64, i64, i64, i64)>,
    active_cubes_b: HashSet<(i64, i64, i64, i64)>,
    active_set: usize,
}

impl CubeAutomaton4D {
    fn active_cubes(&self) -> usize {
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .len()
    }

    fn is_active(&self, x: i64, y: i64, z: i64, w: i64) -> bool {
        // check the current set
        match self.active_set {
            0 => &self.active_cubes_a,
            1 => &self.active_cubes_b,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .contains(&(x, y, z, w))
    }

    fn add_cube(&mut self, x: i64, y: i64, z: i64, w: i64) {
        // add the cube to the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .insert((x, y, z, w));
    }

    fn remove_cube(&mut self, x: i64, y: i64, z: i64, w: i64) {
        // remove the cube from the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .remove(&(x, y, z, w));
    }

    fn active_neighbors(&self, x: i64, y: i64, z: i64, w: i64) -> usize {
        let active = itertools::iproduct!((x - 1)..=(x + 1), (y - 1)..=(y + 1), (z - 1)..=(z + 1), (w - 1)..=(w + 1))
            .filter(|(dx, dy, dz, dw)| self.is_active(*dx, *dy, *dz, *dw))
            .count();
        // exclude the given point
        if self.is_active(x, y, z, w) {
            active - 1
        } else {
            active
        }
    }

    fn run_cycle(&mut self, cycle: i64) {
        // clear the upcoming set
        match self.active_set {
            0 => &mut self.active_cubes_b,
            1 => &mut self.active_cubes_a,
            _ => panic!("invalid active cube set {}", self.active_set),
        }
        .clear();

        let x_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let y_range = (-cycle - 1)..=(self.initial_size as i64 + cycle);
        let z_range = (-cycle - 1)..=(cycle + 1);
        let w_range = (-cycle - 1)..=(cycle + 1);

        for (x, y, z, w) in itertools::iproduct!(x_range, y_range, z_range, w_range) {
            let active_neighbors = self.active_neighbors(x, y, z, w);
            if self.is_active(x, y, z, w) {
                if active_neighbors != 2 && active_neighbors != 3 {
                    self.remove_cube(x, y, z, w);
                } else {
                    self.add_cube(x, y, z, w);
                }
            } else if active_neighbors == 3 {
                self.add_cube(x, y, z, w);
            }
        }

        self.active_set = (self.active_set + 1) % 2;
    }

    fn run_to_completion(&mut self, cycles: usize) {
        for n in 0..cycles {
            self.run_cycle(n as i64);
        }
    }
}

impl From<&'static str> for CubeAutomaton4D {
    fn from(s: &'static str) -> Self {
        let mut active_cubes_a = HashSet::new();
        let mut active_cubes_b = HashSet::new();
        let mut initial_size = None;

        for (row, line) in input_to_lines(s).enumerate() {
            initial_size = Some(line.len());
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    active_cubes_a.insert((col as i64, row as i64, 0, 0));
                    active_cubes_b.insert((col as i64, row as i64, 0, 0));
                }
            }
        }

        Self {
            initial_size: initial_size.unwrap(),
            active_cubes_a,
            active_cubes_b,
            active_set: 0,
        }
    }
}

pub struct Day17 {}

impl Day17 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day17 {
    // Starting with your given initial configuration, simulate six cycles in a
    // 3-dimensional space. How many cubes are left in the active state after
    // the sixth cycle?
    fn part1(&self) -> Result<Solution> {
        let mut automaton = CubeAutomaton3D::from(INPUT);
        automaton.run_to_completion(6);
        Ok(automaton.active_cubes().into())
    }

    // Starting with your given initial configuration, simulate six cycles in a
    // 4-dimensional space. How many cubes are left in the active state after
    // the sixth cycle?
    fn part2(&self) -> Result<Solution> {
        let mut automaton = CubeAutomaton4D::from(INPUT);
        automaton.run_to_completion(6);
        Ok(automaton.active_cubes().into())
    }
}
