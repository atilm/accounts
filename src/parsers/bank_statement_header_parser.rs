use encoding_rs_io::DecodeReaderBytes;
use crate::model::account_history::AccountHistory;
use std::{self, io::{self, BufReader}};
use regex;
use std::io::Read;
use io::BufRead;
use std::str::FromStr;
use chrono::NaiveDate;
use super::*;

pub struct BankStatementHeaderParser {
    pub header_length: i32,
    pub parse_amount: fn(s: &str) -> Result<f64, ParserError>,
    pub account_number_regex: String,
    pub balance_amount_regex: String,
    pub balance_date_regex: String,
    pub account_type: AccountType,
}

impl BankStatementHeaderParser {
    pub fn parse(
        &self,
        line_reader: &mut BufReader<DecodeReaderBytes<impl Read, Vec<u8>>>,
    ) -> Result<AccountHistory, ParserError> {
        let mut account_name = String::from_str("AccountNumber").unwrap();
        let mut current_balance = 0.0;
        let mut current_balance_date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    
        for _i in 0..self.header_length {
            let mut buf = String::new();
            let _ = line_reader.read_line(&mut buf);
    
            let account_number_regex = regex::Regex::new(&self.account_number_regex).unwrap();
            if let Some(captures) = account_number_regex.captures(&buf) {
                account_name = captures["account"].trim().to_string();
            }
    
            let current_balance_date_regex =
                regex::Regex::new(&self.balance_date_regex).unwrap();
            if let Some(captures) = current_balance_date_regex.captures(&buf) {
                current_balance_date = parse_date(&captures["date"].to_string()).unwrap();
            }
    
            let current_balance_regex =
                regex::Regex::new(&self.balance_amount_regex).unwrap();
            if let Some(captures) = current_balance_regex.captures(&buf) {
                current_balance = (self.parse_amount)(&captures["amount"].to_string())?;
            }
        }
    
        Ok(AccountHistory {
            account_name,
            account_type: self.account_type,
            current_balance_date,
            current_balance,
            records: vec![],
        })
    }
}