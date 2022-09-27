use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver};
use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, Timelike, Utc};
use chrono_tz::Europe::Kiev;
use serde::{self, Serialize, Deserialize};

#[derive(Clone)]
pub struct Clock {
    prev_time: NaiveDateTime,
    next_time: NaiveDateTime,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            prev_time: Self::get_current_datetime(),
            next_time: Self::get_current_datetime(),
        }
    }

    pub async fn check_clock(&mut self) -> bool {
        let now = Self::get_current_datetime();
        if self.next_time < now {
            self.prev_time = now;
            true
        } else {
            false
        }
    }
    
    pub fn get_current_datetime() -> NaiveDateTime {
        Utc::now().with_timezone(&Kiev).naive_utc()
    }

    pub fn set_next_update(&mut self, next_time: NaiveDateTime) {
        self.next_time = next_time;
    }

    pub fn add_interval_time_to_time(time: NaiveDateTime, interval_time: NaiveTime) -> NaiveDateTime {
        time
            + Duration::hours(interval_time.hour() as i64)
            + Duration::minutes(interval_time.minute() as i64)
            + Duration::seconds(interval_time.second() as i64)
    }
    
    
}