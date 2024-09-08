use super::*;

#[derive(Debug, Error, PartialEq)]
pub enum AccountHistoryError {
    #[error("Date out of bounds.")]
    DateOutOfBounds
}

pub struct AccountHistory {
    pub account_name: String,
    pub account_type: AccountType,
    pub current_balance_date: NaiveDate,
    pub current_balance: f64,
    pub records : Vec<AccountRecord>,
}

impl AccountHistory {
    pub fn get_balance_at(&self, date: NaiveDate) -> Result<f64, AccountHistoryError> {
        
        let mut current_balance = self.current_balance;
        for record in self.records.iter() {
            if date >= record.date {
                return Ok(current_balance)
            }

            current_balance -= record.amount;
        }

        Err(AccountHistoryError::DateOutOfBounds)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    use approx::assert_relative_eq;
    use chrono::Duration;

    fn given_a_history() -> AccountHistory {
        AccountHistory {
            account_name: String::from_str("1018793511").unwrap(),
            account_type: AccountType::DKBAccount,
            current_balance_date: NaiveDate::from_ymd_opt(2024, 3, 6).unwrap(),
            current_balance: 350.0,
            records : vec![
                new_record(300.0, NaiveDate::from_ymd_opt(2024, 3, 5).unwrap()),
                new_record(-50.0, NaiveDate::from_ymd_opt(2024, 3, 3).unwrap()),
                new_record(100.0, NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()),  
            ],
        }
    }

    fn new_record(amount: f64, date: NaiveDate) -> AccountRecord {
        AccountRecord {
            amount,
            date,
            ..AccountRecord::default()
        }
    }

    #[test]
    fn get_account_balances_at_historic_dates() {
        let history = given_a_history();
        let later_date = history.current_balance_date + Duration::days(1);
        assert_relative_eq!(history.get_balance_at(later_date).unwrap(), 350.0);
        assert_relative_eq!(history.get_balance_at(history.current_balance_date).unwrap(), 350.0);
        assert_relative_eq!(history.get_balance_at(history.records[1].date).unwrap(), 50.0);
        assert_relative_eq!(history.get_balance_at(history.records[2].date).unwrap(), 100.0);
        assert_relative_eq!(history.get_balance_at(NaiveDate::from_ymd_opt(2024, 3, 2).unwrap()).unwrap(), 100.0);
    }

    #[test]
    fn get_account_balance_before_first_records_returns_error() {
        let history = given_a_history();

        let too_early_date = history.records[2].date - Duration::days(1);
        assert_eq!(history.get_balance_at(too_early_date), Err(AccountHistoryError::DateOutOfBounds))
    }
}