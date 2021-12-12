use cached::proc_macro::cached;
use rocket::get;

use crate::constants::NOT_IMPLEMENTED;
use crate::moex;
use crate::spbex;

#[get("/<exchange>/<ticker>")]
pub async fn quotes(exchange: String, ticker: String) -> String {
    cached_quotes(exchange.to_lowercase(), ticker.to_lowercase()).await
}

#[cached(time = 300, sync_writes = true)]
async fn cached_quotes(exchange: String, ticker: String) -> String {
    match exchange.as_str() {
        "moex" => {
            // let data = moex::get_security_parameters(&ticker).await.unwrap();
            // format!("{:?}", data)
            let data = moex::get_ticker(&ticker).await.unwrap();
            serde_json::to_string(&data).unwrap()
        }
        "spbex" => {
            let data = spbex::get_ticker(&ticker).await.unwrap();
            serde_json::to_string(&data).unwrap()
        }
        _ => NOT_IMPLEMENTED.to_owned(),
    }
}
