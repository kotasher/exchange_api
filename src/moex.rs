use crate::history::HistoryEntry;
use crate::utils::current_local_time_formatted;
use std::io::{Error, ErrorKind};

static BASE_URL: &'static str = "https://iss.moex.com";

pub async fn get_ticker(ticker: &str) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let security = get_security_parameters(ticker).await?;
    println!("{:?}", security);

    let mut start = 0;
    let mut out: Vec<HistoryEntry> = Vec::new();

    loop {
        let url = get_security_history_url(ticker, &security, start);
        let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

        let data = response["history"]["data"].as_array().unwrap();
        if data.len() == 0 {
            break;
        }

        for entry in data {
            out.push(HistoryEntry {
                date: convert_datetime(entry[0].as_str().unwrap().to_owned()),
                close: entry[1].as_f64().unwrap(),
                high: entry[2].as_f64().unwrap(),
                low: entry[3].as_f64().unwrap(),
                volume: entry[4].as_u64().unwrap() as u128,
            });
        }

        start += 100;
    }
    Ok(out)
}

fn get_security_parameters_url(ticker: &str) -> String {
    format!(
        "{base_url}/iss/securities/{ticker}.json?iss.only=boards&iss.meta=off&boards.columns=boardid,market,engine,is_primary",
        base_url = BASE_URL,
        ticker = ticker,
    )
}

#[derive(Debug)]
struct MoexSecurityParameters {
    board: String,
    market: String,
    engine: String,
}

async fn get_security_parameters(
    ticker: &str,
) -> Result<MoexSecurityParameters, Box<dyn std::error::Error>> {
    let url = get_security_parameters_url(ticker);
    let response = reqwest::get(url).await?.json::<serde_json::Value>().await?;

    for entry in response["boards"]["data"][0].as_array() {
        if entry[3] == 1 {
            return Ok(MoexSecurityParameters {
                board: entry[0].to_string().replace("\"", ""),
                market: entry[1].to_string().replace("\"", ""),
                engine: entry[2].to_string().replace("\"", ""),
            });
        }
    }
    let err = Box::new(Error::new(ErrorKind::InvalidData, "no primary board found"));
    Err(err)
}

struct MoexDurationRange {
    start: String,
    end: String,
}

fn get_ranges() -> MoexDurationRange {
    MoexDurationRange {
        start: "2015-01-01".to_owned(),
        end: current_local_time_formatted("%Y-%m-%d"),
    }
}

fn get_security_history_url(
    ticker: &str,
    security: &MoexSecurityParameters,
    start: usize,
) -> String {
    let range = get_ranges();
    format!(
        "{base_url}/iss/history/engines/{engine}/markets/{market}/boards/{board}/securities/{ticker}.json?from={from}&till={till}&start={start}&iss.meta=off&history.columns=TRADEDATE,CLOSE,HIGH,LOW,VOLUME",
        base_url = BASE_URL,
        engine = security.engine,
        market = security.market,
        board = security.board,
        ticker = ticker,
        from = range.start,
        till = range.end,
        start = start,
    )
}

fn convert_datetime(input: String) -> String {
    let split = input.split("-").collect::<Vec<&str>>();
    format!(
        "{day}.{month}.{year}",
        day = split[2],
        month = split[1],
        year = split[0],
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_security_parameters_url_test() {
        use crate::moex::get_security_parameters_url;
        let left = "https://iss.moex.com/iss/securities/sber.json?iss.only=boards&iss.meta=off&boards.columns=boardid,market,engine,is_primary";
        let right = get_security_parameters_url("sber");
        assert_eq!(left, right);
    }

    #[test]
    fn get_security_history_url_test() {
        use crate::moex::{get_ranges, get_security_history_url, MoexSecurityParameters};
        let security = MoexSecurityParameters {
            board: "TQBR".to_owned(),
            market: "shares".to_owned(),
            engine: "stock".to_owned(),
        };
        let range = get_ranges();
        let left = format!(
            "https://iss.moex.com/iss/history/engines/stock/markets/shares/boards/TQBR/securities/sber.json?from={from}&till={till}&start=0&iss.meta=off&history.columns=TRADEDATE,CLOSE,HIGH,LOW,VOLUME",
            from = range.start,
            till = range.end,
        );
        let right = get_security_history_url("sber", &security, 0);
        assert_eq!(left, right);
    }

    #[test]
    fn convert_datetime_test() {
        use crate::moex::convert_datetime;
        let left = "2015-01-01".to_string();
        let right = "01.01.2015";
        assert_eq!(convert_datetime(left), right);
    }
}
