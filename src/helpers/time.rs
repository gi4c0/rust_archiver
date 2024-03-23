use time::{macros::time, Date, Month, OffsetDateTime, Time};

pub fn get_hong_kong_11_hours() -> OffsetDateTime {
    OffsetDateTime::now_utc().replace_time(time!(3:00))
}

pub fn get_hong_kong_11_hours_from_date(date: Date) -> OffsetDateTime {
    OffsetDateTime::new_utc(date, Time::from_hms(3, 0, 0).unwrap())
}

pub fn add_month(date: Date) -> Date {
    match date.month() {
        Month::December => {
            Date::from_calendar_date(date.year() + 1, date.month().next(), 1).unwrap()
        }
        _ => date.replace_month(date.month().next()).unwrap(),
    }
}
