
use std::fmt::{Display, Formatter, Result as FmtRes};

pub type NumberType = f64;

#[derive(Debug, Clone)]
pub struct Number {
    pub n: NumberType,
}

impl Number {
    pub fn new() -> Self {
        println!("-> Number::new()");

        Number {
            n: 0.0,
        }
    }

    pub fn from(n: NumberType) -> Self {
        Number {
            n,
        }
    }

    pub fn unwrap(&self) -> NumberType {
        self.n
    }
}

impl ToDisplay for Number {
    fn to_display(self) -> NumberDisplay {
        NumberDisplay::new(self)
    }
}

pub struct NumberDisplay {
    n: Number,
}

impl NumberDisplay {
    pub fn new(n: Number) -> Self {
        NumberDisplay {
            n,
        }
    }
}

impl Display for NumberDisplay {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        if self.n.n != 0.0 {
            write!(f, "{:>10.2}", self.n.n)
        } else {
            write!(f, "{:>10}", "")
        }
    }
}

pub trait ToDisplay {
    fn to_display(self) -> NumberDisplay;
}
