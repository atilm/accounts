use super::AccountRecord;
use chrono::NaiveDate;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MergeRule {
    pub other_side_is: Option<String>,
    pub booking_text_contains: Option<String>,
}

impl MergeRule {
    fn applies(&self, record: &AccountRecord) -> bool {
        self.other_side_rule_applies(record) && self.booking_text_rule_applies(record)
    }

    fn other_side_rule_applies(&self, record: &AccountRecord) -> bool {
        // contrary to the booking text rule, a rule value of None must
        // also be evaluated
        if self.other_side_is == None && record.other_side == None {
            return true;
        }

        if let Some(required_other_side) = &self.other_side_is {
            if let Some(actual_other_side) = &record.other_side {
                return actual_other_side.to_lowercase() == required_other_side.to_lowercase();
            }
        }

        false
    }

    fn booking_text_rule_applies(&self, record: &AccountRecord) -> bool {
        // check only, if the rule has some value
        if let Some(required_partial_booking_text) = &self.booking_text_contains {
            return record
                .booking_text
                .to_lowercase()
                .contains(&required_partial_booking_text.to_lowercase());
        }

        // otherwise the result must not negate the total result
        true
    }
}

pub fn merge_records(
    histories: Vec<Vec<AccountRecord>>,
    remove_rules: Vec<MergeRule>,
) -> Vec<AccountRecord> {
    let all_records = histories.concat();
    let unique_records: Vec<AccountRecord> = all_records.into_iter().unique().collect();
    let filtered_records = unique_records
        .into_iter()
        .filter(|record| !remove_rules.iter().any(|rule| rule.applies(record)))
        .collect();
    return filtered_records;
}

pub fn merge_records_from_date(
    histories: Vec<Vec<AccountRecord>>,
    remove_rules: Vec<MergeRule>,
    start_date: NaiveDate,
) -> Vec<AccountRecord> {
    let merged_records = merge_records(histories, remove_rules);

    merged_records
        .into_iter()
        .filter(|r| r.date >= start_date)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::account_history::AccountHistory;
    use crate::model::*;
    use chrono::NaiveDate;
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

    fn when_records_are_merged(histories: Vec<Vec<AccountRecord>>) -> Vec<AccountRecord> {
        merge_records(histories, vec![])
    }

    fn when_records_are_merged_with_rules(
        histories: Vec<Vec<AccountRecord>>,
        own_account_rules: Vec<MergeRule>,
    ) -> Vec<AccountRecord> {
        merge_records(histories, own_account_rules)
    }

    #[test]
    fn when_only_one_history_is_merged_then_the_history_is_returned_unmodified() {
        let history = given_a_history();

        let merge_result = when_records_are_merged(vec![history.records]);
        assert_eq!(merge_result, given_a_history().records);
    }

    #[test]
    fn when_two_sets_of_unique_records_are_merged_they_are_all_contained_in_the_result() {
        let first_set = vec![new_record(100.0, "1.3.2024"), new_record(200.0, "2.3.2024")];

        let second_set = vec![new_record(300.0, "3.3.2024"), new_record(400.0, "4.3.2024")];

        let merge_result = when_records_are_merged(vec![first_set.clone(), second_set.clone()]);

        let expected = vec![
            new_record(100.0, "1.3.2024"),
            new_record(200.0, "2.3.2024"),
            new_record(300.0, "3.3.2024"),
            new_record(400.0, "4.3.2024"),
        ];

        assert_eq!(merge_result, expected);
    }

    #[test]
    fn when_a_start_date_is_given_records_before_this_date_are_removed() {
        let first_set = vec![new_record(100.0, "1.3.2024"), new_record(200.0, "2.3.2024")];
        let second_set = vec![new_record(300.0, "3.3.2024"), new_record(400.0, "4.3.2024")];

        let merge_result = merge_records_from_date(
            vec![first_set, second_set],
            vec![],
            NaiveDate::from_ymd_opt(2024, 3, 3).unwrap(),
        );

        let expected = vec![new_record(300.0, "3.3.2024"), new_record(400.0, "4.3.2024")];

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

        let merge_result = when_records_are_merged(vec![first_set.clone(), second_set.clone()]);

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
                other_side_is: Some("John Doe".to_string()),
                booking_text_contains: None,
            },
            MergeRule {
                other_side_is: None,
                booking_text_contains: Some("Einzahlung".to_string()),
            },
            MergeRule {
                other_side_is: None,
                booking_text_contains: Some("UEBERTRG.SALDO".to_string()),
            },
        ];

        let merge_result = when_records_are_merged_with_rules(vec![record_set], merge_rules);

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
