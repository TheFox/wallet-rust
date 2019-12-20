/// Extern Implementations

use crate::types::Number;

pub trait StringExt {
    fn replace_comma(&self) -> String;
    fn to_num(&self) -> Number;
}

impl StringExt for String {
    /// On some places on earth you write 1,23
    /// instead of 1.23 as a floating point number.
    /// This function replaces ',' with '.'.
    fn replace_comma(&self) -> String {
        println!("-> String.replace_comma() -> {:?}", self);

        self.replace(",", ".")
    }

    /// Convert String to Number.
    fn to_num(&self) -> Number {
        println!("-> String.to_num() -> {:?}", self);

        self.parse().expect("Failed to convert String to Number")
    }
}

#[cfg(test)]
mod tests {
    use super::StringExt;

    #[test]
    fn test_replace_comma1(){
        let s1 = String::from("1,2");
        assert_eq!("1.2", s1.replace_comma());
    }

    #[test]
    fn test_to_num1(){
        let s1 = String::from("1.2");
        assert_eq!(1.2, s1.to_num());
    }
}
