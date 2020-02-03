
use std::fmt::{Display, Formatter, Result as FmtRes};
use std::convert::From;
use std::str::FromStr;
use uuid::Uuid;
use crate::date::Date;
use crate::number::{Number, NumberType, ToDisplay};
use crate::command::CommandOptions;
use crate::yaml::{ToYaml, FromYaml};
use crate::string::ShortString;
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
        // println!("-> Entry::new()");

        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            date: Date::new(),
            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
            category: "default".to_string(),
            comment: String::new(),
            epic: "default".to_string(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn date(&self) -> Date {
        self.date
    }

    pub fn set_date(&mut self, d: Date) {
        self.date = d;
    }

    pub fn revenue(&self) -> Number {
        self.revenue.clone()
    }

    pub fn set_revenue(&mut self, v: NumberType) {
        self.revenue = Number::from(v.abs());
        self.calc();
    }

    pub fn has_revenue(&self) -> bool {
        self.revenue.unwrap() > 0.0
    }

    pub fn expense(&self) -> Number {
        self.expense.clone()
    }

    pub fn set_expense(&mut self, v: NumberType) {
        self.expense = Number::from(-v.abs());
        self.calc();
    }

    pub fn has_expense(&self) -> bool {
        self.expense.unwrap() < 0.0
    }

    pub fn category(&self) -> String {
        self.category.clone()
    }

    pub fn set_category(&mut self, v: String) {
        self.category = v;
    }

    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    pub fn set_comment(&mut self, v: String) {
        self.comment = v;
    }

    pub fn epic(&self) -> String {
        self.epic.clone()
    }

    pub fn set_epic(&mut self, v: String) {
        self.epic = v;
    }

    pub fn balance(&self) -> Number {
        self.balance.clone()
    }

    fn calc(&mut self) {
        self.balance = Number::from(self.revenue.unwrap() + self.expense.unwrap());
    }
}

impl Display for Entry {
    /// Needed?
    fn fmt(&self, f: &mut Formatter) -> FmtRes {
        write!(f, "{}", self.id)
    }
}

// TODO tests
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

impl From<&str> for Entry {
    fn from(s: &str) -> Entry {
        // println!("-> Entry::from<&str>({:?})", s);

        Entry::from(s.to_string())
    }
}

impl From<String> for Entry {
    fn from(s: String) -> Entry {
        // println!("-> Entry::from<String>({:?})", s);

        let items: Vec<&str> = s.split('/').collect();
        // println!("-> items: {:?}", items);

        let date = Date::from_str(items[1]).expect("Failed to parse Date from String");
        // println!("-> date: {:?}", date);

        let mut entry = Entry::new();
        entry.set_title(items[0].to_string());
        entry.set_date(date);
        entry.set_revenue(items[2].parse().unwrap());
        entry.set_expense(items[3].parse().unwrap());
        entry
    }
}

// TODO: remove
impl From<i8> for Entry {
    fn from(_y: i8) -> Entry {
        // println!("-> Entry::from i8");

        let entry = Entry::new();
        entry
    }
}

// TODO tests
impl ToYaml for Entry {
    fn to_yaml(self) -> Yaml {
        println!("-> Entry::to_yaml()");

        let mut entry = Hash::new();
        entry.insert("id".to_string().to_yaml(), self.id().to_yaml());
        entry.insert("title".to_string().to_yaml(), self.title().to_yaml());
        entry.insert("date".to_string().to_yaml(), self.date().to_string().to_yaml());
        entry.insert("revenue".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.revenue.unwrap())));
        entry.insert("expense".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.expense.unwrap())));
        entry.insert("balance".to_string().to_yaml(), Yaml::Real(format!("{:.2}", self.balance.unwrap())));
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
                    // entry.expense = expense.parse().unwrap();
                    entry.expense = Number::from(expense.parse().unwrap());
                }
            }

            // Balance
            let key = "balance".to_string().to_yaml();
            if let Some(o_val) = item_ref.get(&key) {
                if let Yaml::Real(balance) = o_val {
                    // println!("-> balance: {:?}", balance);
                    // entry.balance = balance.parse().unwrap();
                    entry.balance = Number::from(balance.parse().unwrap());
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
    pub revenue: Number,
    pub expense: Number,
    pub balance: Number,
}

// TODO tests
impl EntrySum {
    pub fn new() -> Self {
        EntrySum {
            n: 0,
            revenue: Number::new(),
            expense: Number::new(),
            balance: Number::new(),
        }
    }

    pub fn inc(&mut self) {
        self.n += 1;
    }

    fn inc_revenue(&mut self, v: Number) {
        self.revenue += v;
    }

    fn inc_expense(&mut self, v: Number) {
        self.expense += v;
    }

    fn inc_balance(&mut self, v: Number) {
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
            let revenue_number = entry.revenue();
            let expense_number = entry.expense();
            let balance_number = entry.balance();

            sum.inc();
            // sum.inc_revenue(revenue_number.clone());
            // sum.inc_expense(expense_number.clone());
            // sum.inc_balance(balance_number.clone());

            let title = ShortString::from(entry.title(), 23);

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2}  {}",
                sum.n,
                entry.date().ymd(),
                revenue_number.to_display(),
                expense_number.to_display(),
                balance_number.to_display(),
                title,
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue.to_display(),
            sum.expense.to_display(),
            sum.balance.to_display());
    }

    fn show_normal(&self) {
        println!("-> EntryDisplay::show_normal()");

        let mut sum = EntrySum::new();

        println!("#### Date          Revenue    Expense    Balance   Category       Epic  Title");

        for entry in &self.entries {
            let revenue_number = entry.revenue();
            let expense_number = entry.expense();
            let balance_number = entry.balance();

            sum.inc();
            // sum.inc_revenue(revenue_number.clone());
            // sum.inc_expense(expense_number.clone());
            // sum.inc_balance(balance_number.clone());

            let category = ShortString::from(entry.category(), 10);
            let mut epic = ShortString::from(entry.epic(), 10); // TODO: use EpicDisplay here
            let title = ShortString::from(entry.title(), 23);

            if epic.to_string() == "default".to_string() {
                epic = ShortString::new();
            }

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2} {:>10} {:>10}  {}",
                sum.n,
                entry.date().ymd(),
                revenue_number.to_display(),
                expense_number.to_display(),
                balance_number.to_display(),
                category.to_string(),
                epic.to_string(),
                title.to_string(),
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue.to_display(),
            sum.expense.to_display(),
            sum.balance.to_display());
    }

    fn show_long(&self) {
        println!("-> EntryDisplay::show_long()");

        let mut sum = EntrySum::new();

        println!("#### Date          Revenue    Expense    Balance             Category                 Epic   Title");

        for entry in &self.entries {
            let revenue_number = entry.revenue();
            let expense_number = entry.expense();
            let balance_number = entry.balance();

            sum.inc();
            // sum.inc_revenue(revenue_number.clone());
            // sum.inc_expense(expense_number.clone());
            // sum.inc_balance(balance_number.clone());

            println!("{:<4} {} {:>10.2} {:>10.2} {:>10.2} {:>20} {:>20}   {}",
                sum.n,
                entry.date().ymd(),
                revenue_number.to_display(),
                expense_number.to_display(),
                balance_number.to_display(),
                entry.category(),
                entry.epic(),
                entry.title(),
            );
        }

        println!("TOTAL           {:>10.2} {:>10.2} {:>10.2}",
            sum.revenue.to_display(),
            sum.expense.to_display(),
            sum.balance.to_display());
    }
}

#[cfg(test)]
mod tests_basic {
    use super::Entry;

    #[test]
    fn test_entry_calc_revenue() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        assert_eq!(entry.balance().unwrap(), 1.0);

        entry.set_revenue(-2.0);
        assert_eq!(entry.balance().unwrap(), 2.0);
    }

    #[test]
    fn test_entry_calc_expense() {
        let mut entry = Entry::new();

        entry.set_expense(1.0);
        assert_eq!(entry.balance().unwrap(), -1.0);

        entry.set_expense(-2.0);
        assert_eq!(entry.balance().unwrap(), -2.0);
    }

    #[test]
    fn test_entry_calc_balance() {
        let mut entry = Entry::new();

        entry.set_revenue(1.0);
        entry.set_expense(1.0);
        assert_eq!(entry.balance().unwrap(), 0.0);

        entry.set_revenue(1.0);
        entry.set_expense(20.0);
        assert_eq!(entry.balance().unwrap(), -19.0);

        entry.set_revenue(10.0);
        entry.set_expense(1.0);
        assert_eq!(entry.balance().unwrap(), 9.0);
    }
}

#[cfg(test)]
mod tests_from_commandoptions {}

#[cfg(test)]
mod tests_from_string {
    use std::convert::From;
    use super::Entry;

    #[test]
    fn test_entry_from_string1() {
        Entry::from("Hello World/2001-02-03/30/20");
    }

    #[test]
    fn test_entry_from_string2() {
        let e1 = Entry::from("Hello World/2001-02-03/30/20".to_string());

        assert_eq!("Hello World", e1.title());

        let d1 = e1.date();
        assert_eq!(2001, d1.year());

        assert!(e1.has_revenue());
        assert!(e1.has_expense());
        assert_eq!(30.0, e1.revenue().unwrap());
        assert_eq!(-20.0, e1.expense().unwrap());
        assert_eq!(10.0, e1.balance().unwrap());

        // assert!(false);
    }
}

#[cfg(test)]
mod tests_to_yaml {}

#[cfg(test)]
mod tests_from_yaml {
    use super::Entry;
    use yaml_rust::yaml::Hash;
    use crate::yaml::{ToYaml, FromYaml};

    #[test]
    fn test_entry_fromyaml1() {
        let h = Hash::new();
        let y = h.to_yaml();
        Entry::from_yaml(&y);
    }

    #[test]
    fn test_entry_fromyaml2() {
        let mut hash = Hash::new();
        hash.insert("id".to_string().to_yaml(), "ID".to_string().to_yaml());
        hash.insert("title".to_string().to_yaml(), "Title".to_string().to_yaml());
        hash.insert("date".to_string().to_yaml(), "2019-02-21".to_string().to_yaml());
        hash.insert("revenue".to_string().to_yaml(), 42.2_f64.to_yaml());
        hash.insert("expense".to_string().to_yaml(), 42.3_f64.to_yaml());
        hash.insert("balance".to_string().to_yaml(), 42.4_f64.to_yaml());
        hash.insert("category".to_string().to_yaml(), "Category".to_string().to_yaml());
        hash.insert("comment".to_string().to_yaml(), "Comment".to_string().to_yaml());
        hash.insert("epic".to_string().to_yaml(), "Epic".to_string().to_yaml());

        let y = hash.to_yaml();
        let entry = Entry::from_yaml(&y);

        assert_eq!("ID", entry.id());
        assert_eq!("Title", entry.title());
        assert_eq!("2019-02-21", entry.date().to_string());
        assert_eq!(42.2, entry.revenue().unwrap());
        assert_eq!(42.3, entry.expense().unwrap());
        assert_eq!(42.4, entry.balance().unwrap());
        assert_eq!("Category", entry.category());
        assert_eq!("Comment", entry.comment());
        assert_eq!("Epic", entry.epic());
    }
}

#[cfg(test)]
mod tests_sum {
    use super::EntrySum;
    use crate::number::Number;

    #[test]
    fn test_entry_sum1() {
        let s1 = EntrySum::new();
        assert_eq!(0, s1.n);
    }

    #[test]
    fn test_entry_sum2() {
        let mut s1 = EntrySum::new();
        s1.inc();
        s1.inc();
        s1.inc_revenue(Number::from(1.23));
        s1.inc_revenue(Number::from(1.23));
        s1.inc_expense(Number::from(1.23));
        s1.inc_expense(Number::from(1.23));
        s1.inc_balance(Number::from(1.23));
        s1.inc_balance(Number::from(1.23));

        assert_eq!(2, s1.n);
        assert_eq!(Number::from(2.46), s1.revenue);
        assert_eq!(Number::from(2.46), s1.expense);
        assert_eq!(Number::from(2.46), s1.balance);
    }
}

#[cfg(test)]
mod tests_display {
    use super::{Entry, EntryDisplay, EntryDisplayKind};

    #[test]
    fn test_display1() {
        let list: Vec<Entry> = vec![];
        EntryDisplay::new(list, EntryDisplayKind::Normal);
    }
}
