use std::fmt::Display;

use itertools::Itertools;

use super::year_month::YearMonth;
use super::AccountRecord;

#[derive(Debug, PartialEq)]
pub struct MonthlyReport {
    pub month: YearMonth,
    pub records: Vec<AccountRecord>,
}

impl Display for MonthlyReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = 10;

        write!(f, "{}\n", self.month)?;
        write!(f, "Earnings: {}\n", self.earnings())?;
        for r in self.biggest_earnings(n) {
            write!(f, "  {:?}\n", r)?;
        }
        write!(f, "Spendings: {}\n", self.spendings())?;
        for r in self.biggest_spendings(n) {
            write!(f, "  {:?}\n", r)?;
        }
        write!(f, "Balance: {}\n", self.balance())
    }
}

impl MonthlyReport {
    pub fn biggest_earnings(&self, n: usize) -> Vec<&AccountRecord> {
        self.records
            .iter()
            .filter(|r| r.is_earning())
            .sorted_unstable_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap())
            .take(n)
            .collect()
    }

    pub fn biggest_spendings(&self, n: usize) -> Vec<&AccountRecord> {
        self.records
            .iter()
            .filter(|r| r.is_spending())
            .sorted_unstable_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap())
            .take(n)
            .collect()
    }

    pub fn earnings(&self) -> f64 {
        self.records
            .iter()
            .filter(|r| r.is_earning())
            .map(|r| r.amount)
            .sum()
    }

    pub fn spendings(&self) -> f64 {
        self.records
            .iter()
            .filter(|r| r.is_spending())
            .map(|r| r.amount)
            .sum()
    }

    pub fn balance(&self) -> f64 {
        self.spendings() + self.earnings()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{monthly_report::YearMonth, AccountRecord};

    use super::{super::test_util::*, MonthlyReport};

    #[test]
    fn return_biggest_earnings() {
        let report = MonthlyReport {
            month: YearMonth {
                year: 2024,
                month0: 0,
            },
            records: vec![
                new_record(110.0, "1.1.2024"),
                new_record(130.0, "1.1.2024"),
                new_record(120.0, "1.1.2024"),
                new_record(-150.0, "1.1.2024"),
            ],
        };

        let result = report.biggest_earnings(2);

        let expected = vec![new_record(130.0, "1.1.2024"), new_record(120.0, "1.1.2024")];

        assert_eq!(result, expected.iter().collect::<Vec<&AccountRecord>>());
    }

    #[test]
    fn return_biggest_spendings() {
        let report = MonthlyReport {
            month: YearMonth {
                year: 2024,
                month0: 0,
            },
            records: vec![
                new_record(-110.0, "1.1.2024"),
                new_record(-130.0, "1.1.2024"),
                new_record(-120.0, "1.1.2024"),
                new_record(150.0, "1.1.2024"),
            ],
        };

        let result = report.biggest_spendings(2);

        let expected = vec![
            new_record(-130.0, "1.1.2024"),
            new_record(-120.0, "1.1.2024"),
        ];

        assert_eq!(result, expected.iter().collect::<Vec<&AccountRecord>>());
    }
}
