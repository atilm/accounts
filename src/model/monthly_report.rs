use std::{cmp::Ordering, collections::HashMap};

use chrono::Datelike;

use super::AccountRecord;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct YearMonth {
    year: i32,
    month0: u32,
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

impl MonthlyReport {
    pub fn earnings(&self) -> f64 {
        self.records.iter()
            .filter(|r| r.amount >= 0.0)
            .map(|r| r.amount)
            .sum()
    }

    pub fn spendings(&self) -> f64 {
        self.records.iter()
            .filter(|r| r.amount < 0.0)
            .map(|r| r.amount)
            .sum()
    }

    pub fn balance(&self) -> f64 {
        self.spendings() + self.earnings()
    }
}

#[derive(Default)]
pub struct MonthlyReports {
    reports: Vec<MonthlyReport>,
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

    use crate::model::monthly_report::YearMonth;

    use super::{super::test_util::*, MonthlyReport, MonthlyReports};

    #[test]
    fn create_a_monthly_report() {
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
