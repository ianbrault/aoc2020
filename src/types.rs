/*
** src/types.rs
*/

use std::collections::{hash_map::IntoIter, HashMap};
use std::error;
use std::fmt;
use std::hash::Hash;

#[derive(Debug)]
pub enum TypeParseErrorKind {
    Passport,
}

impl TypeParseErrorKind {
    fn type_name(&self) -> &'static str {
        match self {
            Self::Passport => "Passport",
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
    where
        S: Into<String>,
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

pub struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    pub fn get(&self, key: &T) -> usize {
        match self.counts.get(key) {
            Some(&count) => count,
            None => 0,
        }
    }

    pub fn extend<Iter>(&mut self, iter: Iter)
    where
        Iter: Iterator<Item = T>,
    {
        for item in iter {
            let entry = self.counts.entry(item).or_insert(0);
            *entry += 1;
        }
    }
}

impl<T> IntoIterator for Counter<T> {
    type Item = (T, usize);
    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.into_iter()
    }
}

impl<Iter, T> From<Iter> for Counter<T>
where
    Iter: Iterator<Item = T>,
    T: Eq + Hash,
{
    fn from(iter: Iter) -> Self {
        let mut counter = Self::new();
        counter.extend(iter);
        counter
    }
}
