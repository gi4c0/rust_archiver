use serde::{Deserialize, Serialize};
use strum_macros::{Display, VariantArray};

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq, Clone, Copy, VariantArray)]
pub enum ProviderGameKind {
    #[serde(rename = "baccarat")]
    #[strum(serialize = "baccarat")]
    Baccarat,
    #[serde(rename = "dragontiger")]
    #[strum(serialize = "dragontiger")]
    DragonTiger,

    #[serde(rename = "sicbo")]
    #[strum(serialize = "sicbo")]
    SicBo,

    #[serde(rename = "roulette")]
    #[strum(serialize = "roulette")]
    Roulette,

    #[serde(rename = "bull")]
    #[strum(serialize = "bull")]
    Bull,

    #[serde(rename = "three_cards")]
    #[strum(serialize = "three_cards")]
    ThreeCards,

    #[serde(rename = "red_blue_duel")]
    #[strum(serialize = "red_blue_duel")]
    RedBlueDuel,

    #[serde(rename = "slot")]
    #[strum(serialize = "slot")]
    Slot,

    #[serde(rename = "fishing")]
    #[strum(serialize = "fishing")]
    Fishing,

    #[serde(rename = "poker")]
    #[strum(serialize = "poker")]
    Poker,

    #[serde(rename = "blackjack")]
    #[strum(serialize = "blackjack")]
    BlackJack,

    #[serde(rename = "joker")]
    #[strum(serialize = "joker")]
    Joker,

    #[serde(rename = "online")]
    #[strum(serialize = "online")]
    Online,

    #[serde(rename = "multiplayer")]
    #[strum(serialize = "multiplayer")]
    Multiplayer,

    #[serde(rename = "fan_tan")]
    #[strum(serialize = "fan_tan")]
    FatTan,
}
