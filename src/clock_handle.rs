
use chrono::{Duration, NaiveDateTime, NaiveTime, Timelike, Utc};
use chrono_tz::Europe::Kiev;

pub struct Clock {
    interval: NaiveTime,
    prev_time: NaiveDateTime,
}

impl Clock {
    pub fn new(interval: NaiveTime) -> Self {
        Self {
            interval,
            prev_time: Self::get_current_datetime(),
        }
    }

    pub async fn check_clock(&mut self) -> bool {
        let interval = self.interval;
        let now = Self::get_current_datetime();
        let next_time = Self::add_interval_time_to_time(self.prev_time.clone(), interval);
        if next_time < now {
            self.prev_time = now;
            true
        } else {
            false
        }
    }
    
    pub fn get_current_datetime() -> NaiveDateTime {
        Utc::now().with_timezone(&Kiev).naive_utc()
    }

    fn add_interval_time_to_time(time: NaiveDateTime, interval_time: NaiveTime) -> NaiveDateTime {
        time
            + Duration::hours(interval_time.hour() as i64)
            + Duration::minutes(interval_time.minute() as i64)
            + Duration::seconds(interval_time.second() as i64)

    }
}