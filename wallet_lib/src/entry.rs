
use std::fmt::{Display, Formatter, Result as FmtRes};
use uuid::Uuid;
use crate::date::Date;
use crate::types::Number;
use crate::yaml::ToYaml;
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

#[derive(Debug)]
pub struct Entry {
    id: String,
    title: String,
    date: Date,
    revenue: Number,
    expense: Number,
    balance: Number,
    category: String,
    comment: String,
    epic: String,
}

impl Entry {
    pub fn new() -> Self {
        println!("-> Entry::new()");

        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            date: Date::new(),
            revenue: 0.0,
            expense: 0.0,
            balance: 0.0,
            category: String::from("default"),
            comment: String::new(),
            epic: String::from("default"),
        }
    }

    // TODO: use this instead of Entry::new()
    // pub fn from()

    pub fn id(&self) -> String {
        // println!("-> Entry::id()");
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        println!("-> Entry::set_id({})", id);
        self.id = id;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        println!("-> Entry::set_title({})", title);
        self.title = title;
    }

    pub fn date(&self) -> Date {
        println!("-> Entry::date()");
        self.date
    }

    pub fn set_date(&mut self, d: Date) {
        println!("-> Entry::set_date({})", d);
        self.date = d;
    }

    pub fn set_revenue(&mut self, v: Number) {
        // println!("-> Entry::set_revenue({})", v);
        self.revenue = v.abs();
        self.calc();
    }

    pub fn set_expense(&mut self, v: Number) {
        // println!("-> Entry::set_expense({}) -> {}", v, -v.abs());
        self.expense = -v.abs();
        self.calc();
    }

    pub fn category(&self) -> String {
        self.category.clone()
    }

    pub fn set_category(&mut self, v: String) {
        // println!("-> Entry::set_category({})", v);
        self.category = v;
    }

    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    pub fn set_comment(&mut self, v: String) {
        // println!("-> Entry::set_comment({})", v);
        self.comment = v;
    }

    pub fn epic(&self) -> String {
        self.epic.clone()
    }

    pub fn set_epic(&mut self, v: String) {
        // println!("-> Entry::set_epic({})", v);
        self.epic = v;
    }

    pub fn get_balance(&self) -> Number {
        // println!("-> Entry::get_balance() -> {}", self.balance);
        self.balance
    }

    fn calc(&mut self) {
        // println!("-> Entry::calc() -> r={} e={}", self.revenue, self.expense);
        self.balance = self.revenue + self.expense;
        // println!("-> b={}", self.balance);
    }
}

impl Display for Entry {
    /// Needed?
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        write!(f, "{}", self.id)
    }
}

impl ToYaml for Entry {
    fn to_yaml(self) -> Yaml {
        println!("-> Entry::to_yaml()");

        let mut entry = Hash::new();
        entry.insert("id".to_string().to_yaml(), self.id().to_yaml());
        entry.insert("title".to_string().to_yaml(), self.title().to_yaml());
        entry.insert("date".to_string().to_yaml(), self.date().to_string().to_yaml());
        entry.insert("revenue".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.revenue)));
        entry.insert("expense".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.expense)));
        entry.insert("category".to_string().to_yaml(), self.category().to_yaml());
        entry.insert("comment".to_string().to_yaml(), self.comment().to_yaml());
        entry.insert("epic".to_string().to_yaml(), self.epic().to_yaml());

        Yaml::Hash(entry)
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
