pub mod account_history;
pub mod record_merging;

use std::hash::Hash;
use core::hash::Hasher;

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AccountRecord {
    pub amount: f64,
    pub date: NaiveDate,
    pub other_side: Option<String>,
    pub booking_text: String,
    pub purpose: Option<String>
}

impl Eq for AccountRecord { }

impl Hash for AccountRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.other_side.hash(state);
        self.booking_text.hash(state);
        self.purpose.hash(state);
    }
}