use bank_statement_header_parser::BankStatementHeaderParser;
use encoding_rs_io::DecodeReaderBytes;
use thiserror::Error;
use crate::model::{account_history::AccountHistory, *};
use std::{self, io::{self, BufReader}};
use encoding_rs::WINDOWS_1252;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex;
use std::io::Read;

pub mod bank_statement_header_parser;

pub mod dkb_account_parser;
pub mod dkb_credit_card_parser;
pub mod ing_giro_account_parser;
pub mod ing_extra_account_parser;


#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid date.")]
    InvalidDate,
    #[error("Invalid float.")]
    FloatError
}

pub trait BankStatementParserImplementation {
    fn parse_record(&self, record: &csv::StringRecord) -> Result<AccountRecord, ParserError>;
    fn get_header_parser(&self) -> BankStatementHeaderParser;
}

pub struct BankStatementParser {
    pub implementation: Box<dyn BankStatementParserImplementation>,
}

impl BankStatementParser {
    pub fn parse(&self, reader: impl io::Read) -> Result<AccountHistory, ParserError> {
        let mut buf_reader = get_decoded_lines_reader(reader);
        let account_history = self.parse_file_header(&mut buf_reader)?;
        let records = self.parse_records(&mut buf_reader)?;
        Ok(AccountHistory {
            records,
            ..account_history
        })
    }

    fn parse_file_header(
        &self,
        line_reader: &mut BufReader<DecodeReaderBytes<impl Read, Vec<u8>>>,
    ) -> Result<AccountHistory, ParserError> {
        let header_parser = self.implementation.get_header_parser();
        header_parser.parse(line_reader)
    }

    fn parse_records(
        &self,
        line_reader: &mut BufReader<DecodeReaderBytes<impl Read, Vec<u8>>>,
    ) -> Result<Vec<AccountRecord>, ParserError> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(line_reader);
    
        let account_records: Vec<AccountRecord> = csv_reader
            .records()
            .into_iter()
            .filter(|r| r.is_ok())
            .map(|r| self.implementation.parse_record(&r.unwrap()).unwrap())
            .collect();
    
        Ok(account_records)
    }
}

fn get_decoded_lines_reader(reader: impl io::Read) -> BufReader<DecodeReaderBytes<impl Read, Vec<u8>>> {
    let decoder = DecodeReaderBytesBuilder::new()
    .encoding(Some(WINDOWS_1252))
    .build(reader);

     BufReader::new(decoder)
}

fn parse_float(s: &str) -> Result<f64, ParserError> {
    parse_std_float(&s.replace(".", "").replace(",", "."))
}

fn parse_std_float(s: &str) -> Result<f64, ParserError> {
    s.parse::<f64>().map_err(|_|ParserError::FloatError)
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, ParserError> {
    let result = chrono::NaiveDate::parse_from_str(s, "%d.%m.%Y");

    match result {
        Ok(date) => Ok(date),
        Err(_) => Err(ParserError::InvalidDate)
    }
}