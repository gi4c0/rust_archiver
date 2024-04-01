use strum_macros::{AsRefStr, Display, VariantNames};

#[derive(sqlx::Type, Clone, Copy, Display, AsRefStr, VariantNames)]
#[sqlx(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum BetStatus {
    Active,
    Pending,
    Closed,
    Cancelled,
    Suspended,
    Void,
}
