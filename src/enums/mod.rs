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
