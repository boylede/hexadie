use amethyst::{
    assets::{
        Asset, AssetStorage, Completion, Format, Handle, Loader, Progress, ProgressCounter,
        RonFormat, Tracker,
    },
    core::ecs::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        formats::mesh::ObjFormat, ImageFormat, Mesh, SpriteSheet, SpriteSheetFormat, Texture,
    },
    ui::{FontAsset, TtfFormat},
    window::ScreenDimensions,
};

use crate::assets::HexAssets;
use crate::config::GameSettings;
use crate::entities::{create_camera, create_sprite};
use crate::main_menu::MainMenuState;

/// The initial state will load any needed assets, and set them up in the world as needed. It will display a progress bar and loading text. Once loading is complete, we pass to the main menu state.
#[derive(Default)]
pub struct InitialState {
    progress: ProgressCounter,
    loading_sprites: Vec<Entity>,
    spritesheet: Option<Handle<SpriteSheet>>,
    keep_open: Option<Box<dyn Tracker>>, // amethyst does not expose the required type, using dynamic dispatch to get around this.
}

impl SimpleState for InitialState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let camera = create_camera(world);

        let dice_texture = load_asset::<Texture, ImageFormat>(
            world,
            "boardgamepack/dice/diceRed.png",
            ImageFormat::default(),
            &mut self.progress,
        );
        let dice_spritesheet = load_asset::<SpriteSheet, SpriteSheetFormat>(
            world,
            "boardgamepack/dice/diceRed.ron",
            SpriteSheetFormat(dice_texture),
            &mut self.progress,
        );

        let hex_texture = load_asset::<Texture, ImageFormat>(
            world,
            "hexes/hexes.png",
            ImageFormat::default(),
            &mut self.progress,
        );
        let hex_spritesheet = load_asset::<SpriteSheet, SpriteSheetFormat>(
            world,
            "hexes/hexes.ron",
            SpriteSheetFormat(hex_texture),
            &mut self.progress,
        );

        world.insert(AssetStorage::<GameSettings>::new());

        let settings = load_asset(world, "config.ron", RonFormat, &mut self.progress);

        let font = load_asset(
            world,
            "kenneyfonts/Kenney Future Narrow.ttf",
            TtfFormat,
            &mut self.progress,
        );

        self.spritesheet = Some(dice_spritesheet.clone());

        <&mut ProgressCounter as Progress>::add_assets(&mut &mut self.progress, 1);
        let keep_open = self.progress.create_tracker();
        self.keep_open = Some(Box::new(keep_open));
        let assets: HexAssets =  HexAssets {
            spritesheet: dice_spritesheet,
            font,
            settings,
            hex_sprites: hex_spritesheet,
            camera: camera,
        };
        world.insert(assets);
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
                    data.world
                        .delete_entity(*ent)
                        .expect("Tried to delete entities twice.");
                }

                Trans::Switch(MainMenuState::new_boxed())
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
                        self.loading_sprites.push(create_sprite(
                            data.world,
                            &spritesheet,
                            complete,
                            x,
                            y,
                            None,
                            0.0,
                            1.0,
                        ));
                    }
                }
                Trans::None
            }
        }
    }
}

fn load_asset<A, F>(
    world: &mut World,
    name: &str,
    format: F,
    progress: &mut ProgressCounter,
) -> Handle<A>
where
    A: Asset,
    F: Format<<A as Asset>::Data>,
{
    let loader = world.read_resource::<Loader>();
    let store = world.read_resource::<AssetStorage<A>>();
    let asset = loader.load(name, format, &mut *progress, &store);
    asset
}
impl InitialState {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(InitialState::new())
    }
}
