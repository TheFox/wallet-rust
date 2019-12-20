
use chrono::{NaiveDate, Datelike};
use std::fmt::{Display, Formatter, Result as FmtRes, Debug};
use std::str::FromStr;
use std::num::ParseIntError;
// use std::num::IntErrorKind;
use regex::Regex;
use std::vec;

#[derive(Debug)]
pub enum DateError {
    InvalidDate,
}

#[derive(Debug, Clone)]
enum Parts {
    None,
    Day,
    Month,
    Year,
}

/// Sometimes we do not need all parts of a date.
///
/// For example, when you run `-d 2019-11`, to filter
/// all entries from Nov 2019. Or you want to auto-fill
/// the year and month by providing only the day.
pub struct Date {
    date: NaiveDate,
    used: u8, // 3 Bits: Y | M | D
}

impl Date {
    fn new() -> Self {
        println!("-> Date::new()");

        Date {
            date: NaiveDate::from_ymd(1970, 1, 1),
            used: 0, // 1 << 2 | 1 << 1 | 1
        }
    }

    fn has(&self, p: Parts) -> bool {
        let i = p as u8 - 1;
        // println!("-> Date::has({})", i);

        // Shift
        let s = 1 << i;

        self.used & s > 0
    }

    fn has_year(&self) -> bool {
        println!("-> Date::has_year() -> {}", self.has(Parts::Year));
        self.has(Parts::Year)
    }

    fn has_month(&self) -> bool {
        println!("-> Date::has_month()");
        self.has(Parts::Month)
    }

    fn has_day(&self) -> bool {
        println!("-> Date::has_day()");
        self.has(Parts::Day)
    }

    fn year(&self) -> i32 {
        println!("-> Date::year()");
        self.date.year()
    }

    fn month(&self) -> u32 {
        println!("-> Date::month()");
        self.date.month()
    }

    fn day(&self) -> u32 {
        println!("-> Date::day()");
        self.date.day()
    }
}

impl FromStr for Date {
    // type Err = ParseIntError;
    type Err = DateError;

    /// Available formats:
    ///
    /// - YYYY-MM-DD
    /// - YY-MM-DD
    /// - YYYY-MM
    /// - MM-DD
    /// - DD.MM.YYYY
    /// - DD.MM
    /// - MM/DD/YYYY
    /// - MM/DD
    /// - YYYY
    /// - DD
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("-> Date::from_str({})", s);

        let mut y: i32 = 0;
        let mut m: u32 = 0;
        let mut d: u32 = 0;

        // Break on first found.
        let patterns = vec![
            // The Good
            r"^(?P<y>\d{4})-(?P<m>\d{1,2})-(?P<d>\d{1,2})", // YYYY-MM-DD
            r"^(?P<y>\d{2})-(?P<m>\d{1,2})-(?P<d>\d{1,2})", // YY-MM-DD
            r"^(?P<y>\d{4})-(?P<m>\d{1,2})", // YYYY-MM
            r"^(?P<m>\d{1,2})-(?P<d>\d{1,2})", // MM-DD

            // The Bad
            r"^(?P<d>\d{1,2})\.(?P<m>\d{1,2})\.(?P<y>\d{4})", // DD.MM.YYYY
            r"^(?P<d>\d{1,2})\.(?P<m>\d{1,2})", // DD.MM

            // The Ugly
            r"^(?P<m>\d{1,2})/(?P<d>\d{1,2})/(?P<y>\d{2,4})", // MM/DD/YYYY
            r"^(?P<m>\d{1,2})/(?P<d>\d{1,2})", // MM/DD

            // The Default
            r"^(?P<y>\d{4})", // YYYY
            r"^(?P<d>\d{1,2})", // DD
        ];

        for pattern in patterns {
            let re = Regex::new(pattern).unwrap();
            // println!("-> pattern: {:?}", re);

            match re.captures(s) {
                Some(captures) => {
                    // println!("-> captures: {:?}", captures);

                    match captures.name("y") {
                        Some(t) => {
                            // println!("-> y: {:?}", t);
                            y = t.as_str().parse().expect("Parse field 'y' failed");
                        },
                        None => (),
                    }

                    match captures.name("m") {
                        Some(t) => {
                            // println!("-> m: {:?}", t);
                            m = t.as_str().parse().expect("Parse field 'm' failed");
                        },
                        None => (),
                    }

                    match captures.name("d") {
                        Some(t) => {
                            // println!("-> d: {:?}", t);
                            d = t.as_str().parse().expect("Parse field 'd' failed");
                        },
                        None => (),
                    }

                    break;
                },
                None => (),
            }
        } // for pattern in patterns

        let mut usage = 0;

        // Year Usage
        if y == 0 {
            y = 1970;
        } else {
            usage |= 1 << (Parts::Year as u8 - 1);
        }

        // Month Usage
        if m == 0 {
            m = 1;
        } else {
            usage |= 1 << (Parts::Month as u8 - 1);
        }

        // Day Usage
        if d == 0 {
            d = 1;
        } else {
            usage |= 1;
        }

        // println!("-> usage: {}", usage);
        // println!("-> ymd: {}-{}-{}", y, m, d);

        if usage == 0 {
            Err(DateError::InvalidDate)
            // Err(ParseIntError{ kind: std::num::IntErrorKind::Zero })
            // Self::Err(ParseIntError{})
            // Self::Err(ParseIntError{ kind: IntErrorKind::InvalidDigit })
        } else {
            let d = Self {
                date: NaiveDate::from_ymd(y, m, d),
                used: usage,
            };

            Ok(d)
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        let mut items: Vec<String> = vec![];

        if self.has_year() {
            items.push(self.date.format("%Y").to_string());
        }
        if self.has_month() {
            items.push(self.date.format("%m").to_string());
        }
        if self.has_day() {
            items.push(self.date.format("%d").to_string());
        }

        write!(f, "{}", items.join("-"))
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtRes {
        println!("-> Debug Date::fmt()");

        write!(f, "Date[{},Y={} M={} D={}]", self.date.format("%Y-%m-%d"),
            if self.has_year()  { "Y" } else { "N" },
            if self.has_month() { "Y" } else { "N" },
            if self.has_day()   { "Y" } else { "N" }
        )
    }
}

#[cfg(test)]
mod tests {
    // // use super::*;
    // use super::{Date, FromStr};
    use super::{Date, DateError};
    use std::str::FromStr;

    #[test]
    fn test_new_date() {
        Date::new();
    }

    #[test]
    fn test_from_ok1() {
        let d1 = Date::from_str("1987-02-21").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
    }

    #[test]
    fn test_from_ok2() {
        let d1 = Date::from_str("1987-02").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(1, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("1987-02", d1.to_string());
    }

    #[test]
    fn test_from_ok3() {
        let d1 = Date::from_str("1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(1, d1.month());
        assert_eq!(1, d1.day());
        assert!(d1.has_year());
        assert!(!d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("1987", d1.to_string());
    }

    #[test]
    fn test_from_ok4() {
        let d1 = Date::from_str("21.2.1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
    }

    #[test]
    fn test_from_ok5() {
        let d1 = Date::from_str("21.2").unwrap();
        assert_eq!(1970, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(!d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("02-21", d1.to_string());
    }

    #[test]
    fn test_from_ok6() {
        let d1 = Date::from_str("2/21/1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
    }

    #[test]
    fn test_from_ok7() {
        let d1 = Date::from_str("2/21").unwrap();
        assert_eq!(1970, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(!d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("02-21", d1.to_string());
    }

    // TODO
    #[test]
    fn test_from_bad1() {
        let d1 = Date::from_str("x");
        println!("-> from_bad1: {:?}", d1);

        // match d1 {
        //     Err(DateError::InvalidDate) => println!("-> Err(DateError::InvalidDate)"),
        //     // Err(DateError) => println!("-> Err(DateError)"),
        //     // Err(e) => println!("-> Err(e) -> {:?}", e),
        //     _ => println!("-> UNKNOWN"),
        // }
        // assert!(false);

        assert!(match d1 {
            Err(DateError::InvalidDate) => true,
            _ => false,
        });
    }

    #[test]
    fn test_display() {
        let d1 = Date::new();
        assert_eq!("", d1.to_string());
    }

    #[test]
    fn test_has() {
        let d1 = Date::new();
        assert!(!d1.has(super::Parts::Year));
    }

    #[test]
    fn test_year() {
        let d1 = Date::new();
        assert!(!d1.has_year());
    }

    #[test]
    fn test_month() {
        let d1 = Date::new();
        assert!(!d1.has_month());
    }

    #[test]
    fn test_day() {
        let d1 = Date::new();
        assert!(!d1.has_day());
    }
}
