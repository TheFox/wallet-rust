
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::convert::From;
use std::str::FromStr;
use uuid::Uuid;
use crate::date::Date;
use crate::types::Number;
use crate::command::CommandOptions;
use crate::yaml::{ToYaml, FromYaml};
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

#[derive(Debug, Clone)]
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

    pub fn id(&self) -> String {
        // println!("-> Entry::id()");
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        // println!("-> Entry::set_id({})", id);
        self.id = id;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        // println!("-> Entry::set_title({})", title);
        self.title = title;
    }

    pub fn date(&self) -> Date {
        // println!("-> Entry::date()");
        self.date
    }

    pub fn set_date(&mut self, d: Date) {
        // println!("-> Entry::set_date({})", d);
        self.date = d;
    }

    pub fn revenue(&self) -> Number {
        self.revenue
    }

    pub fn set_revenue(&mut self, v: Number) {
        // println!("-> Entry::set_revenue({})", v);
        self.revenue = v.abs();
        self.calc();
    }

    pub fn has_revenue(&self) -> bool {
        self.revenue > 0.0
    }

    pub fn expense(&self) -> Number {
        self.expense
    }

    pub fn set_expense(&mut self, v: Number) {
        // println!("-> Entry::set_expense({}) -> {}", v, -v.abs());
        self.expense = -v.abs();
        self.calc();
    }

    pub fn has_expense(&self) -> bool {
        self.expense < 0.0
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

    pub fn balance(&self) -> Number {
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

impl From<CommandOptions> for Entry {
    fn from(options: CommandOptions) -> Entry {
        println!("-> Entry::from({:?})", options);

        let mut entry = Entry::new();

        if let Some(ref id) = options.id {
            entry.set_id(id.clone());
        }

        if let Some(date) = options.date {
            entry.set_date(date);
        }

        if let Some(title) = options.title {
            entry.set_title(title.to_string());
        }
        if let Some(revenue) = options.revenue {
            entry.set_revenue(revenue);
        }
        if let Some(expense) = options.expense {
            entry.set_expense(expense);
        }
        if let Some(category) = options.category {
            entry.set_category(category.to_string());
        }
        if let Some(comment) = options.comment {
            entry.set_comment(comment.to_string());
        }
        if let Some(epic) = options.epic {
            entry.set_epic(epic.to_string());
        }

        entry
    }
}

impl From<i8> for Entry {
    fn from(y: i8) -> Entry {
        println!("-> Entry::from i8");

        let mut entry = Entry::new();
        entry
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
        entry.insert("balance".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.balance)));
        entry.insert("category".to_string().to_yaml(), self.category().to_yaml());
        entry.insert("comment".to_string().to_yaml(), self.comment().to_yaml());
        entry.insert("epic".to_string().to_yaml(), self.epic().to_yaml());

        Yaml::Hash(entry)
    }
}

// TODO tests
impl FromYaml for Entry {
    fn from_yaml(x: &Yaml) -> Self {
        println!("-> Entry::from_yaml()");
        // println!("-> x: {:?}", x);

        let mut entry = Entry::new();

        if let Yaml::Hash(ref item_ref) = x {
            // println!("-> item_ref: {:?}", item_ref);

            // ID
            let key = "id".to_string().to_yaml();
            if let Yaml::String(id) = &item_ref[&key] {
                // println!("-> id: {:?}", id);
                entry.id = id.to_string();
            }

            // Title
            let key = "title".to_string().to_yaml();
            if let Yaml::String(title) = &item_ref[&key] {
                // println!("-> title: {:?}", title);
                entry.title = title.to_string();
            }

            // Date
            let key = "date".to_string().to_yaml();
            if let Yaml::String(date) = &item_ref[&key] {
                // println!("-> date: {:?}", date);
                entry.date = Date::from_str(date).unwrap();
            }

            // Revenue
            let key = "revenue".to_string().to_yaml();
            // println!("-> revenue: {:?}", item_ref[&key]);
            if let Yaml::Real(revenue) = &item_ref[&key] {
                // println!("-> revenue: {:?}", revenue);
                entry.revenue = revenue.parse().unwrap();
                // println!("-> revenue: {:?}", entry.revenue);
            }

            // Expense
            let key = "expense".to_string().to_yaml();
            // println!("-> expense: {:?}", item_ref[&key]);
            if let Yaml::Real(expense) = &item_ref[&key] {
                // println!("-> expense: {:?}", expense);
                entry.expense = expense.parse().unwrap();
                // println!("-> expense: {:?}", entry.expense);
            }

            // Balance
            let key = "balance".to_string().to_yaml();
            // println!("-> balance: {:?}", item_ref[&key]);
            if let Yaml::Real(balance) = &item_ref[&key] {
                // println!("-> balance: {:?}", balance);
                entry.balance = balance.parse().unwrap();
            //     println!("-> balance: {:?}", entry.balance);
            }

            // Category
            let key = "category".to_string().to_yaml();
            if let Yaml::String(category) = &item_ref[&key] {
                // println!("-> category: {:?}", category);
                entry.category = category.to_string();
            }

            // Comment
            let key = "comment".to_string().to_yaml();
            if let Yaml::String(comment) = &item_ref[&key] {
                // println!("-> comment: {:?}", comment);
                entry.comment = comment.to_string();
            }

            // Epic
            let key = "epic".to_string().to_yaml();
            if let Yaml::String(epic) = &item_ref[&key] {
                // println!("-> epic: {:?}", epic);
                entry.epic = epic.to_string();
            }
        }

        entry
    }
}

#[derive(Debug)]
pub struct EntrySum {
    pub n: u64,
    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

// TODO tests
impl EntrySum {
    pub fn new() -> Self {
        EntrySum {
            n: 0,
            revenue: 0.0,
            expense: 0.0,
            balance: 0.0,
        }
    }

    pub fn inc(&mut self) {
        self.n += 1;
    }

    pub fn inc_revenue(&mut self, v: Number) {
        self.revenue += v;
    }

    pub fn inc_expense(&mut self, v: Number) {
        self.expense += v;
    }

    pub fn inc_balance(&mut self, v: Number) {
        self.balance += v;
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_calc_revenue() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        assert_eq!(entry.balance(), 1.0);

        entry.set_revenue(-2.0);
        assert_eq!(entry.balance(), 2.0);
    }

    #[test]
    fn test_calc_expense() {
        let mut entry = Entry::new();

        entry.set_expense(1.0);
        assert_eq!(entry.balance(), -1.0);

        entry.set_expense(-2.0);
        assert_eq!(entry.balance(), -2.0);
    }

    #[test]
    fn test_calc_balance() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        entry.set_expense(1.0);
        assert_eq!(entry.balance(), 0.0);

        entry.set_revenue(1.0);
        entry.set_expense(20.0);
        assert_eq!(entry.balance(), -19.0);

        entry.set_revenue(10.0);
        entry.set_expense(1.0);
        assert_eq!(entry.balance(), 9.0);
    }
}
