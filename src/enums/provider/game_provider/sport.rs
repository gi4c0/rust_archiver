use strum_macros::{AsRefStr, EnumString, VariantArray};

use super::GameProvider;

#[derive(AsRefStr, Debug, EnumString, VariantArray, Copy, Clone, PartialEq, Eq, Hash)]
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

impl Sportsbook {
    pub fn into_game_provider(self) -> GameProvider {
        GameProvider::Sport(self)
    }
}
