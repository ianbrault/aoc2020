/*
** src/puzzle/day13.rs
** https://adventofcode.com/2020/day/13
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/13.input");

// an adaptation of Bézout's identity (using the extended Euclidean algorithm)
// for modular integers
// see: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
fn inverse(a: i64, n: i64) -> i64 {
    let (mut t, mut new_t) = (0, 1);
    let (mut r, mut new_r) = (n, a);

    while new_r != 0 {
        let q = r / new_r;

        let tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;

        let tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }

    if r > 1 {
        panic!("a is not invertible");
    }

    if t < 0 {
        t + n
    } else {
        t
    }
}

pub struct Day13 {
    earliest_departure: u64,
    bus_ids: Vec<u64>,
}

impl Day13 {
    pub fn new() -> Self {
        let lines = input_to_lines(INPUT).collect::<Vec<&str>>();
        let (earliest_str, ids_str) = match lines.as_slice() {
            [earliest, ids] => (earliest, ids),
            _ => unreachable!(),
        };

        let earliest_departure = earliest_str.parse().unwrap();
        let bus_ids = ids_str
            .split(',')
            .map(|s| {
                if s == "x" {
                    // leave placeholder values for out-of-service buses
                    0
                } else {
                    s.parse().unwrap()
                }
            })
            .collect::<Vec<u64>>();

        Self {
            earliest_departure,
            bus_ids,
        }
    }
}

impl Puzzle for Day13 {
    // What is the ID of the earliest bus you can take to the airport
    // multiplied by the number of minutes you'll need to wait for that bus?
    fn part1(&self) -> Result<Solution> {
        // the multiple of bus ID B that is closest (and greater than) our
        // timestamp T is T + B - (T % B), so the difference is B - (T % B)
        let (id, delay) = self
            .bus_ids
            .iter()
            .filter(|&&bid| bid > 0)
            .map(|bid| (bid, bid - (self.earliest_departure % bid)))
            .min_by_key(|(_, delay)| *delay)
            .unwrap();

        Ok((id * delay).into())
    }

    // What is the earliest timestamp such that all of the listed bus IDs
    // depart at offsets matching their positions in the list?
    #[allow(non_snake_case)]
    fn part2(&self) -> Result<Solution> {
        // the non-brute-force solution uses the Chinese Remainder Theorem,
        // with the existence direct construction:
        // by inspection, note that all bus IDs are prime numbers, therefore
        // all possible pairs are coprime;
        // the IDs and offsets form a system of congruences where the solution
        // S is such that S % n_i = a_i for each ID n_i, offset a_i

        // filter for non-zero bus IDs and get the set of offsets
        // note: `a` terms are NOT the offsets, they are the IDs with the
        // offsets subtracted out
        let (a, ids): (Vec<_>, Vec<_>) = self
            .bus_ids
            .iter()
            .enumerate()
            .filter(|(_, &bid)| bid > 0)
            // convert all terms to i64
            .map(|(offset, &id)| (id as i64 - offset as i64, id as i64))
            .unzip();

        // get the product N of all moduli (i.e. bus IDs)
        let N = ids.iter().product::<i64>();
        // get the N_i terms, the product of all moduli except n_i, for each i
        let N_i = ids.iter().map(|&n_i| N / n_i).collect::<Vec<_>>();

        // find the Bézout coefficients of the N_i and n_i terms
        // note: the solution only requires the first coefficient
        let M = N_i
            .iter()
            .zip(ids.iter())
            .map(|(&a, &b)| inverse(a, b))
            .collect::<Vec<_>>();

        // calculate the solution
        let X = (0..ids.len()).map(|i| a[i] * M[i] * N_i[i]).sum::<i64>();

        Ok((X % N).into())
    }
}
