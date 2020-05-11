use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        ecs::{Component, DenseVecStorage, Entities, Entity, Join, ReadStorage, WriteStorage},
        transform::Transform,
    },
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        palette::Srgb, resources::Tint, Camera, Material, MaterialDefaults, Mesh, SpriteRender,
        SpriteSheet, Transparent,
    },
    ui::{
        Anchor, FontAsset, Interactable, TtfFormat, UiButtonBuilder, UiEvent, UiText, UiTransform,
    },
    window::ScreenDimensions,
};

use rand::{thread_rng, Rng};
use std::collections::HashMap;

use crate::assets::HexAssets;
use crate::config::GameSettings;
use crate::entities::create_sprite;

pub struct MapSelectionState {}

impl SimpleState for MapSelectionState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let assets = (*world.read_resource::<HexAssets>()).clone();

        world.register::<Hexagon>();
        world.register::<Player>();

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
            let UiEvent { event_type, target } = event;
            match event_type {
                Click => {
                    println!("Clicked! {:?}", target);
                }
                HoverStart => {}
                HoverStop => {}
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
        text.to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        0.0,
        -100.0,
        1.0,
        800.0,
        75.0,
    );

    let text = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            text.to_string(),
            [0.1, 0.1, 0.1, 1.0],
            96.0,
        ))
        .build();

    text
}

fn create_map(world: &mut World) {
    let assets = (*world.read_resource::<HexAssets>()).clone();

    let mut settings: GameSettings = {
        let handle = &assets.settings;
        let asset_storage = world.read_resource::<AssetStorage<GameSettings>>();
        asset_storage
            .get(handle)
            .expect("failed to load settings")
            .clone()
    };

    let player_count = settings.player_count.unwrap_or(8);
    let territory_count = player_count * 4;

    let width = settings.width;
    let height = settings.height;

    let area_size = if settings.area_size == 0 {
        1
    } else {
        settings.area_size
    };

    let hex_count = territory_count * settings.area_size;
    let overall_area = width * height;

    let hex_offset_y = 56.0;
    let hex_offset_x = 49.0;
    let scale = 1.0;

    {
        let camera = assets.camera;
        world.exec(
            |(mut transforms, cameras): (WriteStorage<Transform>, ReadStorage<Camera>)| {
                for (mut transform, camera) in (&mut transforms, &cameras).join() {
                    let width = width as f32 * hex_offset_x * scale;
                    let height = height as f32 * hex_offset_y * scale;
                    transform.set_translation_xyz(width / 2.0, height / 2.0, 1.0);
                }
            },
        );
    }

    println!(
        "making map {}x{} = {}, with {} hexes in {} territories({}-sized)",
        width, height, overall_area, hex_count, territory_count, area_size
    );

    // generate a number of hexagon entities
    // these will be combined into groups of area_size hexes
    // called areas
    let mut hex_list = vec![];
    // world.exec(|(mut entities, mut hexagons): (Entities, WriteStorage<Hexagon>)| {
    for x in 0..width {
        for y in 0..height {
            let even_column_offset: (f32, f32) = if x % 2 == 0 { (-0.5, 28.0) } else { (0.0, 0.0) };
            let x = x as f32;
            let y = y as f32;

            // entities.build_entity()
            let sprite_sheet_handle = assets.hex_sprites.clone();

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet_handle,
                sprite_number: 0,
            };
            let mut transform = Transform::default();
            let x = (x * hex_offset_x + even_column_offset.0) * scale;
            let y = (y * hex_offset_y + even_column_offset.1) * scale;
            transform.set_translation_xyz(x, y, 0.0);
            transform.set_scale([scale, scale, scale].into());
            // transform.set_rotation_2d(rotation / 180.0 * PI);

            let hexagon = Hexagon {
                x: x,
                y: y,
                init: None,
            };

            let mut hex_builder = world
                .create_entity()
                .with(sprite_render)
                .with(transform)
                .with(hexagon)
                .with(Tint(Srgb::new(0.2, 0.2, 0.2).into()))
                .with(Transparent);
            // if let Some((r, g, b)) = tint {
            // hex_builder = hex_builder.with(Tint(Srgb::new(0.2, 0.2, 0.2).into()));
            // }
            let hex = hex_builder.build();
            hex_list.push(hex);
        }
    }

    // randomly select territory_count hexes to seed the map
    let mut rng = thread_rng();
    let mut players = world.write_storage::<Player>();
    println!("randomly placing initial areas");
    let mut areas: HashMap<Entity, (u32, Player, bool)> = HashMap::new();
    for country in 0..territory_count {
        let x = rng.gen_range(0, width) as usize;
        let y = rng.gen_range(0, height) as usize;
        println!(
            "picking area {},{}, which is index {} out of {}",
            x,
            y,
            y * width as usize + x,
            hex_list.len()
        );
        let hex = hex_list.get(y * width as usize + x).unwrap();
        println!("initializing area {}", country);

        let player = match country % player_count {
            0 => Player::One,
            1 => Player::Two,
            2 => Player::Three,
            3 => Player::Four,
            4 => Player::Five,
            5 => Player::Six,
            6 => Player::Seven,
            7 => Player::Eight,
            // 8 => Player::Nine,
            _ => panic!("modulo overran allowed player count"),
        };
        players.insert(*hex, player);
        areas.insert(*hex, (1, player, false));
    }

    // flood fill the map until each area has area_size hexes
    while areas
        .iter()
        .any(|(_origin, (size, _player, blocked))| *size < area_size && !blocked)
    {
        let mut working_areas = areas
            .iter()
            .filter(|(_origin, (size, _player, blocked))| *size < area_size && !blocked)
            .collect::<Vec<(&Entity, &(u32, Player, bool))>>();
        working_areas.sort_by(|a, b| a.0.cmp(b.0));
        let next_area = working_areas.iter().next();
        // get this area's neighbor hexes

        // check if they are occupied
        // add one of them to this area
        // if all are occupied, mark this area as blocked
    }

    // color each hex according to which player it represents
    let mut tints = world.write_storage::<Tint>();
    // let tint = tints.get_mut(*hex);
    for (mut tint, player) in (&mut tints, &players).join() {
        use Player::*;
        let color = match player {
            One => Srgb::new(0.306, 0.804, 0.769),
            Two => Srgb::new(0.780, 0.957, 0.392),
            Three => Srgb::new(1.0, 0.420, 0.420),
            Four => Srgb::new(0.769, 0.302, 0.345),
            Five => Srgb::new(0.333, 0.384, 0.439),
            Six => Srgb::new(0.286, 0.039, 0.239),
            Seven => Srgb::new(0.741, 0.082, 0.314),
            Eight => Srgb::new(0.914, 0.498, 0.008),
            // Nine => Srgb::new(0.973, 0.792, 0.0),
            // Ten => Srgb::new(0.541, 0.608, 0.059),
        };
        *tint = Tint(color.into());

        println!("tint: {:?}", tint);
    }
}

/// the Hexagon component is the basic building block of the game board.
/// 
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

/// The Player component will enumerate which player any enitity (just hexes for now) belongs to.
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

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
/// The Area component will store the id of the area to which each hex belongs.
struct Area {
    id: usize,
}

impl Component for Area {
    type Storage = DenseVecStorage<Self>;
}
