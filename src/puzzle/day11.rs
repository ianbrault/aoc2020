/*
** src/puzzle/day11.rs
** https://adventofcode.com/2020/day/11
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/11.input");

#[derive(Clone, Copy)]
enum State {
    Floor,
    Empty,
    Occupied,
}

impl State {
    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    fn is_occupied(&self) -> bool {
        match self {
            Self::Occupied => true,
            _ => false,
        }
    }

    fn occupied(&self) -> u8 {
        match self {
            Self::Occupied => 1,
            _ => 0,
        }
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

enum Visibility {
    Adjacent,
    LineOfSight,
    // default option to support builder pattern
    NotSet,
}

// the ferry seating is a cellular automaton
//
// the rule is:
// if a seat is empty and there are no occupied seats in the neighborhood, the
// seat becomes occupied; if a seat is occupied and 4 or more seats in the
// neighborhood are also occupied, the seat becomes empty; otherwise, no change
//
// the neighborhood includes up, down, left, right, and diagonals
struct FerryAutomaton<const GRID_SIZE: usize>
where
    [(); (GRID_SIZE + 2) * (GRID_SIZE + 2)]: Sized,
{
    // automaton generation is double-buffered; the rules are applied to the
    // current generation and results are stored in the future generation which
    // allows us to do an "atomic update", i.e. the incomplete results in the
    // future generation will not cause problems
    // note: we can cheat a bit by adding an extra cell to the borders of the
    // grid so we do not have to bounds-check when checking neighbors
    generation_a: [State; (GRID_SIZE + 2) * (GRID_SIZE + 2)],
    generation_b: [State; (GRID_SIZE + 2) * (GRID_SIZE + 2)],
    // tracks the current (and thus, future) generation
    generation: u8,
    // rule configuration
    visibility: Visibility,
    occupied_threshold: u8,
}

impl<const GRID_SIZE: usize> FerryAutomaton<GRID_SIZE>
where
    [(); (GRID_SIZE + 2) * (GRID_SIZE + 2)]: Sized,
{
    // to be used following From<&str> in support of the builder pattern
    fn with(mut self, visibility: Visibility, occupied_threshold: u8) -> Self {
        self.visibility = visibility;
        self.occupied_threshold = occupied_threshold;
        self
    }

    // note: gets from the current generation
    fn get(&self, row: usize, col: usize) -> State {
        if self.generation == 0 {
            self.generation_a[(row * (GRID_SIZE + 2)) + col]
        } else {
            self.generation_b[(row * (GRID_SIZE + 2)) + col]
        }
    }

    // note: sets to the future generation
    fn set(&mut self, row: usize, col: usize, state: State) {
        if self.generation == 0 {
            self.generation_b[(row * (GRID_SIZE + 2)) + col] = state;
        } else {
            self.generation_a[(row * (GRID_SIZE + 2)) + col] = state;
        }
    }

    fn occupied_adjacent(&self, row: usize, col: usize) -> u8 {
        let up = self.get(row - 1, col).occupied();
        let down = self.get(row + 1, col).occupied();
        let left = self.get(row, col - 1).occupied();
        let right = self.get(row, col + 1).occupied();
        let uleft = self.get(row - 1, col - 1).occupied();
        let uright = self.get(row - 1, col + 1).occupied();
        let lleft = self.get(row + 1, col - 1).occupied();
        let lright = self.get(row + 1, col + 1).occupied();

        up + down + left + right + uleft + uright + lleft + lright
    }

    fn check_line_of_sight(&self, from_y: usize, from_x: usize, dy: i32, dx: i32) -> bool {
        // convert everything to signed so math becomes easier
        let mut y = (from_y as i32) + dy;
        let mut x = (from_x as i32) + dx;

        while x >= 0 && y >= 0 && (x as usize) < (GRID_SIZE + 2) && (y as usize) < (GRID_SIZE + 2) {
            match self.get(y as usize, x as usize) {
                State::Occupied => return true,
                State::Empty => return false,
                _ => {
                    y += dy;
                    x += dx;
                }
            }
        }

        false
    }

    fn occupied_line_of_sight(&self, row: usize, col: usize) -> u8 {
        let mut occupied = 0;

        // up
        if self.check_line_of_sight(row, col, -1, 0) {
            occupied += 1;
        }
        // down
        if self.check_line_of_sight(row, col, 1, 0) {
            occupied += 1;
        }
        // left
        if self.check_line_of_sight(row, col, 0, -1) {
            occupied += 1;
        }
        // right
        if self.check_line_of_sight(row, col, 0, 1) {
            occupied += 1;
        }
        // upper-left
        if self.check_line_of_sight(row, col, -1, -1) {
            occupied += 1;
        }
        // upper-right
        if self.check_line_of_sight(row, col, -1, 1) {
            occupied += 1;
        }
        // lower-left
        if self.check_line_of_sight(row, col, 1, -1) {
            occupied += 1;
        }
        // lower-right
        if self.check_line_of_sight(row, col, 1, 1) {
            occupied += 1;
        }

        occupied
    }

    fn visible_occupied(&self, row: usize, col: usize) -> u8 {
        match self.visibility {
            Visibility::Adjacent => self.occupied_adjacent(row, col),
            Visibility::LineOfSight => self.occupied_line_of_sight(row, col),
            // should never be hit unless Self::with() has not been called
            _ => unreachable!(),
        }
    }

    // creates the next generation of the automaton by applying the rule to the
    // current generation; returns the number of cells that changed state
    fn run(&mut self) -> u32 {
        let mut changed = 0;

        // iterate thru the grid, accounting for the padding along the borders
        for row in 1..=GRID_SIZE {
            for col in 1..=GRID_SIZE {
                let mut state = self.get(row, col);
                // note: save a bit by not checking adjacencies for the floor
                if state.is_empty() && self.visible_occupied(row, col) == 0 {
                    state = State::Occupied;
                    changed += 1;
                } else if state.is_occupied()
                    && self.visible_occupied(row, col) >= self.occupied_threshold
                {
                    state = State::Empty;
                    changed += 1;
                }
                self.set(row, col, state);
            }
        }

        // advance to the next generation, and return
        self.generation = (self.generation + 1) % 2;
        changed
    }

    // run to a fixed point (no seats change)
    fn run_to_completion(&mut self) {
        let mut changed = u32::MAX;
        while changed > 0 {
            changed = self.run();
        }
    }

    fn occupied_seats(&self) -> u64 {
        if self.generation == 0 {
            self.generation_a
        } else {
            self.generation_b
        }
        .iter()
        .filter(|cell| cell.is_occupied())
        .count() as u64
    }
}

impl<const GRID_SIZE: usize> From<&'static str> for FerryAutomaton<GRID_SIZE>
where
    [(); (GRID_SIZE + 2) * (GRID_SIZE + 2)]: Sized,
{
    fn from(s: &'static str) -> Self {
        // build up both generations from scratch
        let mut generation_a = [State::Floor; (GRID_SIZE + 2) * (GRID_SIZE + 2)];
        let mut generation_b = [State::Floor; (GRID_SIZE + 2) * (GRID_SIZE + 2)];

        for (row, line) in input_to_lines(s).enumerate() {
            for (col, c) in line.chars().enumerate() {
                // note: account for the row of padding in front
                let i = ((row + 1) * (GRID_SIZE + 2)) + col + 1;
                let state = State::from(c);
                generation_a[i] = state;
                generation_b[i] = state;
            }
        }

        Self {
            generation_a,
            generation_b,
            generation: 0,
            // default options, call Self::with() afterwards
            visibility: Visibility::NotSet,
            occupied_threshold: 0,
        }
    }
}

pub struct Day11 {}

impl Day11 {
    pub fn new() -> Self {
        Self {}
    }
}

const SIZE: usize = 98;

impl Puzzle for Day11 {
    // Simulate your seating area by applying the seating rules repeatedly
    // until no seats change state. How many seats end up occupied?
    fn part1(&self) -> Result<Solution> {
        let mut automaton = FerryAutomaton::<SIZE>::from(INPUT).with(Visibility::Adjacent, 4);
        automaton.run_to_completion();
        Ok(automaton.occupied_seats().into())
    }

    // Given the new visibility method and the rule change for occupied seats
    // becoming empty, once equilibrium is reached, how many seats end up
    // occupied?
    fn part2(&self) -> Result<Solution> {
        let mut automaton = FerryAutomaton::<SIZE>::from(INPUT).with(Visibility::LineOfSight, 5);
        automaton.run_to_completion();
        Ok(automaton.occupied_seats().into())
    }
}
