use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HistoryEntry {
    pub date: String,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u128,
}
