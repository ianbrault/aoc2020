/*
** src/puzzle/day4.rs
** https://adventofcode.com/2020/day/4
*/

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::puzzle::{self, Puzzle, Solution};
use crate::types::{TypeParseError, TypeParseErrorKind};

const INPUT: &str = include_str!("../../input/4.input");

// passport height
pub enum Height {
    Centimeters(u8),
    Inches(u8),
}

impl TryFrom<&str> for Height {
    type Error = TypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err = Passport::parse_error;

        // find the unit
        let i = value
            .find(|c: char| !c.is_digit(10))
            .ok_or_else(|| err(format!("height \"{}\" missing unit", value)))?;
        // just parse into a large integer, we can bounds check the u8 later
        let n = value[0..i].parse::<u64>().unwrap();

        let unit = &value[i..value.len()];
        match unit {
            "cm" => {
                if n >= 150 && n <= 193 {
                    Ok(Self::Centimeters(n as u8))
                } else {
                    Err(err(format!(
                        "invalid centimeters value {}, must be 150-193cm",
                        n
                    )))
                }
            }
            "in" => {
                if n >= 59 && n <= 76 {
                    Ok(Self::Inches(n as u8))
                } else {
                    Err(err(format!("invalid inches value {}, must be 59-76in", n)))
                }
            }
            _ => Err(err(format!("invalid height unit \"{}\"", unit))),
        }
    }
}

// passport eye color
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Gray,
    Green,
    Hazel,
    Other,
}

impl TryFrom<&str> for EyeColor {
    type Error = TypeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Gray),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(Passport::parse_error(format!(
                "invalid eye color \"{}\"",
                value
            ))),
        }
    }
}

// passports have the following fields:
// byr: birth year
// iyr: issue year
// eyr: expiration year
// hgt: height
// hcl: hair color
// ecl: eye color
// pid: passport ID
// cid: country ID (optional)
// TODO: remove dead_code suppressions
pub struct Passport {
    #[allow(dead_code)]
    byr: u16,
    #[allow(dead_code)]
    iyr: u16,
    #[allow(dead_code)]
    eyr: u16,
    #[allow(dead_code)]
    hgt: Height,
    #[allow(dead_code)]
    hcl: &'static str,
    #[allow(dead_code)]
    ecl: EyeColor,
    #[allow(dead_code)]
    pid: u32,
    #[allow(dead_code)]
    cid: Option<&'static str>,
}

impl Passport {
    fn parse_error<S>(s: S) -> TypeParseError
    where
        S: Into<String>,
    {
        TypeParseError::new(TypeParseErrorKind::Passport, s)
    }

    // checks if a passport entry from a batch file has all required fields
    pub fn has_fields(batch: &str) -> bool {
        // note: excluding the optional cid key
        let mut keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|&k| (k, false))
            .collect::<HashMap<&str, bool>>();

        for entry in batch.split_whitespace().filter(|s| !s.is_empty()) {
            let key = entry.split(':').next().unwrap();
            if keys.contains_key(&key) {
                keys.insert(key, true);
            }
        }

        keys.into_iter().filter(|(_, v)| !v).count() == 0
    }

    fn parse_year(s: &str, min: u16, max: u16) -> Result<u16, TypeParseError> {
        let year = s
            .parse()
            .map_err(|_| Self::parse_error(format!("invalid year \"{}\"", s)))?;

        if year >= min && year <= max {
            Ok(year)
        } else {
            Err(Self::parse_error(format!("invalid year \"{}\"", s)))
        }
    }

    fn parse_hex(s: &str) -> Result<&str, TypeParseError> {
        let err = Self::parse_error(format!("invalid color \"{}\"", s));

        if !s.starts_with('#') {
            return Err(err);
        }

        let non_hex_digits = s[1..s.len()].chars().filter(|c| !c.is_digit(16)).count();
        if non_hex_digits != 0 {
            Err(err)
        } else {
            Ok(s)
        }
    }

    fn parse_pid(s: &str) -> Result<u32, TypeParseError> {
        if s.len() != 9 {
            Err(Self::parse_error(format!(
                "passport ID \"{}\" must be 9 characters",
                s
            )))
        } else {
            s.parse()
                .map_err(|_| Self::parse_error(format!("invalid passport ID \"{}\"", s)))
        }
    }
}

impl TryFrom<PassportBuilder> for Passport {
    type Error = TypeParseError;

    fn try_from(builder: PassportBuilder) -> Result<Self, Self::Error> {
        let err = |f| Self::parse_error(format!("missing field {}", f));
        Ok(Self {
            byr: builder.byr.ok_or_else(|| err("byr"))?,
            iyr: builder.iyr.ok_or_else(|| err("iyr"))?,
            eyr: builder.eyr.ok_or_else(|| err("eyr"))?,
            hgt: builder.hgt.ok_or_else(|| err("hgt"))?,
            hcl: builder.hcl.ok_or_else(|| err("hcl"))?,
            ecl: builder.ecl.ok_or_else(|| err("ecl"))?,
            pid: builder.pid.ok_or_else(|| err("pid"))?,
            cid: builder.cid,
        })
    }
}

impl TryFrom<&'static str> for Passport {
    type Error = TypeParseError;

    fn try_from(batch: &'static str) -> Result<Self, Self::Error> {
        let mut builder = PassportBuilder::default();

        for entry in batch.split_whitespace().filter(|s| !s.is_empty()) {
            split_into!(entry, ':', key, value);
            builder.set(key, value)?;
        }

        Self::try_from(builder)
    }
}

// used to construct passports one field at a time
#[derive(Default)]
struct PassportBuilder {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Height>,
    hcl: Option<&'static str>,
    ecl: Option<EyeColor>,
    pid: Option<u32>,
    cid: Option<&'static str>,
}

impl PassportBuilder {
    fn set(&mut self, key: &str, value: &'static str) -> Result<(), TypeParseError> {
        match key {
            "byr" => {
                let year = Passport::parse_year(value, 1920, 2002)?;
                self.byr = Some(year);
            }
            "iyr" => {
                let year = Passport::parse_year(value, 2010, 2020)?;
                self.iyr = Some(year);
            }
            "eyr" => {
                let year = Passport::parse_year(value, 2020, 2030)?;
                self.eyr = Some(year);
            }
            "hgt" => {
                let height = Height::try_from(value)?;
                self.hgt = Some(height);
            }
            "hcl" => {
                let color = Passport::parse_hex(value)?;
                self.hcl = Some(color);
            }
            "ecl" => {
                let eye_color = EyeColor::try_from(value)?;
                self.ecl = Some(eye_color);
            }
            "pid" => {
                let pid = Passport::parse_pid(value)?;
                self.pid = Some(pid);
            }
            "cid" => {
                self.cid = Some(value);
            }
            _ => unreachable!(),
        };

        Ok(())
    }
}

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day4 {
    // In your batch file, how many passports are valid?
    // note: does not include field validation
    fn part1(&self) -> puzzle::Result<Solution> {
        let n_valid = INPUT
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .map(|batch| Passport::has_fields(batch))
            .filter(|&b| b)
            .count();

        Ok((n_valid as u64).into())
    }

    // In your batch file, how many passports are valid?
    // note: includes field validation
    fn part2(&self) -> puzzle::Result<Solution> {
        let mut passports = vec![];

        // parse passports from the fields in the batch file
        for batch in INPUT.split("\n\n").filter(|s| !s.is_empty()) {
            if let Ok(passport) = Passport::try_from(batch) {
                passports.push(passport);
            }
        }

        Ok((passports.len() as u64).into())
    }
}
