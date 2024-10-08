use thiserror::Error;

use super::{dkb_account_parser::DkbAccountParser, dkb_credit_card_parser::DkbCreditCardParser, ing_extra_account_parser::IngExtraAccountParser, ing_giro_account_parser::IngGiroAccountParser, BankStatementParser, BankStatementParserImplementation};

#[derive(Error, Debug)]
pub enum ParserFactoryError {
    #[error("No parser found.")]
    NoParserFound
}

pub struct ParserFactory { }

impl ParserFactory {
    pub fn create(file_path: &str) -> Result<BankStatementParser, ParserFactoryError> {
        let implementation = ParserFactory::get_implementation(file_path)?;
        
        Ok(BankStatementParser {
            implementation
        })
    }

    fn get_implementation(file_path: &str) -> Result<Box<dyn BankStatementParserImplementation>, ParserFactoryError> {
        if DkbAccountParser::can_parse(file_path).unwrap() {
            return Ok(Box::new(DkbAccountParser {}))
        }

        if DkbCreditCardParser::can_parse(file_path).unwrap() {
            return Ok(Box::new(DkbCreditCardParser {}))
        }

        if IngGiroAccountParser::can_parse(file_path).unwrap() {
            return Ok(Box::new(IngGiroAccountParser {}))
        }

        if IngExtraAccountParser::can_parse(file_path).unwrap() {
            return Ok(Box::new(IngExtraAccountParser {}))
        }

        Err(ParserFactoryError::NoParserFound)
    }
}