use super::BankStatementParserImplementation;
use super::ParserError;
use crate::parsers::*;

pub struct IngGiroAccountParser {}

impl BankStatementParserImplementation for IngGiroAccountParser {
    fn get_header_parser(&self) -> BankStatementHeaderParser {
        BankStatementHeaderParser {
            header_length: 12,
            account_number_regex: r#"IBAN;(?P<account>[A-Z\d\s]+)"#.to_string(),
            balance_amount_regex: r#"Saldo;(?P<amount>[+-]?[\d,.]+);EUR"#.to_string(),
            parse_amount: parse_float,
            balance_date_regex: r#"Datei erstellt am: (?P<date>[\d.]+)"#.to_string(),
            account_type: AccountType::IngGiroAccount,
        }
    }

    fn parse_record(&self, record: &csv::StringRecord) -> Result<AccountRecord, ParserError> {
        Ok(AccountRecord {
            amount: parse_float(&record[5]).unwrap(),
            date: parse_date(&record[0])?,
            other_side: Some(record[2].to_string()),
            booking_text: record[3].to_string(),
            purpose: Some(record[4].to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IngGiroAccountParser;
    use crate::model::{AccountRecord, AccountType};
    use crate::parsers::BankStatementParser;
    use approx::assert_relative_eq;
    use chrono::NaiveDate;

    fn given_an_ing_giro_account_statement_file() -> std::fs::File {
        std::fs::File::open("./src/parsers/testData/ing_giro_account_statement.csv")
            .expect("Could not open file.")
    }

    #[test]
    fn can_parse_ing_giro_account_statment() {
        let file = given_an_ing_giro_account_statement_file();

        let parser = BankStatementParser {
            implementation: Box::new(IngGiroAccountParser {}),
        };

        let parser_result = parser.parse(file).unwrap();

        let expected_records = vec![
            AccountRecord {
                amount: -16.98,
                date: chrono::NaiveDate::from_ymd_opt(2024, 9, 4).unwrap(),
                other_side: Some("VISA AMZN MKTP DE*CB3UF2VD5".to_string()),
                booking_text: "Lastschrift".to_string(),
                purpose: Some("NR XXXX 5015 800-279-662 LU KAUFUMSATZ 02.09 16.98 101352 ARN74279814246101247805768".to_string())
            },
            AccountRecord {
                amount: 5000.72,
                date: chrono::NaiveDate::from_ymd_opt(2024, 8, 13).unwrap(),
                other_side: Some("Company".to_string()),
                booking_text: "Gehalt/Rente".to_string(),
                purpose: Some("LOHN / GEHALT 08/24".to_string())
            }
        ];

        assert_eq!(parser_result.account_name, "DE25 5001 0123 4567 8910 11");
        assert_eq!(parser_result.account_type, AccountType::IngGiroAccount);
        assert_relative_eq!(parser_result.current_balance, 12234.0);
        assert_eq!(
            parser_result.current_balance_date,
            NaiveDate::from_ymd_opt(2024, 9, 4).unwrap()
        );
        assert_eq!(parser_result.records, expected_records);
    }
}
