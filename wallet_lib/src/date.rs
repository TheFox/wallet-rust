
use chrono::{NaiveDate, Datelike};
use std::fmt::{Display, Formatter, Result as FmtRes, Debug};
use std::str::FromStr;
use regex::Regex;
use std::vec;
use std::convert::From;

#[derive(Debug)]
pub enum DateError {
    InvalidDate,
}

#[derive(Debug, Clone)]
enum Parts {
    Day = 1,
    Month = 2,
    Year = 3,
}

/// Sometimes we do not need all parts of a date.
///
/// For example, when you run `-d 2019-11`, to filter
/// all entries from Nov 2019. Or you want to auto-fill
/// the year and month by providing only the day.
#[derive(Copy, Clone)]
pub struct Date {
    date: NaiveDate,
    used: u8, // 3 Bits: Y | M | D
}

impl Date {
    pub fn new() -> Self {
        Self {
            date: NaiveDate::from_ymd(1970, 1, 1),
            used: 0, // 1 << 2 | 1 << 1 | 1
        }
    }

    fn has(&self, p: Parts) -> bool {
        let i = p as u8 - 1;

        // Shift
        let s = 1 << i;

        self.used & s > 0
    }

    pub fn has_year(&self) -> bool {
        self.has(Parts::Year)
    }

    pub fn has_month(&self) -> bool {
        self.has(Parts::Month)
    }

    pub fn has_day(&self) -> bool {
        self.has(Parts::Day)
    }

    pub fn year(&self) -> i32 {
        self.date.year()
    }

    /// Set new Year.
    pub fn set_year(&mut self, y: i32) {
        self.date = self.date.with_year(y).expect("Invalid year");
        self.used |= 1 << (Parts::Year as u8 - 1);
    }

    pub fn raw_set_year(&mut self, y: i32) {
        self.date = self.date.with_year(y).expect("Invalid year");
    }

    pub fn month(&self) -> u32 {
        self.date.month()
    }

    /// Formatted Month
    pub fn fmonth(&self) -> String {
        self.date.format("%m").to_string()
    }

    /// Set new Month.
    pub fn set_month(&mut self, m: u32) {
        self.date = self.date.with_month(m).expect("Invalid month");
        self.used |= 1 << (Parts::Month as u8 - 1);
    }

    pub fn raw_set_month(&mut self, m: u32) {
        self.date = self.date.with_month(m).expect("Invalid month");
    }

    pub fn day(&self) -> u32 {
        self.date.day()
    }

    /// Set new Day.
    pub fn set_day(&mut self, d: u32) {
        self.date = self.date.with_day(d).expect("Invalid day");
        self.used |= 1;
    }

    pub fn raw_set_day(&mut self, d: u32) {
        self.date = self.date.with_day(d).expect("Invalid day");
    }

    /// Year-Month
    pub fn ym(&self) -> String {
        let mut items: Vec<String> = vec![];

        if self.has_year() {
            items.push(self.date.format("%Y").to_string());
        }
        if self.has_month() {
            items.push(self.date.format("%m").to_string());
        }

        items.join("-")
    }

    /// Formatted Year-Month
    pub fn fym(&self, f: &str) -> String {
        let mut items: Vec<String> = vec![];

        if self.has_year() {
            items.push(self.date.format("%Y").to_string());
        }
        if self.has_month() {
            items.push(self.date.format("%m").to_string());
        }

        items.join(f)
    }

    // Raw Year_Month
    pub fn rym(&self) -> String {
        let mut items: Vec<String> = vec![];

        items.push(self.date.format("%Y").to_string());
        items.push(self.date.format("%m").to_string());

        items.join("_")
    }

    /// Year-Month-Day
    pub fn ymd(&self) -> String {
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

        items.join("-")
    }
}

impl FromStr for Date {
    // type Err = ParseIntError;
    type Err = DateError;

    /// Available formats:
    ///
    /// - YYYY-MM-DD
    /// - YYYY/MM/DD
    /// - YY-MM-DD
    /// - YY/MM/DD
    /// - YYYY-MM
    /// - MM-DD
    /// - DD.MM.YYYY
    /// - DD.MM
    /// - MM/DD/YYYY
    /// - MM/DD
    /// - YYYY
    /// - DD
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("-> Date::from_str({})", s);

        let mut y: i32 = 0;
        let mut m: u32 = 0;
        let mut d: u32 = 0;

        // Break on first found.
        let patterns = vec![
            // The Good
            r"^(?P<y>\d{4})-(?P<m>\d{1,2})-(?P<d>\d{1,2})", // YYYY-MM-DD
            r"^(?P<y>\d{4})/(?P<m>\d{1,2})/(?P<d>\d{1,2})", // YYYY/MM/DD
            r"^(?P<y>\d{2})-(?P<m>\d{1,2})-(?P<d>\d{1,2})", // YY-MM-DD
            r"^(?P<y>\d{2})/(?P<m>\d{1,2})/(?P<d>\d{1,2})", // YY/MM/DD
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

                    if let Some(t) = captures.name("y") {
                        y = t.as_str().parse().expect("Parse field 'y' failed");
                    }
                    if let Some(t) = captures.name("m") {
                        m = t.as_str().parse().expect("Parse field 'm' failed");
                    }
                    if let Some(t) = captures.name("d") {
                        d = t.as_str().parse().expect("Parse field 'd' failed");
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

            if y < 100 {
                y += 2000;
            }
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
        } else {
            let d = Self {
                date: NaiveDate::from_ymd(y, m, d),
                used: usage,
            };

            Ok(d)
        }
    }
}

impl From<String> for Date {
    fn from(s: String) -> Date {
        Date::from_str(&s).unwrap()
    }
}

impl From<&str> for Date {
    fn from(s: &str) -> Date {
        Date::from_str(s).unwrap()
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
        write!(f, "Date[{}, Y={} M={} D={}]", self.date.format("%Y-%m-%d"),
            if self.has_year()  { "Y" } else { "N" },
            if self.has_month() { "Y" } else { "N" },
            if self.has_day()   { "Y" } else { "N" }
        )
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.date.year() == other.date.year() &&
        self.date.month() == other.date.month() &&
        self.date.day() == other.date.day()
    }
}

#[cfg(test)]
mod tests_basic {
    use super::Date;

    #[test]
    fn test_new_date() {
        Date::new();
    }

    #[test]
    fn test_date_to_string() {
        let d1 = Date::new();
        assert_eq!("", d1.to_string());
    }

    #[test]
    fn test_date_has() {
        let d1 = Date::new();
        assert!(!d1.has(super::Parts::Year));
    }

    #[test]
    fn test_date_set() {
        let mut d1 = Date::new();

        d1.set_year(2019);
        assert!(d1.has_year());
        assert!(!d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("2019", d1.to_string());

        d1.set_month(12);
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("2019-12", d1.to_string());

        d1.set_day(31);
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("2019-12-31", d1.to_string());
    }
}

#[cfg(test)]
mod tests_eq {
    use super::Date;

    #[test]
    fn test_date_eq_ymd() {
        let d1 = Date::from("1987-02-21".to_string());
        let d2 = Date::from("1987-02-21".to_string());
        assert!(d1 == d2);
    }

    #[test]
    fn test_date_eq_ym() {
        let d1 = Date::from("1987-02".to_string());
        let d2 = Date::from("1987-02".to_string());
        assert!(d1 == d2);
    }

    #[test]
    fn test_date_eq_y() {
        let d1 = Date::from("1987".to_string());
        let d2 = Date::from("1987".to_string());
        assert!(d1 == d2);
    }

    #[test]
    fn test_date_neq_ymd() {
        let d1 = Date::from("1987-02-21".to_string());
        let d2 = Date::from("1987-02-22".to_string());
        assert!(d1 != d2);
    }

    #[test]
    fn test_date_neq_ym() {
        let d1 = Date::from("1987-02".to_string());
        let d2 = Date::from("1987-03".to_string());
        assert!(d1 != d2);
    }

    #[test]
    fn test_date_neq_y() {
        let d1 = Date::from("1987".to_string());
        let d2 = Date::from("1988".to_string());
        assert!(d1 != d2);
    }
}

#[cfg(test)]
mod tests_from_str {
    use super::{Date, DateError};
    use std::str::FromStr;

    #[test]
    fn test_date_from_str_ok1() {
        let d1 = Date::from_str("1987-02-21").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
        assert_eq!("1987-02", d1.ym());
        assert_eq!("1987_02", d1.fym("_"));
    }

    #[test]
    fn test_date_from_str_ok2() {
        let d1 = Date::from_str("1987-02").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(1, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("1987-02", d1.to_string());
        assert_eq!("1987-02", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok3() {
        let d1 = Date::from_str("19-2-23").unwrap();
        assert_eq!(2019, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(23, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("2019-02-23", d1.to_string());
        assert_eq!("2019-02-23", d1.ymd());
    }

    #[test]
    fn test_date_from_str_ok4() {
        let d1 = Date::from_str("1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(1, d1.month());
        assert_eq!(1, d1.day());
        assert!(d1.has_year());
        assert!(!d1.has_month());
        assert!(!d1.has_day());
        assert_eq!("1987", d1.to_string());
        assert_eq!("1987", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok5() {
        let d1 = Date::from_str("21.2.1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
        assert_eq!("1987-02", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok6() {
        let d1 = Date::from_str("21.2").unwrap();
        assert_eq!(1970, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(!d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("02-21", d1.to_string());
        assert_eq!("02", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok7() {
        let d1 = Date::from_str("2/21/1987").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
        assert_eq!("1987-02", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok8() {
        let d1 = Date::from_str("2/21").unwrap();
        assert_eq!(1970, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(!d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("02-21", d1.to_string());
        assert_eq!("02", d1.ym());
    }

    #[test]
    fn test_date_from_str_ok9() {
        let d1 = Date::from_str("1987/2/21").unwrap();
        assert_eq!(1987, d1.year());
        assert_eq!(2, d1.month());
        assert_eq!(21, d1.day());
        assert!(d1.has_year());
        assert!(d1.has_month());
        assert!(d1.has_day());
        assert_eq!("1987-02-21", d1.to_string());
        assert_eq!("1987-02", d1.ym());
    }

    #[test]
    fn test_date_from_str_bad1() {
        let d1 = Date::from_str("x");
        println!("-> from_bad1: {:?}", d1);

        assert!(match d1 {
            Err(DateError::InvalidDate) => true,
            _ => false,
        });
    }
}

#[cfg(test)]
mod tests_from_trait {
    use super::Date;

    #[test]
    fn test_date_from_trait_ok1() {
        let d1 = Date::from("1987-02-21");
        assert_eq!("1987-02-21", d1.to_string());
    }

    #[test]
    fn test_date_from_trait_ok2() {
        let d1 = Date::from("1987-02-21".to_string());
        assert_eq!("1987-02-21", d1.to_string());
    }
}
