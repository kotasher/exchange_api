use chrono::prelude::DateTime;
use chrono::Local;
use itertools::izip;
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::history::HistoryEntry;

pub async fn get_ticker(ticker: &str) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let data = get_ticker_data(ticker).await?;
    let out = izip!(&data.t, &data.h, &data.l, &data.c)
                                        .map(|(t, h, l, c)| 
                                            HistoryEntry {
                                                date: convert_timestamp(*t), 
                                                close: *c,
                                                high: *h,
                                                low: *l,
                                                volume: 0
                                            })
                                        .collect();
    Ok(out)
}

struct SpbexDurationRange {
    start: u128,
    end: u128,
}

#[derive(Debug, Deserialize)]
struct SpbexResponse {
    t: Vec<i64>,
    // o: Vec<f64>,
    h: Vec<f64>,
    l: Vec<f64>,
    c: Vec<f64>,
    // s: String,
}

fn get_ranges() -> SpbexDurationRange {
    SpbexDurationRange {
        start: 1434014660 as u128,
        end: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            / 1000,
    }
}

fn get_url(ticker: &str, resolution: &str, range: &SpbexDurationRange) -> String {
    format!(
        "{base_url}/chistory?symbol={symbol}&resolution={resolution}&from={from}&to={to}",
        base_url = "https://investcab.ru/api",
        symbol = ticker,
        resolution = resolution,
        from = range.start,
        to = range.end,
    )
}

fn get_url_day_resolution(ticker: &str, range: &SpbexDurationRange) -> String {
    get_url(ticker, "D", range)
}

fn convert_timestamp(timestamp: i64) -> String {
    let date = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
    let datetime = DateTime::<Local>::from(date);
    datetime.format("%d.%m.%Y").to_string()
}

async fn get_ticker_data(ticker: &str) -> Result<SpbexResponse, Box<dyn std::error::Error>> {
    let range = get_ranges();
    let url = get_url_day_resolution(ticker, &range);
    let response = reqwest::get(&url).await?;
    let text = response
                            .text()
                            .await?
                            .replace("\\", "");
    let data: SpbexResponse = serde_json::from_str(&text[1..text.len() - 1]).unwrap();
    Ok(data)
}