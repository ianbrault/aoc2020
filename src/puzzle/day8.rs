/*
** src/puzzle/day8.rs
** https://adventofcode.com/2020/day/8
*/

use std::collections::BTreeSet;

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/8.input");

#[derive(Clone, PartialEq)]
enum Operation {
    Accumulate,
    Jump,
    NoOp,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "acc" => Self::Accumulate,
            "jmp" => Self::Jump,
            "nop" => Self::NoOp,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    op: Operation,
    n: i64,
}

impl Instruction {
    fn new(op: Operation, n: i64) -> Self {
        Self { op, n }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        Self {
            op: Operation::from(&s[0..3]),
            n: s[4..s.len()].parse().unwrap(),
        }
    }
}

struct Program {
    acc: i64,
    pc: i64,
    terminated: bool,
}

impl Program {
    fn new() -> Self {
        Self {
            acc: 0,
            pc: 0,
            terminated: false,
        }
    }

    // runs the instructions until the program terminates or an infinite loop
    // is detected and returns the value of the accumulator
    fn run<'a>(&mut self, instructions: &'a [Instruction]) -> i64 {
        // track the past values of the program counter
        let mut pc_hist = BTreeSet::new();

        let mut running = true;
        while running {
            let instr = &instructions[self.pc as usize];
            // store the program counter for the current instruction
            pc_hist.insert(self.pc);

            match instr.op {
                Operation::Accumulate => {
                    self.acc += instr.n;
                    self.pc += 1;
                }
                Operation::Jump => {
                    self.pc += instr.n;
                }
                Operation::NoOp => {
                    self.pc += 1;
                }
            };

            // check if the new program counter value has already been executed
            if pc_hist.contains(&self.pc) {
                // infinite loop detected, stop running the program but do NOT
                // mark the program as terminated
                running = false;
            } else if self.pc as usize == instructions.len() {
                // program terminated nominally
                self.terminated = true;
                running = false;
            }
        }

        self.acc
    }
}

pub struct Day8 {
    instructions: Vec<Instruction>,
}

impl Day8 {
    pub fn new() -> Self {
        let instructions = input_to_lines(INPUT).map(Instruction::from).collect();
        Self { instructions }
    }
}

impl Puzzle for Day8 {
    // Immediately before any instruction is executed a second time, what value
    // is in the accumulator?
    fn part1(&self) -> Result<Solution> {
        let mut program = Program::new();
        Ok(program.run(&self.instructions).into())
    }

    // Fix the program so that it terminates normally by changing exactly one
    // jmp (to nop) or nop (to jmp). What is the value of the accumulator after
    // the program terminates?
    fn part2(&self) -> Result<Solution> {
        let mut solution = Err(PuzzleError::NoSolution);

        // used to invert the jmp/nop instructions
        let opposite = |op: &Operation| {
            if *op == Operation::Jump {
                Operation::NoOp
            } else if *op == Operation::NoOp {
                Operation::Jump
            } else {
                unreachable!()
            }
        };

        // for each jmp/nop instruction, try the program with the opposite
        let mut instr_index = 0;
        while instr_index < self.instructions.len() {
            // skip any acc instructions
            while self.instructions[instr_index].op == Operation::Accumulate {
                instr_index += 1;
            }

            let new_instructions = self
                .instructions
                .iter()
                .enumerate()
                .map(|(i, instr)| {
                    if i == instr_index {
                        Instruction::new(opposite(&instr.op), instr.n)
                    } else {
                        instr.clone()
                    }
                })
                .collect::<Vec<Instruction>>();

            let mut program = Program::new();
            let rc = program.run(&new_instructions);
            if program.terminated {
                solution = Ok(rc);
                instr_index = self.instructions.len();
            } else {
                instr_index += 1;
            }
        }

        Ok(solution?.into())
    }
}
