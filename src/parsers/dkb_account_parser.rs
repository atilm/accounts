use crate::parsers::*;
use csv;

pub struct DkbAccountParser {}

impl BankStatementParserImplementation for DkbAccountParser {
    fn get_header_parser(&self) -> BankStatementHeaderParser {
        BankStatementHeaderParser {
            header_length: 6,
            account_number_regex: r#""Kontonummer:";"(?P<account>[A-Z\d]+) / Girokonto";"#
                .to_string(),
            balance_amount_regex: r#"(?P<amount>[+-]?[\d,.]+) EUR"#.to_string(),
            parse_amount: parse_float,
            balance_date_regex: r#"Kontostand vom (?P<date>[\d.]+)"#.to_string(),
            account_type: AccountType::DKBAccount,
        }
    }

    fn parse_record(&self, record: &csv::StringRecord) -> Result<AccountRecord, ParserError> {
        Ok(AccountRecord {
            amount: parse_float(&record[7]).unwrap(),
            date: parse_date(&record[0])?,
            other_side: Some(record[3].to_string()),
            booking_text: record[2].to_string(),
            purpose: Some(record[4].to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{BankStatementParser, DkbAccountParser};
    use crate::model::{AccountRecord, AccountType};
    use approx::assert_relative_eq;
    use chrono::NaiveDate;

    fn given_a_dkb_account_statement_file() -> std::fs::File {
        std::fs::File::open("./src/parsers/testData/dkb_account_statement.csv")
            .expect("Could not open file.")
    }

    #[test]
    fn an_account_file_can_be_parsed_correctly() {
        let file = given_a_dkb_account_statement_file();

        let parser = BankStatementParser {
            implementation: Box::new(DkbAccountParser {}),
        };

        let parser_result = parser.parse(file).unwrap();

        let expected_records = vec![
            AccountRecord {
                amount: 0.97,
                date: chrono::NaiveDate::from_ymd_opt(2024, 9, 4).unwrap(),
                other_side: Some("VISA-CARD GELDANLAGE".to_string()),
                booking_text: "UMBUCHUNG".to_string(),
                purpose: Some("4930 0000 2699 0595 AUSGLEICHSBUCHUNG".to_string()),
            },
            AccountRecord {
                amount: -60.01,
                date: chrono::NaiveDate::from_ymd_opt(2024, 9, 2).unwrap(),
                other_side: Some("EDEKA.BERGER".to_string()),
                booking_text: "Kartenzahlung".to_string(),
                purpose: Some("2024-08-31      Debitk.63 VISA Debit".to_string()),
            },
        ];

        assert_eq!(parser_result.account_name, "DE08120300001234567890");
        assert_eq!(parser_result.account_type, AccountType::DKBAccount);
        assert_relative_eq!(parser_result.current_balance, 10123.45);
        assert_eq!(
            parser_result.current_balance_date,
            NaiveDate::from_ymd_opt(2024, 9, 4).unwrap()
        );
        assert_eq!(parser_result.records, expected_records);
    }
}
