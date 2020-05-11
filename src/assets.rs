use amethyst::{
    assets::{
        Asset, AssetStorage, Completion, Format, Handle, Loader, Progress, ProgressCounter,
        RonFormat, Tracker,
    },
    core::ecs::Entity,
    ecs::prelude::Dispatcher,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::mesh::ObjFormat, ImageFormat, Mesh, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{FontAsset, TtfFormat},
    window::ScreenDimensions,
    DataDispose, DataInit, GameDataBuilder,
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

