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

pub struct MainMenuState {
    spritesheet: Handle<SpriteSheet>,
    settings: Handle<GameSettings>,
    font: Handle<FontAsset>,
    hex_sprites: Handle<SpriteSheet>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        create_test_text(&mut world, &self.font, "TEST TEST TEST TEST");
        // create_sprite(&mut world, &self.spritesheet, 5, 21.0, 300.0);
        // create_sprite(&mut world, &self.spritesheet, 5, 640.0, 21.0);
        // create_sprite(&mut world, &self.spritesheet, 5, -320.0, 90.0);
        create_button(&mut world, "hello world", self.font.clone());
        // create_hexagon(&mut world, 0.0, 0.0, 400.0, self.models[0].clone());

        let colors = [
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.5, 0.5, 0.0),
            (0.0, 0.5, 0.5),
            (0.25, 0.25, 0.25),
            (0.25, 0.25, 0.0),
            (0.25, 0.0, 0.25),
            (0.0, 0.125, 0.25),
            (0.125, 0.25, 0.0),
        ];
        let mut color = colors.iter();
        let offset = (colors.len() * 100 / 2) as f32;
        for index in 0..colors.len() {
            let sprite = index % 6;
            let i = index as f32;
            let color = color.next().cloned();
            create_sprite(&mut world, &self.hex_sprites, sprite, 128.0 * i - offset, -64.0, color);
        }
        
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

impl MainMenuState {
    pub fn new(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>, hex_sprites: Handle<SpriteSheet>) -> Self {
        MainMenuState {
            spritesheet,
            settings,
            font,
            hex_sprites,
        }
    }
    pub fn new_boxed(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>, hex_sprites: Handle<SpriteSheet>) -> Box<Self> {
        Box::new(MainMenuState::new(spritesheet, settings, font, hex_sprites))
    }
}

fn create_test_text(world: &mut World, font: &Handle<FontAsset>, text: &str) -> Entity {
    let transform = UiTransform::new(
        text.to_string(), Anchor::TopMiddle, Anchor::TopMiddle,
        -50.0, -50.0, 1.0, 800.0, 75.0,
    );

    let text = world
        .create_entity()
        .with(transform)
        .with(UiText::new(
            font.clone(),
            text.to_string(),
            [0.55, 0.59, 0.66, 1.0],
            50.0,
        )).build();

        text
}

fn create_button(world: &mut World, text: &str, font: Handle<FontAsset>) {
    // interactable
    
    // uitransform
    let transform = UiTransform::new("test".to_string(), Anchor::Middle, Anchor::Middle, 0.0, 0.0, 0.0, 200.0, 50.0);
    // uitext
    let text = UiText::new(font, text.to_string(), [1.0, 1.0, 1.0, 1.0], 20.0);
    world
        .create_entity()
        .with(Interactable)
        .with(transform)
        .with(text)
        .build();
}