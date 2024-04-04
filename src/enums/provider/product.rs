use strum_macros::{AsRefStr, Display};

#[derive(PartialEq, Eq, Clone, Copy, Debug, AsRefStr, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    LiveCasino,
    Slot,
    OnlineCasino,
    Sportsbook,
    Lottery,
}
