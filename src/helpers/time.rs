use time::{macros::time, Date, Duration, Month, OffsetDateTime, Time};

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

pub fn get_figures_date(bet_date: OffsetDateTime) -> Date {
    let threshold = bet_date.replace_time(time!(3:00));

    if bet_date >= threshold {
        return (threshold + Duration::days(1)).date();
    }

    threshold.date()
}

pub fn subtract_one_month(date: Date) -> Date {
    let month = date.month();
    let mut year = date.year();

    if month == Month::January {
        year -= 1;
    }

    let month = month.previous();

    Date::from_calendar_date(year, month, 1).unwrap()
}
