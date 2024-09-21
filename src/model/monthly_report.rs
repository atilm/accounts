use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use chrono::Datelike;
use itertools::Itertools;

use super::AccountRecord;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct YearMonth {
    year: i32,
    month0: u32,
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

#[derive(Debug, PartialEq)]
pub struct MonthlyReport {
    month: YearMonth,
    records: Vec<AccountRecord>,
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

#[derive(Default)]
pub struct MonthlyReports {
    pub reports: Vec<MonthlyReport>,
}

impl MonthlyReports {
    pub fn create(records: Vec<AccountRecord>) -> MonthlyReports {
        let mut records_by_month: HashMap<YearMonth, Vec<AccountRecord>> = HashMap::new();

        // Group records by year-month combination
        for record in records {
            let year_month = YearMonth {
                year: record.date.year(),
                month0: record.date.month0(),
            };

            if !records_by_month.contains_key(&year_month) {
                let v: Vec<AccountRecord> = Vec::new();
                records_by_month.insert(year_month.clone(), v);
            }

            records_by_month.get_mut(&year_month).unwrap().push(record);
        }

        // Convert into report
        let mut reports: Vec<MonthlyReport> = Vec::new();
        for (month, records) in records_by_month.into_iter() {
            let report = MonthlyReport { month, records };
            reports.push(report);
        }

        reports.sort_unstable_by(|a, b| a.month.compare(&b.month));

        MonthlyReports { reports }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use chrono::NaiveDate;

    use crate::model::{monthly_report::YearMonth, AccountRecord};

    use super::{super::test_util::*, MonthlyReport, MonthlyReports};

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

    #[test]
    fn create_monthly_reports() {
        let given_records_from_various_months = vec![
            new_record(-100.0, "5.3.2023"),
            new_record(200.0, "5.3.2023"),
            new_record(-300.0, "6.3.2023"),
            new_record(400.0, "6.4.2023"),
            new_record(200.0, "20.4.2023"),
            new_record(-300.0, "5.3.2024"),
            new_record(400.0, "28.3.2024"),
        ];

        let monthly_reports = MonthlyReports::create(given_records_from_various_months);

        assert!(monthly_reports.reports.len() == 3);

        let expected_reports = vec![
            MonthlyReport {
                month: YearMonth::new(2023, 2),
                records: vec![
                    new_record(-100.0, "5.3.2023"),
                    new_record(200.0, "5.3.2023"),
                    new_record(-300.0, "6.3.2023"),
                ],
            },
            MonthlyReport {
                month: YearMonth::new(2023, 3),
                records: vec![
                    new_record(400.0, "6.4.2023"),
                    new_record(200.0, "20.4.2023"),
                ],
            },
            MonthlyReport {
                month: YearMonth::new(2024, 2),
                records: vec![
                    new_record(-300.0, "5.3.2024"),
                    new_record(400.0, "28.3.2024"),
                ],
            },
        ];

        assert_eq!(monthly_reports.reports, expected_reports);
        assert_relative_eq!(monthly_reports.reports[0].earnings(), 200.0);
        assert_relative_eq!(monthly_reports.reports[0].spendings(), -400.0);
        assert_relative_eq!(monthly_reports.reports[0].balance(), -200.0);
        assert_relative_eq!(monthly_reports.reports[1].earnings(), 600.0);
        assert_relative_eq!(monthly_reports.reports[1].spendings(), 0.0);
        assert_relative_eq!(monthly_reports.reports[2].earnings(), 400.0);
        assert_relative_eq!(monthly_reports.reports[2].spendings(), -300.0);
    }
}
