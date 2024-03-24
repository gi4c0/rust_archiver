use strum_macros::{AsRefStr, EnumString, VariantArray};

use super::GameProvider;

#[derive(AsRefStr, Debug, EnumString, VariantArray, Copy, Clone)]
pub enum OnlineCasinoProvider {
    #[strum(serialize = "evoplay_online_casino")]
    Evoplay,
    #[strum(serialize = "bet_games_online_casino")]
    BetGames,
    #[strum(serialize = "pg_online_casino")]
    PG,
    #[strum(serialize = "ambpoker_online_casino")]
    AmbPoker,
    #[strum(serialize = "slot_xo_online_casino")]
    SlotXO,
    #[strum(serialize = "dragoon_online_casino")]
    Dragoon,
    #[strum(serialize = "jili_online_casino")]
    Jili,
    #[strum(serialize = "mg_online_casino")]
    MG,
    #[strum(serialize = "gaming_soft_online_casino")]
    GamingSoft,
    #[strum(serialize = "ambslot_online_casino")]
    AmbSlot,
    #[strum(serialize = "habanero_online_casino")]
    Habanero,
    #[strum(serialize = "astro_tech_online_casino")]
    AstroTech,
    #[strum(serialize = "iconic_gaming_online_casino")]
    IconicGaming,
    #[strum(serialize = "funta_online_casino")]
    FunTa,
    #[strum(serialize = "all_way_spin_online_casino")]
    AllWaySpin,
    #[strum(serialize = "ka_gaming_online_casino")]
    KaGaming,
    #[strum(serialize = "kingmaker")]
    Kingmaker,
    #[strum(serialize = "netent_online_casino")]
    Netent,
    #[strum(serialize = "red_tiger_online_casino")]
    RedTiger,
    #[strum(serialize = "arcadia")]
    Arcadia,
    #[strum(serialize = "ninja_online_casino")]
    Ninja,
    #[strum(serialize = "kiss_online_casino")]
    Kiss,
}

impl OnlineCasinoProvider {
    pub fn into_game_provider(self) -> GameProvider {
        GameProvider::OnlineCasino(self)
    }
}
