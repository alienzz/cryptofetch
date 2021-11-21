use colored::{ColoredString, Colorize};
use reqwest::Error;
use serde::Deserialize;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CoinData {
    id: String,
    rank: String,
    name: String,
    price_usd: String,
    change_percent_24_hr: String,
}

#[derive(Deserialize, Debug)]
struct Data {
    data: Vec<CoinData>,
}

fn parse_to_usd(price_string: String, change: String) -> ColoredString {
    let price_usd = price_string.parse::<f32>().unwrap();
    let change_direction = change.parse::<f32>().unwrap();
    let trunced = f32::trunc(price_usd * 100.0) / 100.0;
    if change_direction > 0.0 {
        trunced.to_string().green()
    } else {
        trunced.to_string().red()
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.coincap.io/v2/assets?limit=10");
    let response = reqwest::get(&request_url).await?.json::<Data>().await?;
    let mut table = Table::new();
    table.add_row(row!["Rank", "Name", "Price"]);

    for i in response.data {
        let i: CoinData = i;
        table.add_row(row![
            format!("#{}", i.rank),
            i.name,
            parse_to_usd(i.price_usd, i.change_percent_24_hr)
        ]);
    }
    table.printstd();
    Ok(())
}
