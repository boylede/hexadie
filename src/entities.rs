use amethyst::{
    assets::{
        AssetStorage, Loader, ProgressCounter, Handle, Progress, Completion, RonFormat,
    },
    core::{transform::Transform, ecs::Entity},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontAsset},
};

pub fn create_sprite(world: &mut World, spritesheet: &Handle<SpriteSheet>, which: usize, x: f32, y: f32) -> Entity {

    let sprite_sheet_handle = spritesheet.clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: which,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);

    world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .build()
}

pub fn create_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(1600.0, 1600.0))
        .with(transform)
        .build();
}