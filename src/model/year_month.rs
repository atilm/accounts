use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month0: u32,
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.month0 + 1)
    }
}

impl YearMonth {
    pub fn new(year: i32, month0: u32) -> YearMonth {
        YearMonth { year, month0 }
    }

    pub fn compare(&self, other: &YearMonth) -> Ordering {
        let a = self.year * 100 + self.month0 as i32;
        let b = other.year * 100 + other.month0 as i32;
        a.cmp(&b)
    }
}