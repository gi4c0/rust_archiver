use parse_display::Display;

#[derive(Display, sqlx::Type)]
#[display(style = "UPPERCASE")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum BetStatus {
    Active,
    Pending,
    Closed,
    Cancelled,
    Suspended,
    Void,
}
