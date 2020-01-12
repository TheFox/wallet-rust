
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
}
