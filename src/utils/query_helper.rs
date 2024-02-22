use time::Date;

pub fn get_archive_schema_name(date: impl Into<Date>) -> String {
    format!("archive_{}", date.into().year())
}

pub fn get_dynamic_table_name(table_name: &str, date: impl Into<Date>) -> String {
    format!(
        "{table_name}_{}_{}",
        date.into().year(),
        get_double_digit_month(date.into())
    )
}

fn get_double_digit_month(date: Date) -> String {
    let month = date.month() as u8;

    if month > 9 {
        return format!("{month}");
    };

    format!("0{month}")
}
