use strum::VariantArray;

use crate::enums::provider::{
    GameProvider, LiveCasinoProvider, OnlineCasinoProvider, SlotProvider, Sportsbook,
};

pub fn get_game_providers() -> Vec<GameProvider> {
    let providers: Vec<GameProvider> = [
        LiveCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect(),
        OnlineCasinoProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect(),
        SlotProvider::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect(),
        Sportsbook::VARIANTS
            .into_iter()
            .map(|p| p.into_game_provider())
            .collect::<Vec<GameProvider>>(),
    ]
    .concat();

    providers
}
