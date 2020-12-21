/*
** src/puzzle/day12.rs
** https://adventofcode.com/2020/day/12
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/12.input");

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn is_cardinal(&self) -> bool {
        match self {
            Direction::North | Direction::South | Direction::East | Direction::West => true,
            _ => false,
        }
    }

    fn is_rotational(&self) -> bool {
        match self {
            Direction::Left | Direction::Right => true,
            _ => false,
        }
    }

    fn rotate_left(&mut self) {
        match self {
            Direction::North => *self = Direction::West,
            Direction::South => *self = Direction::East,
            Direction::East => *self = Direction::North,
            Direction::West => *self = Direction::South,
            _ => unreachable!(),
        }
    }

    fn rotate_right(&mut self) {
        match self {
            Direction::North => *self = Direction::East,
            Direction::South => *self = Direction::West,
            Direction::East => *self = Direction::South,
            Direction::West => *self = Direction::North,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => unreachable!(),
        }
    }
}

struct NavigationInstruction {
    direction: Direction,
    distance: i32,
}

impl From<&str> for NavigationInstruction {
    fn from(s: &str) -> Self {
        let direction = Direction::from(s.chars().next().unwrap());
        let distance = s[1..s.len()].parse().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

struct Navigator<I> {
    x: i32,
    y: i32,
    direction: Direction,
    instructions: I,
    waypoint: Option<(i32, i32)>,
}

impl<I> Navigator<I> {
    fn with_waypoint(mut self, x: i32, y: i32) -> Self {
        self.waypoint = Some((x, y));
        self
    }

    fn direction_to_dx_dy(direction: Direction, distance: i32) -> (i32, i32) {
        match direction {
            Direction::North => (0, distance),
            Direction::South => (0, -distance),
            Direction::East => (distance, 0),
            Direction::West => (-distance, 0),
            _ => unreachable!(),
        }
    }

    fn move_ship(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn move_waypoint(&mut self, dx: i32, dy: i32) {
        if let Some((x, y)) = self.waypoint {
            self.waypoint = Some((x + dx, y + dy));
        } else {
            unreachable!()
        }
    }

    fn move_forward(&mut self, distance: i32) {
        if let Some((wx, wy)) = self.waypoint {
            for _ in 0..distance {
                self.move_ship(wx, wy);
            }
        } else {
            let (dx, dy) = Self::direction_to_dx_dy(self.direction, distance);
            self.move_ship(dx, dy);
        }
    }

    fn moves(&mut self, direction: Direction, distance: i32) {
        assert!(direction.is_cardinal());

        let (dx, dy) = Self::direction_to_dx_dy(direction, distance);

        // move the waypoint, if it is set
        // otherwise move the ship
        if self.waypoint.is_some() {
            self.move_waypoint(dx, dy);
        } else {
            self.move_ship(dx, dy);
        }
    }

    fn rotate_ship(&mut self, direction: Direction, degrees: i32) {
        let rotator = match direction {
            Direction::Left => Direction::rotate_left,
            Direction::Right => Direction::rotate_right,
            _ => unreachable!(),
        };

        for _ in 0..(degrees / 90) {
            rotator(&mut self.direction);
        }
    }

    fn rotate_waypoint(&mut self, direction: Direction, degrees: i32) {
        if let Some((mut x, mut y)) = self.waypoint {
            match direction {
                Direction::Left => {
                    for _ in 0..(degrees / 90) {
                        let t = x;
                        x = -y;
                        y = t;
                    }
                }
                Direction::Right => {
                    for _ in 0..(degrees / 90) {
                        let t = y;
                        y = -x;
                        x = t;
                    }
                }
                _ => unreachable!(),
            };
            self.waypoint = Some((x, y));
        } else {
            unreachable!()
        }
    }

    fn rotates(&mut self, direction: Direction, degrees: i32) {
        assert!(direction.is_rotational());
        assert!(degrees % 90 == 0);

        // rotate the waypoint, if it is set
        // otherwise rotate the ship
        if self.waypoint.is_some() {
            self.rotate_waypoint(direction, degrees);
        } else {
            self.rotate_ship(direction, degrees);
        }
    }
}

impl<'a, I> From<I> for Navigator<I>
where
    I: Iterator<Item = &'a NavigationInstruction>,
{
    fn from(instructions: I) -> Self {
        Self {
            x: 0,
            y: 0,
            // ship starts facing East
            direction: Direction::East,
            instructions,
            waypoint: None,
        }
    }
}

impl<'a, I> Iterator for Navigator<I>
where
    I: Iterator<Item = &'a NavigationInstruction>,
{
    // each iteration returns the new position
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        // consume instructions until they have been exhausted
        if let Some(instr) = self.instructions.next() {
            match instr.direction {
                dir if dir.is_cardinal() => self.moves(dir, instr.distance),
                dir if dir.is_rotational() => self.rotates(dir, instr.distance),
                Direction::Forward => self.move_forward(instr.distance),
                _ => unreachable!(),
            };
            Some((self.x, self.y))
        } else {
            None
        }
    }
}

pub struct Day12 {
    navigation_instructions: Vec<NavigationInstruction>,
}

impl Day12 {
    pub fn new() -> Self {
        let navigation_instructions = input_to_lines(INPUT)
            .map(NavigationInstruction::from)
            .collect();

        Self {
            navigation_instructions,
        }
    }

    fn manhattan_distance<X, Y>(x: X, y: Y) -> i64
    where
        X: Into<i64>,
        Y: Into<i64>,
    {
        x.into().abs() + y.into().abs()
    }
}

impl Puzzle for Day12 {
    // Figure out where the navigation instructions lead. What is the Manhattan
    // distance between that location and the ship's starting position?
    fn part1(&self) -> Result<Solution> {
        let (x, y) = Navigator::from(self.navigation_instructions.iter())
            .last()
            .unwrap();
        Ok(Self::manhattan_distance(x, y).into())
    }

    // Figure out where the navigation instructions actually lead (using the
    // ship waypoint). What is the Manhattan distance between that location and
    // the ship's starting position?
    fn part2(&self) -> Result<Solution> {
        let (x, y) = Navigator::from(self.navigation_instructions.iter())
            .with_waypoint(10, 1)
            .last()
            .unwrap();
        Ok(Self::manhattan_distance(x, y).into())
    }
}
