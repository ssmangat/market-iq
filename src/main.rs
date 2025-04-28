use yahoo_finance_api as yahoo;
use chrono::{TimeZone, Utc};
use ta::indicators::SimpleMovingAverage;
use ta::{DataItem, Next};

#[derive(Debug, Clone)]
struct Candle {
    date: String,
    close: f64,
}

#[tokio::main]
async fn main() {
    let symbol = "VFV.TO"; 
    let days_back = 365;

    match fetch_candles(symbol, days_back).await {
        Some(candles) if !candles.is_empty() => {
            signal_200_day_ma(&candles);
            signal_50_200_crossover(&candles);
            signal_10_month_ma(&candles);
            signal_dual_ma(&candles, 10, 50);
            signal_ma_slope(&candles);
        }
        _ => println!("Failed to fetch candle data or no data returned."),
    }
}

// --- Fetch Historical Data ---
async fn fetch_candles(symbol: &str, days: i64) -> Option<Vec<Candle>> {
    let provider = yahoo::YahooConnector::new().ok()?;

    let end_date = yahoo::time::OffsetDateTime::now_utc();
    let start_date = end_date - yahoo::time::Duration::days(days);

    let response = provider.get_quote_history(symbol, start_date, end_date).await.ok()?;
    let quotes = response.quotes().ok()?;

    let candles: Vec<Candle> = quotes
        .iter()
        .filter_map(|q| {
            // Create DateTime<Utc> using timestamp_opt()
            let dt = Utc.timestamp_opt(q.timestamp as i64, 0)
                .single()
                .expect("Invalid timestamp");
            Some(Candle {
                date: dt.to_string(),
                close: q.close,
            })
        })
        .collect();
    println!("Fetched {} candles.", candles.len());

    if candles.is_empty() {
        None
    } else {
        Some(candles)
    }
}


// Helper function to build a valid DataItem using the builder
fn create_data_item(candle: &Candle) -> Option<DataItem> {
    DataItem::builder()
        .open(candle.close)
        .high(candle.close)
        .low(candle.close)
        .close(candle.close)
        .volume(0.0)
        .build()
        .ok()
}

// Helper to determine if we are in the last 5 days
fn is_last_five(index: usize, total: usize) -> bool {
    index >= total.saturating_sub(5)
}

// --- Signal 1: 200-Day Simple Moving Average ---  
fn signal_200_day_ma(candles: &[Candle]) {
    println!("--- Signal 1: 200-Day Moving Average (Last 5 Days) ---");

    if candles.len() < 200 {
        println!("Not enough data to calculate 200-day MA.");
        return;
    }

    match SimpleMovingAverage::new(200) {
        Ok(mut sma) => {
            for (i, candle) in candles.iter().enumerate() {
                if let Some(item) = create_data_item(candle) {
                    let ma = sma.next(&item);
                    // Print only if we are processing the last 5 days.
                    if is_last_five(i, candles.len()) {
                        println!("{}: Close: {:.2}, MA: {:.2}", candle.date, candle.close, ma);
                        if candle.close > ma {
                            println!("{}: BUY", candle.date);
                        } else {
                            println!("{}: SELL", candle.date);
                        }
                    }
                } else {
                    if is_last_five(i, candles.len()) {
                        println!("Failed to build DataItem for candle: {}", candle.date);
                    }
                }
            }
        }
        Err(e) => println!("Failed to create SMA: {}", e),
    }
}

// --- Signal 2: 50/200-Day Crossover ---
fn signal_50_200_crossover(candles: &[Candle]) {
    println!("--- Signal 2: 50/200-Day Crossover (Last 5 Days) ---");
    if let (Ok(mut sma_short), Ok(mut sma_long)) =
        (SimpleMovingAverage::new(50), SimpleMovingAverage::new(200))
    {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let short = sma_short.next(&item);
                let long = sma_long.next(&item);

                if is_last_five(i, candles.len()) {
                    if short > long {
                        println!("{}: BUY (Golden Cross) [Short: {:.2}, Long: {:.2}]", candle.date, short, long);
                    } else {
                        println!("{}: SELL (Death Cross) [Short: {:.2}, Long: {:.2}]", candle.date, short, long);
                    }
                }
            }
        }
    }
}

// --- Signal 3: 10-Month (~210 trading days) Moving Average ---
fn signal_10_month_ma(candles: &[Candle]) {
    println!("--- Signal 3: 10-Month Moving Average (Last 5 Days) ---");
    if let Ok(mut sma) = SimpleMovingAverage::new(210) {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let ma = sma.next(&item);
                if is_last_five(i, candles.len()) {
                    println!("{}: Close: {:.2}, MA: {:.2}, Signal: {}", candle.date, candle.close, ma,
                        if candle.close > ma { "HOLD (Invested)" } else { "CASH (Exit)" });
                }
            }
        }
    }
}

// --- Signal 4: Dual MA (10/50 example) ---
fn signal_dual_ma(candles: &[Candle], short_period: usize, long_period: usize) {
    println!(
        "--- Signal 4: Dual Moving Averages {} / {} (Last 5 Days) ---",
        short_period, long_period
    );
    if let (Ok(mut sma_short), Ok(mut sma_long)) = (
        SimpleMovingAverage::new(short_period),
        SimpleMovingAverage::new(long_period),
    ) {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let short = sma_short.next(&item);
                let long = sma_long.next(&item);

                if is_last_five(i, candles.len()) {
                    if short > long {
                        println!("{}: BUY (Short MA {:.2} above Long MA {:.2})", candle.date, short, long);
                    } else {
                        println!("{}: SELL (Short MA {:.2} below Long MA {:.2})", candle.date, short, long);
                    }
                }
            }
        }
    }
}

// --- Signal 5: Slope of 200-day MA ---
fn signal_ma_slope(candles: &[Candle]) {
    println!("--- Signal 5: Moving Average Slope (Last 5 Days) ---");
    if let Ok(mut sma) = SimpleMovingAverage::new(200) {
        let mut prev_ma: Option<f64> = None;

        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let ma = sma.next(&item);

                if is_last_five(i, candles.len()) {
                    match prev_ma {
                        Some(prev) if ma > prev => println!("{}: TREND UP (BUY) [MA: {:.2} > {:.2}]", candle.date, ma, prev),
                        Some(prev) if ma < prev => println!("{}: TREND DOWN (SELL) [MA: {:.2} < {:.2}]", candle.date, ma, prev),
                        Some(_) => println!("{}: HOLD", candle.date),
                        None => println!("{}: Initial MA: {:.2}", candle.date, ma),
                    }
                }

                prev_ma = Some(ma);
            }
        }
    }
}
