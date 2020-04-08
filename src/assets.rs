use amethyst::{
    DataDispose, DataInit, GameDataBuilder,
    assets::{
        Asset, Format,
        AssetStorage, Loader, ProgressCounter, Handle, Progress, Completion, RonFormat, Tracker,
    },
    core::{ecs::Entity},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture, Mesh, formats::mesh::ObjFormat},
    window::ScreenDimensions,
    ui::{TtfFormat,  FontAsset},
    ecs::prelude::Dispatcher,
};

use crate::config::GameSettings;

#[derive(Clone)]
pub struct HexAssets {
    pub spritesheet: Handle<SpriteSheet>,
    pub font: Handle<FontAsset>,
    pub settings: Handle<GameSettings>,
    pub hex_sprites: Handle<SpriteSheet>,
    pub camera: Entity,
}

