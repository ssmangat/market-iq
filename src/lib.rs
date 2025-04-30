pub mod fetch;
pub mod signals;
pub mod utils;

use fetch::fetch_candles;
use signals::{
    signal_10_month_ma, signal_200_day_ma, signal_50_200_crossover, signal_dual_ma, signal_ma_slope,
};
use std::io::{self, Write};

pub async fn analyze_ticker(
    symbol: &str,
    days_back: i64,
    last_n_days: usize,
) -> Result<(), String> {
    match fetch_candles(symbol, days_back).await {
        Some(candles) if !candles.is_empty() => {
            signal_200_day_ma(&candles, last_n_days);
            signal_50_200_crossover(&candles, last_n_days);
            signal_10_month_ma(&candles, last_n_days);
            signal_dual_ma(&candles, 10, 50, last_n_days);
            signal_ma_slope(&candles, last_n_days);
            Ok(())
        }
        _ => {
            eprintln!(
                "Failed to fetch candle data or no data returned for symbol: {}",
                symbol
            );
            Err("Failed to fetch candle data or no data returned.".to_string())
        }
    }
}

pub fn prompt_ticker() -> String {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            eprintln!("Failed to read user input: {}", e);
            String::new()
        }
    }
}
