use time::{macros::time, Date, OffsetDateTime, Time};

pub fn get_hong_kong_11_hours() -> OffsetDateTime {
    OffsetDateTime::now_utc().replace_time(time!(3:00))
}

pub fn get_hong_kong_11_hours_from_date(date: Date) -> OffsetDateTime {
    OffsetDateTime::new_utc(date, Time::from_hms(3, 0, 0).unwrap())
}
