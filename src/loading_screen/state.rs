use amethyst::{
    assets::{
        AssetStorage, Loader, ProgressCounter, Handle, Progress, Completion, RonFormat, Tracker,
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
    keep_open: Option<Box<dyn Tracker>>, // amethyst does not expose the required type, using dynamic dispatch to get around this.
}

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        create_camera(world);
        let spritesheet = load_spritesheet(world, "boardgamepack/dice/diceRed", &mut self.progress);
        
        world.insert(AssetStorage::<GameSettings>::new());

        let settings = load_settings(world, "config", &mut self.progress);
        
        

        let font = load_font(world, "kenneyfonts/Kenney Future Narrow.ttf", &mut self.progress);

        self.settings = Some(settings);
        self.spritesheet = Some(spritesheet);
        self.font = Some(font);

        <&mut ProgressCounter as Progress>::add_assets(&mut &mut self.progress, 1); // amethyst has an unchecked subtraction so we're increasing this value to get around it.
        let keep_open = self.progress.create_tracker();
        self.keep_open = Some(Box::new(keep_open));
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
            Completion::Loading => {
                let spritesheet = &self.spritesheet;
                if let Some(spritesheet) = spritesheet {
                    let complete = self.loading_sprites.len();
                    if complete == 6 {
                        let last = self.keep_open.take().unwrap();
                        last.success();
                    } else if complete < 6 {
                        let number = complete as f32;
                        let x = number * 64.0 - 160.0;
                        let y = 32.0;
                        self.loading_sprites.push(create_sprite(data.world, &spritesheet, complete, x, y));
                    }
                }
                Trans::None
            },
        }
    }
}

fn load_settings(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Handle<GameSettings> {
    let loader = world.read_resource::<Loader>();
    let store = world.read_resource::<AssetStorage<GameSettings>>();
    let settings = loader.load(
        format!("{}.ron", name),
        RonFormat,
        &mut *progress,
        &store,
    );
    settings
}

fn load_spritesheet(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Handle<SpriteSheet> {
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
    handle
}

fn load_font(world: &mut World, name: &str, progress: &mut ProgressCounter ) -> Handle<FontAsset>  {
    world.read_resource::<Loader>().load(
        name,
        TtfFormat,
        (),
        &world.read_resource(),
    )
}

impl InitialState {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(InitialState::new())
    }
}
