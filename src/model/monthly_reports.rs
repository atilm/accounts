use std::collections::HashMap;

use chrono::Datelike;

use super::monthly_report::MonthlyReport;
use super::year_month::YearMonth;
use super::AccountRecord;

#[derive(Default)]
pub struct MonthlyReports {
    pub reports: Vec<MonthlyReport>,
}

pub fn average(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
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

    pub fn average_earnings(&self) -> f64 {
        let earnings: Vec<f64> = self.reports.iter().map(|r| r.earnings()).collect();
        average(&earnings)
    }

    pub fn average_spendings(&self) -> f64 {
        let spendings: Vec<f64> = self.reports.iter().map(|r| r.spendings()).collect();
        average(&spendings)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::{super::test_util::*, MonthlyReport, MonthlyReports, YearMonth};

    #[test]
    fn return_average_spendings_and_earnings() {
        let reports = MonthlyReports {
            reports: vec![
                MonthlyReport {
                    month: YearMonth {
                        year: 2024,
                        month0: 0,
                    },
                    records: vec![
                        new_record(200.0, "1.1.2024"),
                        new_record(-300.0, "1.1.2024"),
                    ],
                },
                MonthlyReport {
                    month: YearMonth {
                        year: 2024,
                        month0: 1,
                    },
                    records: vec![
                        new_record(100.0, "1.1.2024"),
                        new_record(-400.0, "1.1.2024"),
                    ],
                },
            ],
        };

        let average_earnings = reports.average_earnings();
        assert_relative_eq!(average_earnings, 150.0);

        let average_spendings = reports.average_spendings();
        assert_relative_eq!(average_spendings, -350.0);
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
