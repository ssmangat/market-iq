use crate::utils::Candle;
use chrono::{TimeZone, Utc};
use yahoo_finance_api as yahoo;

pub async fn fetch_candles(symbol: &str, days: i64) -> Option<Vec<Candle>> {
    let provider = yahoo::YahooConnector::new().ok()?;

    let end_date = yahoo::time::OffsetDateTime::now_utc();
    let start_date = end_date - yahoo::time::Duration::days(days);

    let response = provider
        .get_quote_history(symbol, start_date, end_date)
        .await
        .ok()?;

    let quotes = response.quotes().ok()?;

    let candles: Vec<Candle> = quotes
        .iter()
        .filter_map(|q| {
            let dt = Utc.timestamp_opt(q.timestamp as i64, 0).single()?;
            Some(Candle {
                date: dt.to_string(),
                close: q.close,
            })
        })
        .collect();

    if candles.is_empty() {
        None
    } else {
        Some(candles)
    }
}
