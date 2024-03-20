use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, AsRefStr, EnumString)]
pub enum Sportsbook {
    #[strum(serialize = "single_live")]
    SingleLive,
    #[strum(serialize = "single_non_live")]
    SingleNonLive,
    #[strum(serialize = "combo")]
    Combo,
    #[strum(serialize = "parlay")]
    Parlay,
}
