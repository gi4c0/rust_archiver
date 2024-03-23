use strum_macros::{AsRefStr, EnumString, VariantArray};

use super::GameProvider;

#[derive(AsRefStr, Debug, EnumString, VariantArray, Clone)]
pub enum LiveCasinoProvider {
    #[strum(serialize = "sexy")]
    Sexy,
    #[strum(serialize = "pragmatic")]
    Pragmatic,
    #[strum(serialize = "sa")]
    SA,
    #[strum(serialize = "ag")]
    AG,
    #[strum(serialize = "pretty")]
    Pretty,
    #[strum(serialize = "dream")]
    Dream,
    #[strum(serialize = "allbet")]
    AllBet,
    #[strum(serialize = "xtream")]
    Xtream,
    #[strum(serialize = "bet_games_live_casino")]
    BetGames,
    #[strum(serialize = "big_gaming_live_casino")]
    BigGaming,
    #[strum(serialize = "mg_live_casino")]
    MG,
    #[strum(serialize = "green_dragon")]
    GreenDragon,
    #[strum(serialize = "wm_live_casino")]
    WM,
}

impl LiveCasinoProvider {
    pub fn into_game_provider(self) -> GameProvider {
        GameProvider::LiveCasino(self)
    }
}
