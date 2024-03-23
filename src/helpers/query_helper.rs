use time::Date;

pub fn get_archive_schema_name(date: impl Into<Date>) -> String {
    format!("archive_{}", date.into().year())
}

pub fn get_dynamic_table_name(table_name: &str, date: impl Into<Date>) -> String {
    let date = date.into();
    format!(
        "{table_name}_{}_{}",
        date.year(),
        get_double_digit_month(date)
    )
}

pub fn get_double_digit_month(date: Date) -> String {
    let month = date.month() as u8;

    if month > 9 {
        return month.to_string();
    };

    format!("0{month}")
}

pub fn get_bet_table_name(provider: &impl AsRef<str>) -> String {
    format!("bet_{}", provider.as_ref())
}
