use chrono::{Duration, Datelike, DateTime, Utc, TimeZone, Weekday};
use chrono_tz::Europe::Warsaw;
use chrono_tz::Tz;
use lazy_static::lazy_static;

pub fn get_summer_start(year: i32) -> DateTime<Tz> {
    let mut start = Warsaw.ymd(year, 6, 1).and_hms(0, 0, 0);
    let offset = 27 - 1 - start.weekday().num_days_from_sunday() as i64;
    start += Duration::days(offset);
    start
}

pub fn get_summer_end(year: i32) -> DateTime<Tz> {
    let mut end = Warsaw.ymd(year, 9, 1).and_hms(0, 0, 0);
    if end.weekday() != Weekday::Mon {
        let offset = (7 + Weekday::Mon.num_days_from_monday() as i64 - end.weekday().num_days_from_monday() as i64) % 7;
        end += Duration::days(offset);
    }
    end
}

lazy_static! {
    static ref SUMMER_START: DateTime<Tz> = {
        let now = Utc::now().with_timezone(&Warsaw);
        let summer_start = if now > get_summer_end(now.year()) {
            get_summer_start(now.year() + 1)
        } else {
            get_summer_start(now.year())
        };
        summer_start
    };

    static ref SUMMER_END: DateTime<Tz> = {
        let now = Utc::now().with_timezone(&Warsaw);
        let summer_end = get_summer_end(now.year());
        summer_end
    };
}

pub async fn get_time_remaining() -> (i64, i64, i64, i64, bool) {
    let now = Utc::now().with_timezone(&Warsaw);
    let (target_date, to_end) = if now < *SUMMER_START {
        (*SUMMER_START, false)
    } else {
        (*SUMMER_END, true)
    };

    let duration = target_date - now;

    let days = duration.num_days();
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    (days, hours, minutes, seconds, to_end)
}
