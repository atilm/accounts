pub mod account_history;

use chrono::NaiveDate;
use thiserror::Error;

pub enum RecordCategory {
    Housing,
    Child,
    Food,
    Saving
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AccountType {
    DKBAccount,
    DKBCreditCard,
    IngGiroAccount,
    IngExtraAccount
}

#[derive(Debug, Default, PartialEq)]
pub struct AccountRecord {
    pub amount: f64,
    pub date: NaiveDate,
    pub other_side: Option<String>,
    pub booking_text: String,
    pub purpose: Option<String>
}