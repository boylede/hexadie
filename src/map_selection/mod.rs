use amethyst::{
    assets::{
        Handle,
        Loader,
        AssetStorage,
    },
    core::{
        ecs::{
            Entity,
            DenseVecStorage,
            Component,
            Entities,
            WriteStorage
        },
        transform::{
            Transform
        }
    },
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{SpriteSheet, Mesh, MaterialDefaults, Material, SpriteRender, Transparent},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontAsset, UiButtonBuilder, Interactable, UiEvent},
};

use crate::config::GameSettings;
use crate::entities::create_sprite;
use crate::assets::HexAssets;

pub struct MapSelectionState {
}

impl SimpleState for MapSelectionState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let assets = (*world.read_resource::<HexAssets>()).clone();

        world.register::<Hexagon>();

        create_title_text(world, &assets.font, "Map Selection");
        create_map(world);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(_event) = get_key(&event) {
                //
            }
        }
        if let StateEvent::Ui(event) = &event {
            println!("got ui event");
            use amethyst::ui::UiEventType::*;
            let UiEvent{event_type, target} = event;
            match event_type {
                Click => {
                    println!("Clicked! {:?}", target);
                },
                HoverStart => {

                },
                HoverStop => {

                }
                _ => {
                    //
                }
            }
        }
        Trans::None
    }
}

impl MapSelectionState {
    pub fn new() -> Self {
        MapSelectionState {}
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(MapSelectionState::new())
    }
}

fn create_title_text(world: &mut World, font: &Handle<FontAsset>, text: &str) -> Entity {
    let transform = UiTransform::new(
        text.to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        0.0, -100.0, 1.0, 800.0, 75.0,
    );

    let text = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            text.to_string(),
            [0.1, 0.1, 0.1, 1.0],
            96.0,
        )).build();

        text
}

fn create_map(world: &mut World) {

    let assets = (*world.read_resource::<HexAssets>()).clone();

    let mut settings: GameSettings = {
        let handle = &assets.settings;
        let asset_storage =  world.read_resource::<AssetStorage<GameSettings>>();
        asset_storage.get(handle).expect("failed to load settings").clone()
    };

    let player_count = settings.player_count.unwrap_or(8);
    let territory_count = player_count * 4;

    let width = settings.width;
    let height = settings.height;

    let area_size = if settings.area_size == 0 { 1 } else {settings.area_size};

    let hex_count = territory_count * settings.area_size;
    let overall_area = width * height;

    let hex_offset_y = 56.0;
    let hex_offset_x = 49.0;
    let scale = 1.0;



    println!("making map {}x{} = {}, with {} hexes in {} territories", width, height, overall_area, hex_count, territory_count);

    let mut hex_list = vec![];
    // world.exec(|(mut entities, mut hexagons): (Entities, WriteStorage<Hexagon>)| {
    for x in 0..width {
        for y in 0..height {
            let even_column_offset: (f32, f32) = if x % 2 == 0 {
                (-0.5, 28.0)
            } else {
                (0.0, 0.0)
            };
            let x = x as f32;
            let y = y as f32;
            
            // entities.build_entity()
            let sprite_sheet_handle = assets.hex_sprites.clone();

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            };
            let mut transform = Transform::default();
            transform.set_translation_xyz(x * hex_offset_x + even_column_offset.0, y * hex_offset_y + even_column_offset.1, 0.0);
            transform.set_scale([scale, scale, scale].into());
            // transform.set_rotation_2d(rotation / 180.0 * PI);

            let hexagon = Hexagon {
                x: x,
                y: y,
                init: None,
            };

            let mut sprite_builder = world
                .create_entity()
                .with(sprite_render)
                .with(transform)
                .with(hexagon)
                .with(Transparent);
            // if let Some((r, g, b)) = tint {
            //     sprite_builder = sprite_builder.with(Tint(Srgb::new(r, g, b).into()));
            // }
            let hex = sprite_builder.build();
            hex_list.push(hex);
        }
    }        
    // });

    



}

struct Hexagon {
    x: f32,
    y: f32,
    init: Option<InitializedHexagon>,
}

struct InitializedHexagon {
    player: Player,
    neighbors: [Entity; 6],
    center: Entity,
}


impl Component for Hexagon {
    type Storage = DenseVecStorage<Self>;
}

enum Player {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}