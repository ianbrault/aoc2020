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
    PasswordPolicyError,
    TreeMapError,
}

impl TypeParseErrorKind {
    fn type_name(&self) -> &'static str {
        match self {
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
where I: Iterator<Item=bool>
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
        // FIXME: add a util wrapper for the following pattern
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
        Self { tree_map, dy, dx, pos: (0, 0) }
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
