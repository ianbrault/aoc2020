/*
** src/puzzle/day2.rs
** https://adventofcode.com/2020/day/2
*/

use crate::puzzle::{self, Puzzle, Solution};
use crate::types::Counter;
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
struct PasswordPolicy {
    character: char,
    x: u8,
    y: u8,
}

impl PasswordPolicy {
    fn parse_character(s: &str) -> char {
        s.chars().next().unwrap()
    }

    fn parse_number(s: &str) -> u8 {
        s.parse().unwrap()
    }

    fn parse_x_y(s: &str) -> (u8, u8) {
        match split!(s, '-') {
            [xs, ys] => {
                let x = Self::parse_number(xs);
                let y = Self::parse_number(ys);
                (x, y)
            }
            _ => unreachable!(),
        }
    }
}

impl From<&str> for PasswordPolicy {
    fn from(s: &str) -> Self {
        // string should be in the format: <X>-<Y> <C>
        match split!(s, ' ') {
            [srange, schar] => {
                let character = Self::parse_character(schar);
                let (x, y) = Self::parse_x_y(srange);

                Self { character, x, y }
            }
            _ => unreachable!(),
        }
    }
}

// a password
// also stores the frequency of each character in the password string for the
// range-based password policy
struct Password<'a> {
    string: &'a str,
    freq_map: Counter<char>,
}

impl<'a> Password<'a> {
    fn is_valid(&self, policy: &PasswordPolicy, policy_rule: PasswordPolicyRule) -> bool {
        match policy_rule {
            PasswordPolicyRule::RangePolicy => {
                let range = (policy.x)..(policy.y + 1);
                range.contains(&(self.freq_map.get(&policy.character) as u8))
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
    fn from(string: &'a str) -> Self {
        let freq_map = Counter::from(string.chars());
        Self { string, freq_map }
    }
}

pub struct Day2 {
    password_db: Vec<(Password<'static>, PasswordPolicy)>,
}

impl Day2 {
    pub fn new() -> Self {
        // parse input into passwords and password policies
        let mut password_db = vec![];

        for line in input_to_lines(INPUT) {
            let entry = match split!(line, ": ") {
                [spolicy, spass] => {
                    let password = Password::from(*spass);
                    let policy = PasswordPolicy::from(*spolicy);
                    (password, policy)
                }
                _ => unreachable!(),
            };

            password_db.push(entry);
        }

        Self { password_db }
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

        Ok(n_valid.into())
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

        Ok(n_valid.into())
    }
}
