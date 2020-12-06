/*
** src/types.rs
*/

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::fmt;

/*
** error type for parsing any of the below types
*/

#[derive(Debug)]
enum TypeParseErrorKind {
    PassportError,
    PasswordPolicyError,
    TreeMapError,
}

impl TypeParseErrorKind {
    fn type_name(&self) -> &'static str {
        match self {
            Self::PassportError => "Passport",
            Self::PasswordPolicyError => "PasswordPolicy",
            Self::TreeMapError => "TreeMap",
        }
    }
}

#[derive(Debug)]
pub struct TypeParseError {
    kind: TypeParseErrorKind,
    reason: String,
}

impl fmt::Display for TypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to parse {}: {}",
            self.kind.type_name(),
            self.reason
        )
    }
}

impl error::Error for TypeParseError {}

/*
** types
*/

pub struct Bitfield {
    data: u32,
}

impl Bitfield {
    fn at(&self, index: usize) -> bool {
        if index >= 32 {
            false
        } else {
            (self.data & (1 << index)) != 0
        }
    }
}

// build a bitfield from an iterator of booleans
// important: the iterator is treated as going from the least-significant to
// most-significant bit in the bitfield
impl<I> From<I> for Bitfield
where
    I: Iterator<Item = bool>,
{
    fn from(it: I) -> Self {
        let mut data = 0;

        for (index, _) in it.enumerate().filter(|(_, x)| *x) {
            data |= 1 << index;
        }

        Self { data }
    }
}

// there are 2 ways to interpret the x and y numbers in the password policy
// (1) range policy: password must contain the given character at least x and
//     at most y times
// (2) position policy: password must contain the given character at exactly
//     one of the positions x and y
pub enum PasswordPolicyRule {
    RangePolicy,
    PositionPolicy,
}

// defines the validity of a password
// see PasswordPolicyRule for specifics
#[derive(Debug)]
pub struct PasswordPolicy {
    character: char,
    x: u8,
    y: u8,
}

impl PasswordPolicy {
    fn parse_error<S>(s: S) -> TypeParseError
    where
        S: Into<String>,
    {
        TypeParseError {
            kind: TypeParseErrorKind::PasswordPolicyError,
            reason: s.into(),
        }
    }

    fn parse_character(s: &str) -> Result<char, TypeParseError> {
        if s.chars().count() != 1 {
            Err(Self::parse_error(format!("invalid character \"{}\"", s)))
        } else {
            Ok(s.chars().nth(0).unwrap())
        }
    }

    fn parse_number(s: &str) -> Result<u8, TypeParseError> {
        s.parse::<u8>()
            .map_err(|_| Self::parse_error(format!("\"{}\" is not an integer", s)))
    }

    fn parse_x_y(s: &str) -> Result<(u8, u8), TypeParseError> {
        let parts = s.split('-').collect::<Vec<&str>>();
        match parts.as_slice() {
            &[xs, ys] => {
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
        let parts = s.split(' ').collect::<Vec<&str>>();
        match parts.as_slice() {
            &[srange, schar] => {
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
pub struct Password {
    string: &'static str,
    freq_map: HashMap<char, u8>,
}

impl Password {
    pub fn count(&self, key: char) -> u8 {
        match self.freq_map.get(&key) {
            Some(&count) => count,
            None => 0,
        }
    }

    pub fn is_valid(&self, policy: &PasswordPolicy, policy_rule: PasswordPolicyRule) -> bool {
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

impl From<&'static str> for Password {
    fn from(s: &'static str) -> Self {
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

// terrain map which indicates the locations of trees
pub struct TreeMap {
    // each row is stored as a bitfield, where a bit is set if there is a tree
    map: Vec<Bitfield>,
    pub width: usize,
    pub height: usize,
}

impl TreeMap {
    pub fn at(&self, x: usize, y: usize) -> bool {
        if y >= self.height {
            false
        } else {
            self.map[y].at(x % self.width)
        }
    }

    pub fn traverse(&self, dy: u8, dx: u8) -> TreeMapTraverser {
        TreeMapTraverser::new(self, dy, dx)
    }

    fn parse_error<S>(s: S) -> TypeParseError
    where
        S: Into<String>,
    {
        TypeParseError {
            kind: TypeParseErrorKind::TreeMapError,
            reason: s.into(),
        }
    }

    fn parse_map_row(s: &str) -> Result<Bitfield, TypeParseError> {
        if s.len() > 32 {
            Err(Self::parse_error("map row is too long"))
        } else {
            let it = s.chars().map(|c| c == '#');
            Ok(Bitfield::from(it.into_iter()))
        }
    }
}

impl TryFrom<&str> for TreeMap {
    type Error = TypeParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut map = vec![];

        // get the width of the first line
        let width = s.split('\n').nth(0).map_or(0, |ss| ss.len());

        for line in s.split('\n').filter(|ss| !ss.is_empty()) {
            map.push(Self::parse_map_row(line)?);
        }

        let height = map.len();
        Ok(Self { map, width, height })
    }
}

// used to traverse a TreeMap at a given slope, as an iterator
pub struct TreeMapTraverser<'a> {
    tree_map: &'a TreeMap,
    dy: u8,
    dx: u8,
    pos: (usize, usize),
}

impl<'a> TreeMapTraverser<'a> {
    fn new(tree_map: &'a TreeMap, dy: u8, dx: u8) -> Self {
        Self {
            tree_map,
            dy,
            dx,
            pos: (0, 0),
        }
    }
}

impl<'a> Iterator for TreeMapTraverser<'a> {
    // each iteration returns whether or not there is a tree at the new position
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.pos;
        x += self.dx as usize;
        y += self.dy as usize;

        let res = if y >= self.tree_map.height {
            // reached the bottom, done iterating
            None
        } else {
            Some(self.tree_map.at(x, y))
        };

        self.pos = (x, y);
        res
    }
}

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
pub struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: Height,
    hcl: &'static str,
    ecl: EyeColor,
    pid: u32,
    cid: Option<&'static str>,
}

impl Passport {
    fn parse_error<S>(s: S) -> TypeParseError
    where
        S: Into<String>,
    {
        TypeParseError {
            kind: TypeParseErrorKind::TreeMapError,
            reason: s.into(),
        }
    }

    // checks if a passport entry from a batch file has all required fields
    pub fn has_fields(batch: &str) -> bool {
        // note: excluding the optional cid key
        let mut keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|&k| (k, false))
            .collect::<HashMap<&str, bool>>();

        for entry in batch.split_whitespace().filter(|s| !s.is_empty()) {
            let key = entry.split(':').nth(0).unwrap();
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
            let (key, value) = match entry.split(':').collect::<Vec<&str>>().as_slice() {
                &[k, v] => (k, v),
                _ => unreachable!(),
            };
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
