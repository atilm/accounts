pub mod account_history;
pub mod monthly_report;
pub mod record_merging;

use core::hash::Hasher;
use std::hash::Hash;

use chrono::NaiveDate;
use thiserror::Error;

pub enum RecordCategory {
    Housing,
    Child,
    Food,
    Saving,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AccountType {
    DKBAccount,
    DKBCreditCard,
    IngGiroAccount,
    IngExtraAccount,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountRecord {
    pub amount: f64,
    pub date: NaiveDate,
    pub other_side: Option<String>,
    pub booking_text: String,
    pub purpose: Option<String>,
}

impl AccountRecord {
    pub fn is_earning(&self) -> bool {
        self.amount >= 0.0
    }

    pub fn is_spending(&self) -> bool {
        self.amount < 0.0
    }
}

impl Eq for AccountRecord {}

impl Hash for AccountRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.other_side.hash(state);
        self.booking_text.hash(state);
        self.purpose.hash(state);
    }
}

#[cfg(test)]
mod test_util {
    use super::*;
    use crate::model::account_history::AccountHistory;

    pub fn given_a_history() -> AccountHistory {
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

    use std::{str::FromStr, vec};
    pub fn new_record(amount: f64, date: &str) -> AccountRecord {
        let date = str_date(date);

        AccountRecord {
            amount,
            date,
            ..AccountRecord::default()
        }
    }

    pub fn str_date(date: &str) -> NaiveDate {
        chrono::NaiveDate::parse_from_str(date, "%d.%m.%Y").unwrap()
    }
}
