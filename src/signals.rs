use crate::utils::{create_data_item, is_last_n_days, Candle};
use colored::*;
use ta::indicators::SimpleMovingAverage;
use ta::Next; // Import colored for printing colored text

pub fn signal_200_day_ma(candles: &[Candle], last_n_days: usize) {
    if candles.len() < 200 {
        println!("Not enough data to calculate 200-day MA.");
        return;
    }

    if let Ok(mut sma) = SimpleMovingAverage::new(200) {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let ma = sma.next(&item);
                if is_last_n_days(i, candles.len(), last_n_days) {
                    print!(
                        "{}: Close: {:.2}, MA: {:.2} ",
                        candle.date, candle.close, ma
                    );
                    if candle.close > ma {
                        println!("{}", "BUY".green()); // Green for BUY
                    } else {
                        println!("{}", "SELL".red()); // Red for SELL
                    }
                }
            }
        }
    }
}

pub fn signal_50_200_crossover(candles: &[Candle], last_n_days: usize) {
    if let (Ok(mut sma_short), Ok(mut sma_long)) =
        (SimpleMovingAverage::new(50), SimpleMovingAverage::new(200))
    {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let short = sma_short.next(&item);
                let long = sma_long.next(&item);

                if is_last_n_days(i, candles.len(), last_n_days) {
                    print!(
                        "{}: Short MA: {:.2}, Long MA: {:.2} ",
                        candle.date, short, long
                    );
                    if short > long {
                        println!("{}", "BUY (Golden Cross)".green()); // Green for BUY
                    } else {
                        println!("{}", "SELL (Death Cross)".red()); // Red for SELL
                    }
                }
            }
        }
    }
}

pub fn signal_10_month_ma(candles: &[Candle], last_n_days: usize) {
    if let Ok(mut sma) = SimpleMovingAverage::new(210) {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let ma = sma.next(&item);
                if is_last_n_days(i, candles.len(), last_n_days) {
                    print!(
                        "{}: Close: {:.2}, MA: {:.2} ",
                        candle.date, candle.close, ma
                    );
                    if candle.close > ma {
                        println!("{}", "HOLD (Invested)".green());
                    } else {
                        println!("{}", "CASH (Exit)".red());
                    }
                }
            }
        }
    }
}

pub fn signal_dual_ma(
    candles: &[Candle],
    short_period: usize,
    long_period: usize,
    last_n_days: usize,
) {
    if let (Ok(mut sma_short), Ok(mut sma_long)) = (
        SimpleMovingAverage::new(short_period),
        SimpleMovingAverage::new(long_period),
    ) {
        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let short = sma_short.next(&item);
                let long = sma_long.next(&item);

                if is_last_n_days(i, candles.len(), last_n_days) {
                    print!(
                        "{}: Short MA: {:.2}, Long MA: {:.2} ",
                        candle.date, short, long
                    );
                    if short > long {
                        println!("{}", "BUY".green());
                    } else {
                        println!("{}", "SELL".red());
                    }
                }
            }
        }
    }
}

pub fn signal_ma_slope(candles: &[Candle], last_n_days: usize) {
    if let Ok(mut sma) = SimpleMovingAverage::new(200) {
        let mut prev_ma: Option<f64> = None;

        for (i, candle) in candles.iter().enumerate() {
            if let Some(item) = create_data_item(candle) {
                let ma = sma.next(&item);

                if is_last_n_days(i, candles.len(), last_n_days) {
                    match prev_ma {
                        Some(prev) if ma > prev => {
                            print!("{}: MA: {:.2}, Prev MA: {:.2} ", candle.date, ma, prev);
                            println!("{}", "TREND UP (BUY)".green());
                        }
                        Some(prev) if ma < prev => {
                            print!("{}: MA: {:.2}, Prev MA: {:.2} ", candle.date, ma, prev);
                            println!("{}", "TREND DOWN (SELL)".red());
                        }
                        Some(_) => println!("{}: HOLD", candle.date),
                        None => println!("{}: Initial MA: {:.2}", candle.date, ma),
                    }
                }

                prev_ma = Some(ma);
            }
        }
    }
}
