use amethyst::{
    assets::{
        AssetStorage, Loader, ProgressCounter, Handle,
    },
    core::{transform::Transform},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};

use crate::main_menu::MainMenuState;
use crate::config::GameSettings;

/// The initial state will load any needed assets, and set them up in the world as needed. It will display a progress bar and loading text. Once loading is complete, we pass to the main menu state.

#[derive(Default)]
pub struct InitialState {
    progress: ProgressCounter,
    spritesheet: Option<Handle<SpriteSheet>>,
    settings: Option<GameSettings>,
}

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        create_camera(world);
        self.spritesheet = load_spritesheet(world, "boardgamepack/dice/diceRed");

        create_sprite(world, &self.spritesheet, 1, -300.0, 158.0);
        create_sprite(world, &self.spritesheet, 5, 200.0, 500.0);
        create_sprite(world, &self.spritesheet, 2, -153.0, -264.0);
        create_sprite(world, &self.spritesheet, 2, 183.0, -184.0);
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
        }
        Trans::Switch(MainMenuState::new_boxed())
    }
}

fn load_spritesheet(world: &mut World, name: &str) -> Option<Handle<SpriteSheet>> {
    
    let handle = {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("{}.png", name),
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.ron", name),
            SpriteSheetFormat(texture_handle),
            (),
            &sprite_sheet_store,
        )
    };
    Some(handle)
}

fn create_sprite(world: &mut World, spritesheet: &Option<Handle<SpriteSheet>>, which: usize, x: f32, y: f32) {

    let sprite_sheet_handle = spritesheet.as_ref().unwrap().clone();

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
        .build();
}

fn create_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(1600.0, 1600.0))
        .with(transform)
        .build();
}


impl InitialState {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(InitialState::new())
    }
}


