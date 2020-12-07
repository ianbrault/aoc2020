/*
** src/puzzle/day2.rs
** https://adventofcode.com/2020/day/2
*/

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::puzzle::{self, Puzzle, PuzzleError, Solution};
use crate::types::{TypeParseError, TypeParseErrorKind};
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/2.input");

// there are 2 ways to interpret the x and y numbers in the password policy
// (1) range policy: password must contain the given character at least x and
//     at most y times
// (2) position policy: password must contain the given character at exactly
//     one of the positions x and y
enum PasswordPolicyRule {
    RangePolicy,
    PositionPolicy,
}

// defines the validity of a password
// see PasswordPolicyRule for specifics
#[derive(Debug)]
struct PasswordPolicy {
    character: char,
    x: u8,
    y: u8,
}

impl PasswordPolicy {
    fn parse_error<S>(s: S) -> TypeParseError
    where
        S: Into<String>,
    {
        TypeParseError::new(TypeParseErrorKind::PasswordPolicy, s)
    }

    fn parse_character(s: &str) -> Result<char, TypeParseError> {
        if s.chars().count() != 1 {
            Err(Self::parse_error(format!("invalid character \"{}\"", s)))
        } else {
            Ok(s.chars().next().unwrap())
        }
    }

    fn parse_number(s: &str) -> Result<u8, TypeParseError> {
        s.parse::<u8>()
            .map_err(|_| Self::parse_error(format!("\"{}\" is not an integer", s)))
    }

    fn parse_x_y(s: &str) -> Result<(u8, u8), TypeParseError> {
        // FIXME: add split_match! macro
        let parts = s.split('-').collect::<Vec<&str>>();
        match parts.as_slice() {
            [xs, ys] => {
                let x = Self::parse_number(xs)?;
                let y = Self::parse_number(ys)?;
                Ok((x, y))
            }
            _ => Err(Self::parse_error(format!("invalid range \"{}\"", s))),
        }
    }
}

impl TryFrom<&str> for PasswordPolicy {
    type Error = TypeParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // string should be in the format: <X>-<Y> <C>
        // FIXME: add split_match! macro
        let parts = s.split(' ').collect::<Vec<&str>>();
        match parts.as_slice() {
            [srange, schar] => {
                let character = Self::parse_character(schar)?;
                let (x, y) = Self::parse_x_y(srange)?;
                Ok(Self { character, x, y })
            }
            _ => Err(Self::parse_error(s)),
        }
    }
}

// a password
// also stores the frequency of each character in the password string for the
// range-based password policy
#[derive(Debug)]
struct Password<'a> {
    string: &'a str,
    freq_map: HashMap<char, u8>,
}

impl<'a> Password<'a> {
    fn count(&self, key: char) -> u8 {
        match self.freq_map.get(&key) {
            Some(&count) => count,
            None => 0,
        }
    }

    fn is_valid(&self, policy: &PasswordPolicy, policy_rule: PasswordPolicyRule) -> bool {
        match policy_rule {
            PasswordPolicyRule::RangePolicy => {
                let range = (policy.x)..(policy.y + 1);
                range.contains(&self.count(policy.character))
            }
            PasswordPolicyRule::PositionPolicy => {
                // note: passwords are NOT zero-indexed
                let x = policy.x - 1;
                let y = policy.y - 1;
                // maybe be less cavalier about unwrapping here?
                let cx = self.string.chars().nth(x as usize).unwrap();
                let cy = self.string.chars().nth(y as usize).unwrap();
                // xor == exactly 1 is equal
                (cx == policy.character) ^ (cy == policy.character)
            }
        }
    }
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(s: &'a str) -> Self {
        // pre-allocate freq_map using the length of the string, for worst case
        let mut freq_map = HashMap::with_capacity(s.len());

        for c in s.chars() {
            let item = freq_map.entry(c).or_insert(0);
            *item += 1;
        }

        Self {
            string: s,
            freq_map,
        }
    }
}

pub struct Day2 {
    password_db: Vec<(Password<'static>, PasswordPolicy)>,
}

impl Day2 {
    pub fn new() -> puzzle::Result<Self> {
        // parse input into passwords and password policies
        let mut password_db = vec![];

        for line in input_to_lines(INPUT) {
            // FIXME: add split_match! macro
            let parts = line.split(": ").collect::<Vec<&str>>();
            let entry = match parts.as_slice() {
                &[spolicy, spass] => {
                    let password = Password::from(spass);
                    let policy = PasswordPolicy::try_from(spolicy)?;
                    Ok((password, policy))
                }
                _ => Err(PuzzleError::InvalidInput(line.into())),
            }?;

            password_db.push(entry);
        }

        Ok(Self { password_db })
    }
}

impl Puzzle for Day2 {
    // How many passwords are valid according to the (range-based) corporate
    // policies?
    fn part1(&self) -> puzzle::Result<Solution> {
        // count the number of valid passwords, using the range policy
        let n_valid = self
            .password_db
            .iter()
            .filter(|(pwd, policy)| pwd.is_valid(policy, PasswordPolicyRule::RangePolicy))
            .count();

        Ok((n_valid as u64).into())
    }

    // How many passwords are valid according to the new (position-based)
    // interpretation of the policies?
    fn part2(&self) -> puzzle::Result<Solution> {
        // count the number of valid passwords, using the position policy
        let n_valid = self
            .password_db
            .iter()
            .filter(|(pwd, policy)| pwd.is_valid(policy, PasswordPolicyRule::PositionPolicy))
            .count();

        Ok((n_valid as u64).into())
    }
}
