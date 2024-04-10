use strum_macros::{AsRefStr, Display, EnumString, VariantArray};

#[derive(sqlx::Type, Clone, Copy, Display, AsRefStr, VariantArray, EnumString)]
#[sqlx(rename_all = "UPPERCASE", type_name = "bet_status_enum")]
#[strum(serialize_all = "UPPERCASE")]
pub enum BetStatus {
    Active,
    Pending,
    Closed,
    Cancelled,
    Suspended,
    Void,
}
