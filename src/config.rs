use amethyst::{
    assets::{
        Handle, 
        Asset,
    },
    core::ecs::DenseVecStorage,
};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
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
