use anyhow::{anyhow, Result};
use parse_display::Display;

#[derive(Display)]
#[display(style = "snake_case")]
pub enum GameProvider {
    Sexy,
    Ameba,
    Arcadia,
    KingMaker,
    Pragmatic,
    RoyalSlotGaming,
}

impl TryFrom<String> for GameProvider {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "sexy" => Ok(Self::Sexy),
            "ameba" => Ok(Self::Ameba),
            "arcadia" => Ok(Self::Arcadia),
            "king_maker" => Ok(Self::KingMaker),
            "pragmatic" => Ok(Self::Pragmatic),
            "royal_slot_gaming" => Ok(Self::RoyalSlotGaming),
            _ => Err(anyhow!("Unexpected provider name: '{value}'")),
        }
    }
}
