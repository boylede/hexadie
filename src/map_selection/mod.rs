use amethyst::{
    assets::{
        Handle,
        Loader,
    },
    core::{ecs::{Entity, DenseVecStorage, storage::UnprotectedStorage}, transform::{Transform}},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{SpriteSheet, Mesh, MaterialDefaults, Material},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontAsset, UiButtonBuilder, Interactable, UiEvent},
};

use crate::config::GameSettings;
use crate::entities::create_sprite;

pub struct MapSelectionState {
    spritesheet: Handle<SpriteSheet>,
    settings: Handle<GameSettings>,
    font: Handle<FontAsset>,
    hex_sprites: Handle<SpriteSheet>,
    // models: Vec<Handle<Mesh>>,
}

impl SimpleState for MapSelectionState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        create_title_text(world, &self.font, "TESTADIE");
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
    pub fn new(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>, hex_sprites: Handle<SpriteSheet>) -> Self {
        MapSelectionState {
            spritesheet,
            settings,
            font,
            hex_sprites,
            // models,
        }
    }
    pub fn new_boxed(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>, hex_sprites: Handle<SpriteSheet>) -> Box<Self> {
        Box::new(MapSelectionState::new(spritesheet, settings, font, hex_sprites))
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