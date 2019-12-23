
use crate::date::Date;
use crate::types::Number;

#[derive(Debug)]
pub struct Entry {
    date: Date,
    revenue: Number,
    expense: Number,
    balance: Number,
}

impl Entry {
    pub fn new() -> Self {
        // println!("-> Entry::new()");

        Self {
            date: Date::new(),
            revenue: 0.0,
            expense: 0.0,
            balance: 0.0,
        }
    }

    // TODO: use this instead of Entry::new()
    // pub fn from()

    pub fn date(&self) -> Date {
        println!("-> Entry.date()");
        self.date
    }

    pub fn set_date(&mut self, d: Date) {
        println!("-> Entry.set_date({})", d);
        self.date = d;
    }

    pub fn set_revenue(&mut self, v: Number) {
        // println!("-> Entry.set_revenue({})", v);
        self.revenue = v.abs();
        self.calc();
    }

    pub fn set_expense(&mut self, v: Number) {
        // println!("-> Entry.set_expense({}) -> {}", v, -v.abs());
        self.expense = -v.abs();
        self.calc();
    }

    pub fn get_balance(&self) -> Number {
        // println!("-> Entry.get_balance() -> {}", self.balance);
        self.balance
    }

    fn calc(&mut self) {
        // println!("-> Entry.calc() -> r={} e={}", self.revenue, self.expense);
        self.balance = self.revenue + self.expense;
        // println!("-> b={}", self.balance);
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_calc_revenue() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        assert_eq!(entry.get_balance(), 1.0);

        entry.set_revenue(-2.0);
        assert_eq!(entry.get_balance(), 2.0);
    }

    #[test]
    fn test_calc_expense() {
        let mut entry = Entry::new();

        entry.set_expense(1.0);
        assert_eq!(entry.get_balance(), -1.0);

        entry.set_expense(-2.0);
        assert_eq!(entry.get_balance(), -2.0);
    }

    #[test]
    fn test_calc_balance() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        entry.set_expense(1.0);
        assert_eq!(entry.get_balance(), 0.0);

        entry.set_revenue(1.0);
        entry.set_expense(20.0);
        assert_eq!(entry.get_balance(), -19.0);

        entry.set_revenue(10.0);
        entry.set_expense(1.0);
        assert_eq!(entry.get_balance(), 9.0);
    }
}
