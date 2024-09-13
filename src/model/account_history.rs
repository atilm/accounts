use itertools::Itertools;

use super::*;

#[derive(Debug, Error, PartialEq)]
pub enum AccountHistoryError {
    #[error("Date out of bounds.")]
    DateOutOfBounds,
}

#[derive(Debug, PartialEq)]
pub struct AccountHistory {
    pub account_name: String,
    pub account_type: AccountType,
    pub current_balance_date: NaiveDate,
    pub current_balance: f64,
    pub records: Vec<AccountRecord>,
}

impl AccountHistory {
    pub fn get_balance_at(&self, date: NaiveDate) -> Result<f64, AccountHistoryError> {
        let mut current_balance = self.current_balance;
        for record in self.records.iter() {
            if date >= record.date {
                return Ok(current_balance);
            }

            current_balance -= record.amount;
        }

        Err(AccountHistoryError::DateOutOfBounds)
    }
}

// ToDo: merge two AccountHistories of the same account

pub struct MergeRule {
    other_side_contains: Option<String>,
    booking_text_contains: Option<String>,
}

impl MergeRule {
    fn applies(&self, record: &AccountRecord) -> bool {
        false
    }
}

pub fn merge_records(
    histories: Vec<Vec<AccountRecord>>,
    remove_rules: Vec<MergeRule>
) -> Result<Vec<AccountRecord>, AccountHistoryError> {
    return Ok(histories.concat().into_iter().unique().collect());
    // merge two overlapping histories from the same account
    // merge two histories from different accounts
    //   remove duplicate entries
    //   remove bookings between own accounts
    // output a report or log
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use chrono::Duration;
    use std::{str::FromStr, vec};

    fn str_date(date: &str) -> NaiveDate {
        chrono::NaiveDate::parse_from_str(date, "%d.%m.%Y").unwrap()
    }

    fn given_a_history() -> AccountHistory {
        AccountHistory {
            account_name: String::from_str("1018793511").unwrap(),
            account_type: AccountType::DKBAccount,
            current_balance_date: str_date("6.3.2024"),
            current_balance: 350.0,
            records: vec![
                new_record(300.0, "5.3.2024"),
                new_record(-50.0, "3.3.2024"),
                new_record(100.0, "1.3.2024"),
            ],
        }
    }

    fn new_record(amount: f64, date: &str) -> AccountRecord {
        let date = str_date(date);

        AccountRecord {
            amount,
            date,
            ..AccountRecord::default()
        }
    }

    fn new_owned_record(
        amount: f64,
        date: &str,
        other_side: Option<String>,
        booking_text: &str,
    ) -> AccountRecord {
        AccountRecord {
            amount,
            date: str_date(date),
            other_side,
            booking_text: booking_text.to_string(),
            ..AccountRecord::default()
        }
    }

    fn when_records_are_merged(
        histories: Vec<Vec<AccountRecord>>,
    ) -> Result<Vec<AccountRecord>, AccountHistoryError> {
        merge_records(histories, vec![])
    }

    fn when_records_are_merged_with_rules(
        histories: Vec<Vec<AccountRecord>>,
        own_account_rules: Vec<MergeRule>,
    ) -> Result<Vec<AccountRecord>, AccountHistoryError> {
        merge_records(histories, own_account_rules)
    }

    #[test]
    fn get_account_balances_at_historic_dates() {
        let history = given_a_history();
        let later_date = history.current_balance_date + Duration::days(1);
        assert_relative_eq!(history.get_balance_at(later_date).unwrap(), 350.0);
        assert_relative_eq!(
            history
                .get_balance_at(history.current_balance_date)
                .unwrap(),
            350.0
        );
        assert_relative_eq!(
            history.get_balance_at(history.records[1].date).unwrap(),
            50.0
        );
        assert_relative_eq!(
            history.get_balance_at(history.records[2].date).unwrap(),
            100.0
        );
        assert_relative_eq!(
            history
                .get_balance_at(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap())
                .unwrap(),
            100.0
        );
    }

    #[test]
    fn get_account_balance_before_first_records_returns_error() {
        let history = given_a_history();

        let too_early_date = history.records[2].date - Duration::days(1);
        assert_eq!(
            history.get_balance_at(too_early_date),
            Err(AccountHistoryError::DateOutOfBounds)
        )
    }

    #[test]
    fn when_only_one_history_is_merged_then_the_history_is_returned_unmodified() {
        let history = given_a_history();

        let merge_result = when_records_are_merged(vec![history.records]).unwrap();
        assert_eq!(merge_result, given_a_history().records);
    }

    #[test]
    fn when_two_sets_of_unique_records_are_merged_they_are_all_contained_in_the_result() {
        let first_set = vec![new_record(100.0, "1.3.2024"), new_record(200.0, "2.3.2024")];

        let second_set = vec![new_record(300.0, "3.3.2024"), new_record(400.0, "4.3.2024")];

        let merge_result =
            when_records_are_merged(vec![first_set.clone(), second_set.clone()]).unwrap();

        let expected = vec![
            new_record(100.0, "1.3.2024"),
            new_record(200.0, "2.3.2024"),
            new_record(300.0, "3.3.2024"),
            new_record(400.0, "4.3.2024"),
        ];

        assert_eq!(merge_result, expected);
    }

    #[test]
    fn when_two_sets_are_merged_duplicates_are_removed() {
        let first_set = vec![
            new_record(100.0, "1.3.2024"),
            new_record(200.0, "2.3.2024"),
            new_record(500.0, "5.3.2024"),
            new_record(500.0, "5.3.2024"),
        ];

        let second_set = vec![
            new_record(300.0, "3.3.2024"),
            new_record(100.0, "1.3.2024"),
            new_record(500.0, "5.3.2024"),
            new_record(600.0, "5.3.2024"),
        ];

        let merge_result =
            when_records_are_merged(vec![first_set.clone(), second_set.clone()]).unwrap();

        let expected = vec![
            new_record(100.0, "1.3.2024"),
            new_record(200.0, "2.3.2024"),
            new_record(500.0, "5.3.2024"),
            new_record(300.0, "3.3.2024"),
            new_record(600.0, "5.3.2024"),
        ];

        assert_eq!(merge_result, expected);
    }

    #[test]
    fn when_sets_are_merged_the_bookings_between_own_accounts_are_removed() {
        let record_set = vec![
            new_owned_record(
                100.0,
                "1.1.2024",
                Some("Harry Fisher".to_string()),
                "Booking",
            ),
            new_owned_record(100.0, "1.1.2024", Some("John Doe".to_string()), "Booking"),
            new_owned_record(100.0, "1.1.2024", Some("JOHN DOE".to_string()), "Booking"),
            new_owned_record(100.0, "1.1.2024", None, "Einzahlung"),
            new_owned_record(100.0, "1.1.2024", None, "UEBERTRG.SALDO ALTE KARTE"),
        ];

        let merge_rules = vec![
            MergeRule {
                other_side_contains: Some("John Doe".to_string()),
                booking_text_contains: None,
            },
            MergeRule {
                other_side_contains: None,
                booking_text_contains: Some("Einzahlung".to_string()),
            },
            MergeRule {
                other_side_contains: None,
                booking_text_contains: Some("UEBERTRG.SALDO".to_string()),
            },
        ];

        let merge_result = when_records_are_merged_with_rules(vec![record_set], merge_rules).unwrap();

        assert_eq!(
            merge_result,
            vec![new_owned_record(
                100.0,
                "1.1.2024",
                Some("Harry Fisher".to_string()),
                "Booking"
            )]
        );
    }
}
