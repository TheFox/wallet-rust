
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::ops::{Add, AddAssign};

pub type NumberType = f64;

#[derive(Debug, Clone)]
pub struct Number {
    n: NumberType,
}

impl Number {
    pub fn new() -> Self {
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

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            n: self.n + other.n,
        }
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            n: self.n + other.n,
        };
    }
}

impl ToDisplay for Number {
    fn to_display(self) -> NumberDisplay {
        NumberDisplay::new(self)
    }
}

#[derive(Debug)]
pub struct NumberDisplay {
    n: Number,
}

impl NumberDisplay {
    pub fn new(n: Number) -> Self {
        NumberDisplay {
            n,
        }
    }

    pub fn unwrap(&self) -> Number {
        println!("-> NumberDisplay::unwrap()");
        self.n.clone()
    }
}

impl Display for NumberDisplay {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        let n = self.n.unwrap();

        let width = if let Some(twidth) = f.width() {
            twidth
        } else {
            0
        };

        let precision = if let Some(tprecision) = f.precision() {
            tprecision
        } else {
            0
        };

        let empty = "";

        if width != 0 && precision != 0 {
            if n == 0.0 {
                write!(f, "{:>width$}", empty, width = width)?;
            }
            else {
                write!(f, "{1:width$.*}", precision, n, width = width)?;
            }
        } else if width != 0 && precision == 0 {
            if n == 0.0 {
                write!(f, "{0:>width$}", empty, width = width)?;
            } else {
                write!(f, "{0:width$}", n, width = width)?;
            }
        } else if width == 0 && precision != 0 {
            write!(f, "{1:.*}", precision, n)?;
        } else {
            write!(f, "NO_FORMAT")?;
        }

        Ok(())
    }
}

pub trait ToDisplay {
    fn to_display(self) -> NumberDisplay;
}

#[cfg(test)]
mod tests_basic {
    use super::Number;

    #[test]
    fn test_number_basic1() {
        let n1 = Number::from(1.23);
        assert_eq!(1.23, n1.unwrap());
    }
}

#[cfg(test)]
mod tests_add {
    use super::Number;
    // use std::ops::{Add, AddAssign};

    #[test]
    fn test_number_add1() {
        let n1 = Number::from(1.0);
        let n2 = Number::from(2.0);
        let n3 = n1 + n2;

        assert_eq!(3.0, n3.unwrap());
    }

    #[test]
    fn test_number_add2() {
        let mut n1 = Number::from(1.0);
        let n2 = Number::from(2.0);
        n1 += n2;

        assert_eq!(3.0, n1.unwrap());
    }
}

#[cfg(test)]
mod tests_display {
    use super::{Number, NumberDisplay, ToDisplay};

    #[test]
    fn test_number_display1() {
        let n1 = Number::from(1.23);
        let d1 = NumberDisplay::new(n1);
        println!("-> test_number_display1: {:?}", d1);
        let n2 = d1.unwrap();
        assert_eq!(1.23, n2.unwrap());
    }

    #[test]
    fn test_number_display2() {
        let n1 = Number::from(1.23);
        let d1 = n1.to_display();
        let n2 = d1.unwrap();
        assert_eq!(1.23, n2.unwrap());
    }

    #[test]
    fn test_number_display3() {
        let n1 = Number::new();
        let d1 = n1.to_display();
        assert_eq!("     ", format!("{:>5.2}", d1));
    }

    #[test]
    fn test_number_display4() {
        let n1 = Number::from(1.23);
        let d1 = n1.to_display();
        assert_eq!("  1.23", format!("{:>6.2}", d1));
    }
}
