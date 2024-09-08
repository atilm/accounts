use accountslib::parsers::{dkb_account_parser::DkbAccountParser, BankStatementParser};
use plotters::prelude::*;

fn main() {
    let parser = BankStatementParser {
        implementation: Box::new(DkbAccountParser {}),
    };

    let my_account_history = parser.parse("/home/andreas/Dokumente/Konten/1018793511.csv").unwrap();

    let our_account_history = parser.parse("/home/andreas/Dokumente/Konten/1050155058.csv").unwrap();

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
