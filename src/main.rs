/*
** src/main.rs
*/

#![feature(generic_const_exprs)]

#[macro_use]
mod utils;

mod puzzle;
mod types;

fn run() -> puzzle::Result<()> {
    println!("Advent of Code 2020\nsolutions by Ian Brault");

    for (day, puz) in puzzle::all_puzzles()?.into_iter().enumerate() {
        // part 1
        let sol_1 = puz.part1()?;
        println!("Day {}: part 1: {}", day + 1, sol_1);

        // part 2
        let sol_2 = puz.part2()?;
        println!("Day {}: part 2: {}", day + 1, sol_2);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
    }
}
