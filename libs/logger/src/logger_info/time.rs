use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, Clone)]
pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
        )
    }
}

impl Into<chrono::NaiveDateTime> for Time {
    fn into(self) -> chrono::NaiveDateTime {
        let date = NaiveDate::from_ymd(self.year, self.month, self.day);
        let time = NaiveTime::from_hms(self.hour, self.minute, self.second);
        chrono::NaiveDateTime::new(date, time)
    }
}

impl Time {
    pub fn from_time<D: chrono::Datelike + chrono::Timelike>(dt: D) -> Self {
        Time {
            year: dt.year(),
            month: dt.month(),
            day: dt.day(),
            hour: dt.hour(),
            minute: dt.minute(),
            second: dt.second(),
        }
    }
}
