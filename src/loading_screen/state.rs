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

use crate::main_menu::MainMenuState;
use crate::config::GameSettings;
use crate::entities::{create_sprite, create_camera};

/// The initial state will load any needed assets, and set them up in the world as needed. It will display a progress bar and loading text. Once loading is complete, we pass to the main menu state.

#[derive(Default)]
pub struct InitialState {
    progress: ProgressCounter,
    spritesheet: Option<Handle<SpriteSheet>>,
    font: Option<Handle<FontAsset>>,
    settings: Option<Handle<GameSettings>>,
    loading_sprites: Vec<Entity>,
}

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        create_camera(world);
        self.spritesheet = load_spritesheet(world, "boardgamepack/dice/diceRed", &mut self.progress);

        world.insert(AssetStorage::<GameSettings>::new());
        self.settings = load_settings(world, "config", &mut self.progress);

        self.loading_sprites.push(create_sprite(world, &self.spritesheet, 1, -300.0, 158.0));
        self.loading_sprites.push(create_sprite(world, &self.spritesheet, 5, 200.0, 500.0));
        self.loading_sprites.push(create_sprite(world, &self.spritesheet, 2, -153.0, -264.0));
        self.loading_sprites.push(create_sprite(world, &self.spritesheet, 2, 183.0, -184.0));

        self.font = load_font(world, "kenneyfonts/Kenney Future Narrow.ttf", &mut self.progress);
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
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Failed => {
                println!("Failed loading assets: {:?}", self.progress.errors());
                Trans::Quit
            }
            Completion::Complete => {
                for ent in &self.loading_sprites {
                    data.world.delete_entity(*ent);
                }
                Trans::Switch(MainMenuState::new_boxed(
                    self.spritesheet.take().unwrap(),
                    self.settings.take().unwrap(),
                    self.font.take().unwrap(),
                ))
            }

            Completion::Loading => Trans::None,
        }
    }
}

fn load_settings(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Option<Handle<GameSettings>> {
    let loader = world.read_resource::<Loader>();
    let store = world.read_resource::<AssetStorage<GameSettings>>();
    let settings = loader.load(
        format!("{}.ron", name),
        RonFormat,
        &mut *progress,
        &store,
    );
    Some(settings)
}

fn load_spritesheet(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Option<Handle<SpriteSheet>> {
    
    let handle = {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                format!("{}.png", name),
                ImageFormat::default(),
                &mut *progress,
                &texture_storage,
            )
        };

        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            format!("{}.ron", name),
            SpriteSheetFormat(texture_handle),
            &mut *progress,
            &sprite_sheet_store,
        )
    };
    Some(handle)
}

fn load_font(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Option<Handle<FontAsset>>  {
    let font = world.read_resource::<Loader>().load(
        name,
        TtfFormat,
        (),
        &world.read_resource(),
    );

    Some(font)
}

impl InitialState {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(InitialState::new())
    }
}
