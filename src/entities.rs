use amethyst::{
    assets::Handle,
    core::{ecs::Entity, transform::Transform},
    prelude::*,
    renderer::{palette::Srgb, resources::Tint, Camera, SpriteRender, SpriteSheet, Transparent},
};

use std::f32::consts::PI;

pub fn create_sprite(
    world: &mut World,
    spritesheet: &Handle<SpriteSheet>,
    which: usize,
    x: f32,
    y: f32,
    tint: Option<(f32, f32, f32)>,
    rotation: f32,
    scale: f32,
) -> Entity {
    let sprite_sheet_handle = spritesheet.clone();

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: which,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(x, y, 0.0);
    transform.set_scale([scale, scale, scale].into());

    transform.set_rotation_2d(rotation / 180.0 * PI);

    let mut sprite_builder = world
        .create_entity()
        .with(sprite_render)
        .with(transform)
        .with(Transparent);
    if let Some((r, g, b)) = tint {
        sprite_builder = sprite_builder.with(Tint(Srgb::new(r, g, b).into()));
    }
    sprite_builder.build()
}

pub fn create_camera(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(1600.0, 1600.0))
        .with(transform)
        .build()
}
