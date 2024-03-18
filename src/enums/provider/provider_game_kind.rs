use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, Eq)]
pub enum ProviderGameKind {
    #[serde(rename = "baccarat")]
    #[display("baccarat")]
    Baccarat,
    #[serde(rename = "dragontiger")]
    #[display("dragontiger")]
    DragonTiger,

    #[serde(rename = "sicbo")]
    #[display("sicbo")]
    SicBo,

    #[serde(rename = "roulette")]
    #[display("roulette")]
    Roulette,

    #[serde(rename = "bull")]
    #[display("bull")]
    Bull,

    #[serde(rename = "three_cards")]
    #[display("three_cards")]
    ThreeCards,

    #[serde(rename = "red_blue_duel")]
    #[display("red_blue_duel")]
    RedBlueDuel,

    #[serde(rename = "slot")]
    #[display("slot")]
    Slot,

    #[serde(rename = "fishing")]
    #[display("fishing")]
    Fishing,

    #[serde(rename = "poker")]
    #[display("poker")]
    Poker,

    #[serde(rename = "blackjack")]
    #[display("blackjack")]
    BlackJack,

    #[serde(rename = "joker")]
    #[display("joker")]
    Joker,

    #[serde(rename = "online")]
    #[display("online")]
    Online,

    #[serde(rename = "multiplayer")]
    #[display("multiplayer")]
    Multiplayer,

    #[serde(rename = "fan_tan")]
    #[display("fan_tan")]
    FatTan,
}
