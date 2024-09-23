use std::{
    fs::{self},
    str::FromStr,
};

use accountslib::{
    accounts_reading::merge_rule_reading::read_merge_rules,
    model::{
        account_history::AccountHistory,
        monthly_report::MonthlyReports,
        record_merging::{merge_records, merge_records_from_date},
        AccountRecord,
    },
    parsers::{
        dkb_account_parser::DkbAccountParser, parser_factory::ParserFactory, BankStatementParser,
        ParserError,
    },
};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use plotters::prelude::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Plot,
    Balance {
        dir_path: String,
        report_path: Option<String>,
        start_date: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Plot => plot_accounts(),
        Commands::Balance {
            dir_path,
            report_path,
            start_date,
        } => generate_balance_sheet(
            &dir_path,
            &report_path.unwrap_or("./balance".to_string()),
            start_date,
        ),
    }
}

fn generate_balance_sheet(dir_path: &str, report_path: &str, start_date: Option<String>) {
    let dir_entries = fs::read_dir(dir_path).expect("Could not list files in dir {dir_path}");

    let own_account_rules_file = "own_account_rules.json";

    let file_paths: Vec<String> = dir_entries
        .into_iter()
        .filter_map(|r| Some(r.unwrap()))
        .filter(|r| r.path().is_file())
        .filter(|r| r.file_name() != own_account_rules_file)
        .map(|r| String::from_str(&r.path().to_str().unwrap()).unwrap())
        .collect();

    let account_histories: Vec<AccountHistory> = file_paths
        .iter()
        .map(|path| {
            let parser = ParserFactory::create(path);
            match parser {
                Ok(p) => p.parse(path),
                Err(_) => Err(ParserError::FileReadError),
            }
        })
        .filter_map(|r| r.ok())
        .collect();

    let own_account_rules_file_path = std::path::Path::new(dir_path).join(own_account_rules_file);

    let own_account_rules = read_merge_rules(own_account_rules_file_path.to_str().unwrap())
        .expect("Could not read merge rules");

    let all_records: Vec<Vec<AccountRecord>> =
        account_histories.into_iter().map(|h| h.records).collect();

    let start_date = start_date.map(|s| NaiveDate::parse_from_str(s.as_ref(), "%d.%m.%Y"));

    let merged_records = match start_date {
        Some(start_date) => {
            merge_records_from_date(all_records, own_account_rules, start_date.unwrap())
        }
        None => merge_records(all_records, own_account_rules),
    };

    let monthly_reports = MonthlyReports::create(merged_records);

    let report_contents = format!(
        "Average Earnings: {}
Average Spendings: {}

{}",
        monthly_reports.average_earnings(),
        monthly_reports.average_spendings(),
        monthly_reports
            .reports
            .into_iter()
            .rev()
            .map(|r| format!("{r}"))
            .collect::<String>()
    );

    fs::write(report_path, report_contents).expect("Could not write report");
}

fn plot_accounts() {
    let parser = BankStatementParser {
        implementation: Box::new(DkbAccountParser {}),
    };

    let my_account_history = parser
        .parse("/home/andreas/Dokumente/Konten/1018793511.csv")
        .unwrap();

    let our_account_history = parser
        .parse("/home/andreas/Dokumente/Konten/1050155058.csv")
        .unwrap();

    let root_area =
        BitMapBackend::new("/home/andreas/Dokumente/Konten/kontostand.png", (1200, 800))
            .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let start_date = chrono::NaiveDate::from_ymd_opt(2021, 9, 1).unwrap();
    let end_date = chrono::NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(&my_account_history.account_name, ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 0.0..120000.0)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let dates: Vec<chrono::NaiveDate> = my_account_history.records.iter().map(|r| r.date).collect();
    let balances: Vec<(chrono::NaiveDate, f64)> = dates
        .into_iter()
        .map(|d| (d, my_account_history.get_balance_at(d).unwrap()))
        .collect();

    let our_dates: Vec<chrono::NaiveDate> =
        our_account_history.records.iter().map(|r| r.date).collect();
    let our_balances: Vec<(chrono::NaiveDate, f64)> = our_dates
        .into_iter()
        .map(|d| (d, our_account_history.get_balance_at(d).unwrap()))
        .collect();

    ctx.draw_series(
        AreaSeries::new(
            balances,      // The data iter
            0.0,           // Baseline
            &RED.mix(0.2), // Make the series opac
        )
        .border_style(&RED), // Make a brighter border
    )
    .unwrap();

    ctx.draw_series(
        AreaSeries::new(
            our_balances,   // The data iter
            0.0,            // Baseline
            &BLUE.mix(0.2), // Make the series opac
        )
        .border_style(&BLUE), // Make a brighter border
    )
    .unwrap();
}
