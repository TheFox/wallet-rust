
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::string::ToString;
use crate::types::Number;

pub struct ShortString {
    s: String,
    max_len: usize,
}

impl ShortString {
    pub fn new() -> Self {
        ShortString {
            s: String::new(),
            max_len: 0,
        }
    }

    pub fn from(s: String, max_len: usize) -> Self {
        ShortString {
            s,
            max_len,
        }
    }
}

impl Display for ShortString {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        if self.s.len() > self.max_len {
            let s = &self.s[..(self.max_len - 3)];
            write!(f, "{}...", s)
        } else {
            write!(f, "{}", self.s)
        }
    }
}

/// If it's 0.0 it will be converted to "".
pub struct ZeroString {
    n: Number,
}

impl ZeroString {
    pub fn new() -> Self {
        ZeroString {
            n: 0.0,
        }
    }

    // TODO: use real from trait
    pub fn from(n: Number) -> Self {
        ZeroString {
            n,
        }
    }
}

impl Display for ZeroString {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        if self.n != 0.0 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "")
        }
    }
}

// TODO tests
