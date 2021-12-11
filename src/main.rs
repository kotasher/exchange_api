#[macro_use]
extern crate rocket;
use cached::proc_macro::cached;
use chrono::prelude::DateTime;
use chrono::Local;
use itertools::izip;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static NOT_IMPLEMENTED: &str = "Not implemented";
static ROOT: &str = "/";

#[cached(time = 300, sync_writes = true)]
async fn cached_quotes(exchange: String, ticker: String) -> String {
    match exchange.as_str() {
        "spbex" => {
            let data = get_ticker(&ticker).await.unwrap();
            serde_json::to_string(&data).unwrap()
        }
        _ => NOT_IMPLEMENTED.to_owned(),
    }
}

#[get("/<exchange>/<ticker>")]
async fn quotes(exchange: String, ticker: String) -> String {
    cached_quotes(exchange.to_lowercase(), ticker.to_lowercase()).await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(ROOT, routes![quotes])
        .register(ROOT, catchers!(not_found))
}

#[catch(404)]
fn not_found() -> String {
    NOT_IMPLEMENTED.to_owned()
}

pub async fn get_ticker(ticker: &str) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let data = get_ticker_data(ticker).await?;
    let mut out: Vec<HistoryEntry> = Vec::with_capacity(data.t.len());

    for (t, h, l, c) in izip!(&data.t, &data.h, &data.l, &data.c) {
        out.push(HistoryEntry {
            date: convert_timestamp(*t),
            close: *c,
            high: *h,
            low: *l,
            volume: 0,
        });
    }
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
    let mut text = response.text().await?;
    text = text.replace("\\", "");
    text = text[1..text.len() - 1].to_owned();
    let data: SpbexResponse = serde_json::from_str(&text).unwrap();
    Ok(data)
}

#[derive(Debug, Serialize)]
pub struct HistoryEntry {
    date: String,
    close: f64,
    high: f64,
    low: f64,
    volume: u128,
}
