use ta::DataItem;

#[derive(Debug, Clone)]
pub struct Candle {
    pub date: String,
    pub close: f64,
}

pub fn create_data_item(candle: &Candle) -> Option<DataItem> {
    DataItem::builder()
        .open(candle.close)
        .high(candle.close)
        .low(candle.close)
        .close(candle.close)
        .volume(0.0)
        .build()
        .ok()
}

// Check if the current index is within the last n days
pub fn is_last_n_days(index: usize, total: usize, n: usize) -> bool {
    index >= total.saturating_sub(n)
}
