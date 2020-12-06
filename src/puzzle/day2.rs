/*
** src/puzzle/day2.rs
*/

use std::convert::TryFrom;

use crate::puzzle::*;
use crate::types::{Password, PasswordPolicy, PasswordPolicyRule};

const INPUT: &str = include_str!("../../input/2.input");

pub struct Day2 {
    password_db: Vec<(Password, PasswordPolicy)>,
}

impl Day2 {
    pub fn new() -> Result<Self> {
        // parse input into passwords and password policies
        let mut password_db = vec![];

        for line in INPUT.split('\n').filter(|s| !s.is_empty()) {
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

        Ok(Day2 { password_db })
    }
}

impl Puzzle for Day2 {
    // How many passwords are valid according to the (range-based) corporate
    // policies?
    fn part1(&self) -> Result<Solution> {
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
    fn part2(&self) -> Result<Solution> {
        // count the number of valid passwords, using the position policy
        let n_valid = self
            .password_db
            .iter()
            .filter(|(pwd, policy)| pwd.is_valid(policy, PasswordPolicyRule::PositionPolicy))
            .count();

        Ok((n_valid as u64).into())
    }
}
