
use std::fmt::{Display, Formatter, Result as FmtRes};

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
        if self.s.len() > self.max_len && self.max_len >= 3 {
            let s = &self.s[..(self.max_len - 3)];
            write!(f, "{}...", s)
        } else {
            write!(f, "{}", self.s)
        }
    }
}

pub trait ToShortString {
    fn to_short_string(self, max_len: usize) -> ShortString;
}

#[cfg(test)]
mod tests_basic {
    use super::ShortString;

    #[test]
    fn test_short_string1() {
        ShortString::new();
    }

    #[test]
    fn test_short_string2() {
        let s1 = ShortString::from("ABCDEFGH".to_string(), 8);
        assert_eq!("ABCDEFGH", format!("{}", s1));
    }

    #[test]
    fn test_short_string3() {
        let s1 = ShortString::from("ABCDEFGH".to_string(), 5);
        assert_eq!("AB...", format!("{}", s1));

        let s1 = ShortString::from("ABCDEFGH".to_string(), 4);
        assert_eq!("A...", format!("{}", s1));

        let s1 = ShortString::from("ABCDEFGH".to_string(), 3);
        assert_eq!("...", format!("{}", s1));

        let s1 = ShortString::from("ABCDEFGH".to_string(), 2);
        assert_eq!("ABCDEFGH", format!("{}", s1));
    }
}
