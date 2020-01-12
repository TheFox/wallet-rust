
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::convert::From;
use std::str::FromStr;
use uuid::Uuid;
use crate::date::Date;
use crate::types::Number as NumberOldType;
use crate::number::{Number, NumberType};
use crate::command::CommandOptions;
use crate::yaml::{ToYaml, FromYaml};
use crate::string::{ShortString, ZeroString};
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

#[derive(Debug, Clone)]
pub struct Entry {
    id: String,
    title: String,
    date: Date,
    revenue: Number,
    expense: NumberOldType,
    balance: NumberOldType,
    category: String,
    comment: String,
    epic: String,
}

impl Entry {
    pub fn new() -> Self {
        // println!("-> Entry::new()");

        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            date: Date::new(),
            revenue: Number::new(),
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

    pub fn revenue(&self) -> NumberType {
        self.revenue.n
    }

    pub fn set_revenue(&mut self, v: NumberType) {
        // println!("-> Entry::set_revenue({})", v);
        self.revenue = Number::from(v.abs());
        self.calc();
    }

    pub fn has_revenue(&self) -> bool {
        self.revenue.n > 0.0
    }

    pub fn expense(&self) -> NumberOldType {
        self.expense
    }

    pub fn set_expense(&mut self, v: NumberOldType) {
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

    pub fn balance(&self) -> NumberOldType {
        self.balance
    }

    fn calc(&mut self) {
        // println!("-> Entry::calc() -> r={} e={}", self.revenue, self.expense);
        self.balance = self.revenue.n + self.expense;
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
        // println!("-> Entry::from({:?})", options);

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
        // println!("-> Entry::from i8");

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
        entry.insert("revenue".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.revenue.n)));
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
    fn from_yaml(yaml: &Yaml) -> Self {
        // println!("-> Entry::from_yaml()");

        let mut entry = Entry::new();

        if let Yaml::Hash(ref item_ref) = yaml {
            // println!("-> item_ref: {:?}", item_ref);

            // ID
            let key = "id".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(id) = o_val {
                    // println!("-> id: {:?}", id);
                    entry.id = id.to_string();
                }
            }

            // Title
            let key = "title".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(title) = o_val {
                    // println!("-> title: {:?}", title);
                    entry.title = title.to_string();
                }
            }

            // Date
            let key = "date".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(date) = o_val {
                    // println!("-> date: {:?}", date);
                    entry.date = Date::from_str(date).unwrap();
                }
            }

            // Revenue
            let key = "revenue".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::Real(revenue) = o_val {
                    // println!("-> revenue: {:?}", revenue);
                    // entry.revenue = revenue.parse().unwrap();
                    entry.revenue = Number::from(revenue.parse().unwrap());
                }
            }

            // Expense
            let key = "expense".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::Real(expense) = o_val {
                    // println!("-> expense: {:?}", expense);
                    entry.expense = expense.parse().unwrap();
                }
            }

            // Balance
            let key = "balance".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::Real(balance) = o_val {
                    // println!("-> balance: {:?}", balance);
                    entry.balance = balance.parse().unwrap();
                }
            }

            // Category
            let key = "category".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(category) = o_val {
                    // println!("-> category: {:?}", category);
                    entry.category = category.to_string();
                }
            }

            // Comment
            let key = "comment".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(comment) = o_val {
                    // println!("-> comment: {:?}", comment);
                    entry.comment = comment.to_string();
                }
            }

            // Epic
            let key = "epic".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::String(epic) = o_val {
                    // println!("-> epic: {:?}", epic);
                    entry.epic = epic.to_string();
                }
            }
        }

        entry
    }
}

#[derive(Debug)]
pub struct EntrySum {
    pub n: u64,
    pub revenue: NumberOldType,
    pub expense: NumberOldType,
    pub balance: NumberOldType,
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

    pub fn inc_revenue(&mut self, v: NumberOldType) {
        self.revenue += v;
    }

    pub fn inc_expense(&mut self, v: NumberOldType) {
        self.expense += v;
    }

    pub fn inc_balance(&mut self, v: NumberOldType) {
        self.balance += v;
    }
}

pub enum EntryDisplayKind {
    Short,
    Normal,
    Long,
}

pub struct EntryDisplay {
    entries: Vec<Entry>,
    kind: EntryDisplayKind,
}

impl EntryDisplay {
    pub fn new(entries: Vec<Entry>, kind: EntryDisplayKind) -> Self {
        EntryDisplay {
            entries,
            kind,
        }
    }

    pub fn show(&self) {
        if self.entries.len() == 0 {
            println!("No entries found.");
            return;
        }

        match self.kind {
            EntryDisplayKind::Short => self.show_short(),
            EntryDisplayKind::Normal => self.show_normal(),
            EntryDisplayKind::Long => self.show_long(),
        }
    }

    fn show_short(&self) {
        println!("-> EntryDisplay::show_short()");

        let mut sum = EntrySum::new();

        println!("#### Date          Revenue    Expense    Balance  Title");

        for entry in &self.entries {
            sum.inc();
            sum.inc_revenue(entry.revenue());
            sum.inc_expense(entry.expense());
            sum.inc_balance(entry.balance());

            let title = ShortString::from(entry.title(), 23);

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2}  {}",
                sum.n,
                entry.date().ymd(),
                entry.revenue(),
                entry.expense(),
                entry.balance(),
                title,
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue,
            sum.expense,
            sum.balance);
    }

    fn show_normal(&self) {
        println!("-> EntryDisplay::show_normal()");

        let mut sum = EntrySum::new();

        println!("#### Date          Revenue    Expense    Balance   Category       Epic  Title");

        for entry in &self.entries {
            sum.inc();
            sum.inc_revenue(entry.revenue());
            sum.inc_expense(entry.expense());
            sum.inc_balance(entry.balance());

            let category = ShortString::from(entry.category(), 10);
            let mut epic = ShortString::from(entry.epic(), 10);
            let title = ShortString::from(entry.title(), 23);

            if epic.to_string() == "default".to_string() {
                epic = ShortString::new();
            }

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2} {:>10} {:>10}  {}",
                sum.n,
                entry.date().ymd(),
                entry.revenue(),
                entry.expense(),
                entry.balance(),
                category.to_string(),
                epic.to_string(),
                title.to_string(),
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue,
            sum.expense,
            sum.balance);
    }

    fn show_long(&self) {
        println!("-> EntryDisplay::show_long()");

        let mut sum = EntrySum::new();

        println!("#### Date          Revenue    Expense    Balance             Category                 Epic   Title");

        for entry in &self.entries {
            sum.inc();
            sum.inc_revenue(entry.revenue());
            sum.inc_expense(entry.expense());
            sum.inc_balance(entry.balance());

            // let revenue = ZeroString::from(entry.revenue());

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2} {:>20} {:>20}   {}",
                sum.n,
                entry.date().ymd(),
                // revenue.to_string(),
                entry.revenue(),
                entry.expense(),
                entry.balance(),
                entry.category(),
                entry.epic(),
                entry.title(),
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue,
            sum.expense,
            sum.balance);
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;
    use yaml_rust::yaml::Hash;
    use crate::yaml::{ToYaml, FromYaml};

    #[test]
    fn test_entry_calc_revenue() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        assert_eq!(entry.balance(), 1.0);

        entry.set_revenue(-2.0);
        assert_eq!(entry.balance(), 2.0);
    }

    #[test]
    fn test_entry_calc_expense() {
        let mut entry = Entry::new();

        entry.set_expense(1.0);
        assert_eq!(entry.balance(), -1.0);

        entry.set_expense(-2.0);
        assert_eq!(entry.balance(), -2.0);
    }

    #[test]
    fn test_entry_calc_balance() {
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

    #[test]
    fn test_entry_fromyaml1() {
        let h = Hash::new();
        let y = h.to_yaml();
        Entry::from_yaml(&y);
    }

    #[test]
    fn test_entry_fromyaml2() {
        let mut h = Hash::new();
        h.insert("id".to_string().to_yaml(), "ID".to_string().to_yaml());
        h.insert("title".to_string().to_yaml(), "Title".to_string().to_yaml());
        h.insert("date".to_string().to_yaml(), "2019-02-21".to_string().to_yaml());
        h.insert("revenue".to_string().to_yaml(), 42.2_f64.to_yaml());
        h.insert("expense".to_string().to_yaml(), 42.3_f64.to_yaml());
        h.insert("balance".to_string().to_yaml(), 42.4_f64.to_yaml());
        h.insert("category".to_string().to_yaml(), "Category".to_string().to_yaml());
        h.insert("comment".to_string().to_yaml(), "Comment".to_string().to_yaml());
        h.insert("epic".to_string().to_yaml(), "Epic".to_string().to_yaml());

        let y = h.to_yaml();
        let entry = Entry::from_yaml(&y);

        assert_eq!("ID", entry.id());
        assert_eq!("Title", entry.title());
        assert_eq!("2019-02-21", entry.date().to_string());
        assert_eq!(42.2, entry.revenue());
        assert_eq!(42.3, entry.expense());
        assert_eq!(42.4, entry.balance());
        assert_eq!("Category", entry.category());
        assert_eq!("Comment", entry.comment());
        assert_eq!("Epic", entry.epic());
    }
}
