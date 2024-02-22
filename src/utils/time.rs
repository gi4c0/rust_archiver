use time::{macros::time, OffsetDateTime};

pub fn get_hong_kong_11_hours() -> OffsetDateTime {
    OffsetDateTime::now_utc().replace_time(time!(3:00))
}
