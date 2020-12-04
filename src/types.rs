/*
** src/types.rs
*/

use std::collections::HashMap;
use std::convert::TryFrom;
use std::error;
use std::fmt;

#[derive(Debug)]
enum TypeParseErrorKind {
    PasswordPolicyError,
}

impl TypeParseErrorKind {
    fn type_name(&self) -> &'static str {
        match self {
            Self::PasswordPolicyError => "PasswordPolicy",
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

// there are 2 ways to interpret the x and y numbers in the password policy
// (1) range policy: password must contain the given character at least x and
//     at most y times
// (2) position policy: password must contain the given character at exactly
//     one of the positions x and y
pub enum PasswordPolicyRule {
    RangePolicy,
    PositionPolicy,
}

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
            Err(Self::parse_error(format!(
                "invalid character \"{}\"",
                s
            )))
        } else {
            Ok(s.chars().nth(0).unwrap())
        }
    }

    fn parse_number(s: &str) -> Result<u8, TypeParseError> {
        s.parse::<u8>().map_err(|_| Self::parse_error(format!("\"{}\" is not an integer", s)))
    }

    fn parse_x_y(s: &str) -> Result<(u8, u8), TypeParseError> {
        let parts = s.split('-').collect::<Vec<&str>>();
        match parts.as_slice() {
            &[xs, ys] => {
                let x = Self::parse_number(xs)?;
                let y = Self::parse_number(ys)?;
                Ok((x, y))
            }
            _ => Err(Self::parse_error(format!(
                "invalid range \"{}\"",
                s
            ))),
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
            },
            PasswordPolicyRule::PositionPolicy => {
                // note: passwords are NOT zero-indexed
                let x = policy.x - 1;
                let y = policy.y - 1;
                // maybe be less cavalier about unwrapping here?
                let cx = self.string.chars().nth(x as usize).unwrap();
                let cy = self.string.chars().nth(y as usize).unwrap();
                // xor == exactly 1 is equal
                (cx == policy.character) ^ (cy == policy.character)
            },
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

        Self { string: s, freq_map }
    }
}
