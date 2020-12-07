/*
** src/types.rs
*/

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum TypeParseErrorKind {
    Passport,
    PasswordPolicy,
    TreeMap,
}

impl TypeParseErrorKind {
    fn type_name(&self) -> &'static str {
        match self {
            Self::Passport => "Passport",
            Self::PasswordPolicy => "PasswordPolicy",
            Self::TreeMap => "TreeMap",
        }
    }
}

#[derive(Debug)]
pub struct TypeParseError {
    kind: TypeParseErrorKind,
    reason: String,
}

impl TypeParseError {
    pub fn new<S>(kind: TypeParseErrorKind, reason: S) -> Self
    where S: Into<String>
    {
        Self {
            kind,
            reason: reason.into(),
        }
    }
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
    pub fn at(&self, index: usize) -> bool {
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
