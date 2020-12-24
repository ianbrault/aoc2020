/*
** src/puzzle/day14.rs
** https://adventofcode.com/2020/day/14
*/

use std::collections::HashMap;

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/14.input");

const BITS: usize = 36;
const BITMASK: u64 = 0xfffffffff;

#[derive(Clone, Copy, PartialEq)]
enum MaskBit {
    Zero,
    One,
    X,
}

impl From<char> for MaskBit {
    fn from(c: char) -> Self {
        match c {
            '0' => Self::Zero,
            '1' => Self::One,
            'X' => Self::X,
            _ => unreachable!(),
        }
    }
}

// given a number and a set of "floating bits" - which take a superposition of
// all possble values - generate all resulting numeric permutations
struct FloatingBitsPermutations {
    n: u64,
    floating_bits: Vec<usize>,
    // used for iteration
    i: usize,
}

impl FloatingBitsPermutations {
    fn new(n: u64, floating_bits: Vec<usize>) -> Self {
        Self {
            n,
            floating_bits,
            i: 0,
        }
    }

    fn apply_floating_bits(mut n: u64, bit_vals: Vec<(usize, usize)>) -> u64 {
        for (bit, bit_val) in bit_vals {
            match bit_val {
                0 => n &= !(1 << bit),
                1 => n |= 1 << bit,
                _ => unreachable!(),
            }
        }

        n
    }
}

impl Iterator for FloatingBitsPermutations {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n_bits = self.floating_bits.len();
        if self.i == 2usize.pow(n_bits as u32) {
            None
        } else {
            // with n floating bits, the value of the floating bit at postion j
            // on iteration i is (i / 2^(n - j - 1)) % 2
            let bit_vals = self
                .floating_bits
                .iter()
                .enumerate()
                .map(|(j, b)| (*b, (self.i / 2usize.pow((n_bits - j - 1) as u32)) % 2))
                .collect();

            self.i += 1;
            Some(Self::apply_floating_bits(self.n, bit_vals))
        }
    }
}

struct Mask {
    bits: [MaskBit; BITS],
}

impl Mask {
    fn apply_to(&self, mut n: u64) -> u64 {
        for (i, bit) in self.bits.iter().enumerate() {
            match bit {
                MaskBit::Zero => n &= !(1 << i),
                MaskBit::One => n |= 1 << i,
                _ => {}
            }
        }

        n & BITMASK
    }

    fn apply_to_with_floating(&self, mut n: u64) -> impl Iterator<Item = u64> {
        // set all One bits to 1, and mark the floating bits
        let mut floating_bits = vec![];
        for (i, bit) in self.bits.iter().enumerate() {
            match bit {
                MaskBit::One => n |= 1 << i,
                MaskBit::X => floating_bits.push(i),
                _ => {}
            }
        }

        // generate all possible permutations of floating bits
        FloatingBitsPermutations::new(n, floating_bits)
    }
}

impl From<&str> for Mask {
    fn from(s: &str) -> Self {
        let mut bits = [MaskBit::X; BITS];

        // iterate in reverse to start from the least-significant bit
        for (i, c) in s.chars().rev().enumerate() {
            bits[i] = MaskBit::from(c);
        }

        Self { bits }
    }
}

enum Instruction {
    SetMask(Mask),
    SetMem(u64, u64),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        if s.starts_with("mask") {
            let mask = Mask::from(&s[7..s.len()]);
            Self::SetMask(mask)
        } else if s.starts_with("mem") {
            let end_bracket = s.find(']').unwrap();
            let addr = s[4..end_bracket].parse().unwrap();
            let val = s[(end_bracket + 4)..s.len()].parse().unwrap();
            Self::SetMem(addr, val)
        } else {
            unreachable!()
        }
    }
}

struct Program<'a> {
    // it is a bad idea to represent the full 36-bit address space, use a
    // sparse hashmap-based representation instead
    memory: HashMap<u64, u64>,
    // tracks the current mask value
    // note: this must be set by the 1st instruction
    current_mask: Option<&'a Mask>,
}

impl<'a> Program<'a> {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            current_mask: None,
        }
    }

    fn mask(&self) -> &Mask {
        if let Some(mask) = self.current_mask {
            mask
        } else {
            unreachable!()
        }
    }

    fn run_v1(&mut self, instructions: impl Iterator<Item = &'a Instruction>) {
        for instr in instructions {
            match instr {
                Instruction::SetMask(mask) => {
                    self.current_mask = Some(mask);
                }
                Instruction::SetMem(addr, value) => {
                    // apply the mask to the value and write to the address
                    let value = self.mask().apply_to(*value);
                    self.memory.insert(*addr, value);
                }
            }
        }
    }

    fn run_v2(&mut self, instructions: impl Iterator<Item = &'a Instruction>) {
        for instr in instructions {
            match instr {
                Instruction::SetMask(mask) => {
                    self.current_mask = Some(mask);
                }
                Instruction::SetMem(addr, value) => {
                    // apply the mask to the address and write to all possible
                    // address permutations, via floating bits
                    for addr in self.mask().apply_to_with_floating(*addr) {
                        self.memory.insert(addr, *value);
                    }
                }
            }
        }
    }
}

pub struct Day14 {
    instructions: Vec<Instruction>,
}

impl Day14 {
    pub fn new() -> Self {
        let instructions = input_to_lines(INPUT).map(Instruction::from).collect();
        Self { instructions }
    }
}

impl Puzzle for Day14 {
    // Execute the initialization program. What is the sum of all values left
    // in memory after it completes?
    fn part1(&self) -> Result<Solution> {
        let mut program = Program::new();
        program.run_v1(self.instructions.iter());
        let sum = program.memory.values().filter(|&&v| v != 0).sum::<u64>();
        Ok(sum.into())
    }

    // Execute the initialization program using an emulator for a version 2
    // decoder chip. What is the sum of all values left in memory after it
    // completes?
    fn part2(&self) -> Result<Solution> {
        let mut program = Program::new();
        program.run_v2(self.instructions.iter());
        let sum = program.memory.values().filter(|&&v| v != 0).sum::<u64>();
        Ok(sum.into())
    }
}
