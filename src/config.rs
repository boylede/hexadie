use amethyst::{
    assets::{
        Handle, 
        Asset,
        ProcessingState,
    },
    Result,
    core::ecs::DenseVecStorage,
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GameSettings {
    width: u32,
    height: u32,
    area_size: u32,
}

impl Asset for GameSettings {
    const NAME: &'static str = "hexadie::GameSettings";
    type Data = Self;
    type HandleStorage = DenseVecStorage<Handle<GameSettings>>;
}

impl From<GameSettings> for Result<ProcessingState<GameSettings>> {
    fn from(settings: GameSettings) -> Result<ProcessingState<GameSettings>> {
        Ok(ProcessingState::Loaded(settings))
    }
}
