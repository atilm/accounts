use super::BankStatementParserImplementation;
use super::ParserError;
use crate::parsers::*;

pub struct IngExtraAccountParser {}

impl IngExtraAccountParser {
    pub fn can_parse(file_path: &str) -> Result<bool, ParserError>
    {
        let mut decoder = get_decoded_file_reader(file_path);

        let mut buf = String::new();
        decoder.read_to_string(&mut buf).map_err(|_| ParserError::FileReadError)?;

        Ok(buf.contains(r#"Kontoname;Extra-Konto"#))
    }
}

impl BankStatementParserImplementation for IngExtraAccountParser {
    fn get_header_parser(&self) -> BankStatementHeaderParser {
        BankStatementHeaderParser {
            header_length: 12,
            account_number_regex: r#"IBAN;(?P<account>[A-Z\d\s]+)"#.to_string(),
            balance_amount_regex: r#"Saldo;(?P<amount>[+-]?[\d,.]+);EUR"#.to_string(),
            parse_amount: parse_float,
            balance_date_regex: r#"Datei erstellt am: (?P<date>[\d.]+)"#.to_string(),
            account_type: AccountType::IngExtraAccount,
        }
    }

    fn parse_record(&self, record: &csv::StringRecord) -> Result<AccountRecord, ParserError> {
        Ok(AccountRecord {
            amount: parse_float(&record[7])?,
            date: parse_date(&record[0])?,
            other_side: Some(record[2].to_string()),
            booking_text: record[3].to_string(),
            purpose: Some(record[4].to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{AccountRecord, AccountType};
    use crate::parsers::parser_factory::ParserFactory;
    use approx::assert_relative_eq;
    use chrono::NaiveDate;


    const FILE_PATH: &str = "./src/parsers/testData/ing_extra_account_statement.csv";

    #[test]
    fn can_parse_ing_giro_account_statment() {

        let parser = ParserFactory::create(FILE_PATH).unwrap();

        let parser_result = parser.parse(FILE_PATH).unwrap();

        let expected_records = vec![
            AccountRecord {
                amount: 123.54,
                date: chrono::NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(),
                other_side: Some("".to_string()),
                booking_text: "Abschluss".to_string(),
                purpose: Some("".to_string())
            },
            AccountRecord {
                amount: -3.22,
                date: chrono::NaiveDate::from_ymd_opt(2023, 12, 29).unwrap(),
                other_side: Some("".to_string()),
                booking_text: "Zuschlag".to_string(),
                purpose: Some("".to_string())
            }
        ];

        assert_eq!(parser_result.account_name, "DE08 5001 0517 5553 6114 73");
        assert_eq!(parser_result.account_type, AccountType::IngExtraAccount);
        assert_relative_eq!(parser_result.current_balance, 12345.01);
        assert_eq!(
            parser_result.current_balance_date,
            NaiveDate::from_ymd_opt(2024, 9, 4).unwrap()
        );
        assert_eq!(parser_result.records, expected_records);
    }
}
