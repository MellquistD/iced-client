use std::time::Duration;

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use iced::{Color, Size};

///
pub struct Candle {
    pub(crate) open: f32,
    pub(crate) high: f32,
    pub(crate) low: f32,
    pub(crate) close: f32,
    pub(crate) opentime: u32,
}

impl Candle {
    pub fn oc_size(&self) -> f32 {
        self.close - self.open
    }
    pub fn hl_size(&self) -> f32 {
        self.high - self.low
    }
    pub fn top_wick_size(&self) -> f32 {
        self.high - self.open.max(self.close)
    }
    pub fn bot_wick_size(&self) -> f32 {
        self.open.min(self.close) - self.low
    }

    pub fn color(&self) -> Color {
        if self.open < self.close {
            Color::from_rgb(0.0, 255.0, 0.0)
        } else {
            Color::from_rgb(255.0, 0.0, 0.0)
        }
    }
}

pub struct CandleFactory;

impl CandleFactory {
    pub fn create_candles(amount: usize) -> Vec<Candle> {
        let start_time = 0;
        let candle_length = 1;
        let mut previous_close = 0.0;

        let mut result = Vec::new();

        for i in 0..amount {
            let opentime = start_time + (candle_length * i) as u32;
            let open = previous_close;
            let close = previous_close + 25.0;
            let high = ((open + close) / 2.0) + 25.0;
            let low = ((open + close) / 2.0) - 25.0;

            result.push(Candle {
                open,
                high,
                low,
                close,
                opentime,
            });
            previous_close = close;
        }
        result
    }
}
