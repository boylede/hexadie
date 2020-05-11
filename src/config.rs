use amethyst::{
    assets::{Asset, AssetStorage, Handle, ProcessingState},
    core::{
        bundle::SystemBundle,
        ecs::{prelude::*, DenseVecStorage, Entity},
    },
    Error, Result,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct GameSettings {
    pub width: u32,
    pub height: u32,
    pub area_size: u32,
    pub player_count: Option<u32>,
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

pub struct GameSettingsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameSettingsBundle {
    fn build(self, world: &mut World, _dispatcher: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        world.insert(AssetStorage::<GameSettings>::new());
        Ok(())
    }
}
