use serde::{Deserialize, Serialize};
use strum_macros::{Display, VariantArray};

pub mod bet;
pub mod provider;

#[derive(
    Clone, Deserialize, Serialize, Debug, Copy, PartialEq, Eq, sqlx::Type, VariantArray, Display,
)]
#[repr(i16)]
pub enum PositionEnum {
    Owner = 0,
    Company = 1,
    Shareholder = 2,
    Senior = 3,
    MasterAgent = 4,
    Agent = 5,
    Player = 6,
}

impl From<i16> for PositionEnum {
    fn from(value: i16) -> Self {
        match value {
            0 => PositionEnum::Owner,
            1 => PositionEnum::Company,
            2 => PositionEnum::Shareholder,
            3 => PositionEnum::Senior,
            4 => PositionEnum::MasterAgent,
            5 => PositionEnum::Agent,
            6 => PositionEnum::Player,
            _ => panic!("Invalid position value in DB"),
        }
    }
}

#[derive(Serialize, Deserialize, Display)]
pub enum Language {
    #[serde(rename = "en")]
    #[strum(serialize = "en")]
    English,
    #[serde(rename = "th")]
    #[strum(serialize = "th")]
    Thai,
    #[serde(rename = "zh")]
    #[strum(serialize = "zh")]
    Chinese,
    #[serde(rename = "ms")]
    #[strum(serialize = "ms")]
    Malay,
    #[serde(rename = "id")]
    #[strum(serialize = "id")]
    Indonesian,
    #[serde(rename = "lo")]
    #[strum(serialize = "lo")]
    Laotian,
    #[serde(rename = "vi")]
    #[strum(serialize = "vi")]
    Vietnamese,
    #[serde(rename = "tl")]
    #[strum(serialize = "tl")]
    Tagalog,
    #[serde(rename = "hi")]
    #[strum(serialize = "hi")]
    Hindi,
    #[serde(rename = "ko")]
    #[strum(serialize = "ko")]
    Korean,
    #[serde(rename = "ja")]
    #[strum(serialize = "ja")]
    Japanese,
}
