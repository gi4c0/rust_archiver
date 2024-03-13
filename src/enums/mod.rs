use parse_display::Display;
use serde::{Deserialize, Serialize};

pub mod bet;

#[repr(u8)]
#[derive(Clone, Deserialize, Serialize, Debug, Copy)]
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
    #[display("en")]
    English,
    #[serde(rename = "th")]
    #[display("th")]
    Thai,
    #[serde(rename = "zh")]
    #[display("zh")]
    Chinese,
    #[serde(rename = "ms")]
    #[display("ms")]
    Malay,
    #[serde(rename = "id")]
    #[display("id")]
    Indonesian,
    #[serde(rename = "lo")]
    #[display("lo")]
    Laotian,
    #[serde(rename = "vi")]
    #[display("vi")]
    Vietnamese,
    #[serde(rename = "tl")]
    #[display("tl")]
    Tagalog,
    #[serde(rename = "hi")]
    #[display("hi")]
    Hindi,
    #[serde(rename = "ko")]
    #[display("ko")]
    Korean,
    #[serde(rename = "ja")]
    #[display("ja")]
    Japanese,
}
