use crate::parsers::*;
use csv;

pub struct DkbCreditCardParser {}

impl DkbCreditCardParser {
    pub fn can_parse(file_path: &str) -> Result<bool, ParserError>
    {
        let mut decoder = get_decoded_file_reader(file_path);

        let mut buf = String::new();
        decoder.read_to_string(&mut buf).map_err(|_| ParserError::FileReadError)?;

        Ok(buf.contains(r#""Kreditkarte:";"#))
    }
}

impl BankStatementParserImplementation for DkbCreditCardParser {
    fn get_header_parser(&self) -> BankStatementHeaderParser {
        BankStatementHeaderParser {
            header_length: 7,
            account_number_regex: r#""Kreditkarte:";"(?P<account>[\d*]+)";"#
                .to_string(),
            balance_amount_regex: r#"(?P<amount>[+-]?[\d,.]+) EUR"#.to_string(),
            parse_amount: parse_std_float,
            balance_date_regex: r#""Datum:";"(?P<date>[\d.]+)""#.to_string(),
            account_type: AccountType::DKBCreditCard,
        }
    }

    fn parse_record(&self, record: &csv::StringRecord) -> Result<AccountRecord, ParserError> {
        Ok(AccountRecord {
            amount: parse_float(&record[4]).unwrap(),
            date: parse_date(&record[1])?,
            other_side: None,
            booking_text: record[3].to_string(),
            purpose: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::{AccountRecord, AccountType}, parsers::parser_factory::ParserFactory};
    use approx::assert_relative_eq;
    use chrono::NaiveDate;

    const FILE_PATH: &str = "./src/parsers/testData/dkb_credit_card_statement.csv";

    #[test]
    fn an_account_file_can_be_parsed_correctly() {
        let parser = ParserFactory::create(FILE_PATH).unwrap();

        let parser_result = parser.parse(FILE_PATH).unwrap();

        let expected_records = vec![
            AccountRecord {
                amount: 0.6,
                date: chrono::NaiveDate::from_ymd_opt(2024, 1, 23).unwrap(),
                other_side: None,
                booking_text: "HabenzinsenZ 000000432 T 018   0000".to_string(),
                purpose: None,
            },
            AccountRecord {
                amount: -2400.0,
                date: chrono::NaiveDate::from_ymd_opt(2024, 1, 11).unwrap(),
                other_side: None,
                booking_text: "Auszahlung".to_string(),
                purpose: None,
            },
        ];

        assert_eq!(parser_result.account_name, "4930********0595");
        assert_eq!(parser_result.account_type, AccountType::DKBCreditCard);
        assert_relative_eq!(parser_result.current_balance, 0.97);
        assert_eq!(
            parser_result.current_balance_date,
            NaiveDate::from_ymd_opt(2024, 9, 3).unwrap()
        );
        assert_eq!(parser_result.records, expected_records);
    }
}