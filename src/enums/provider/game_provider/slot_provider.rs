use strum_macros::{AsRefStr, EnumString, VariantArray};

use super::GameProvider;

#[derive(AsRefStr, Debug, EnumString, VariantArray, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SlotProvider {
    #[strum(serialize = "slot_xo_slot")]
    SlotXO,
    #[strum(serialize = "pg_slot")]
    PG,
    #[strum(serialize = "evoplay_slot")]
    Evoplay,
    #[strum(serialize = "simpleplay")]
    SimplePlay,
    #[strum(serialize = "spade")]
    Spade,
    #[strum(serialize = "ambpoker_slot")]
    AmbPoker,
    #[strum(serialize = "ambslot_slot")]
    AmbSlot,
    #[strum(serialize = "pragmatic_slot")]
    Pragmatic,
    #[strum(serialize = "dragoon_slot")]
    Dragoon,
    #[strum(serialize = "jili_slot")]
    Jili,
    #[strum(serialize = "mannaplay")]
    MannaPlay,
    #[strum(serialize = "royal_slot_gaming")]
    RoyalSlotGaming,
    #[strum(serialize = "mg_slot")]
    MG,
    #[strum(serialize = "upg_slot")]
    UPG,
    #[strum(serialize = "gaming_soft_slot")]
    GamingSoft,
    #[strum(serialize = "ygg_slot")]
    YGG,
    #[strum(serialize = "slotmill")]
    Slotmill,
    #[strum(serialize = "habanero_slot")]
    Habanero,
    #[strum(serialize = "astro_tech_slot")]
    AstroTech,
    #[strum(serialize = "wazdan")]
    Wazdan,
    #[strum(serialize = "all_way_spin_slot")]
    AllWaySpin,
    #[strum(serialize = "booongo")]
    Booongo,
    #[strum(serialize = "funta_slot")]
    FunTa,
    #[strum(serialize = "iconic_gaming_slot")]
    IconicGaming,
    #[strum(serialize = "gamatron")]
    Gamatron,
    #[strum(serialize = "ka_gaming_slot")]
    KaGaming,
    #[strum(serialize = "play_son")]
    PlaySon,
    #[strum(serialize = "wm_slot")]
    WM,
    #[strum(serialize = "ameba")]
    Ameba,
    #[strum(serialize = "i8")]
    I8,
    #[strum(serialize = "creative_gaming")]
    CreativeGaming,
    #[strum(serialize = "red_tiger_slot")]
    RedTiger,
    #[strum(serialize = "netent_slot")]
    Netent,
    #[strum(serialize = "relax")]
    Relax,
    #[strum(serialize = "ninja_slot")]
    Ninja,
    #[strum(serialize = "kiss_slot")]
    Kiss,
    #[strum(serialize = "spinix_slot")]
    Spinix,
    #[strum(serialize = "dragon_gaming")]
    DragonGaming,
    #[strum(serialize = "hacksaw")]
    Hacksaw,
}

impl SlotProvider {
    pub fn into_game_provider(self) -> GameProvider {
        GameProvider::Slot(self)
    }
}
