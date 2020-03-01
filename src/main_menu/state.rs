use amethyst::{
    assets::{
        Handle,
        Loader,
    },
    core::ecs::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{SpriteSheet},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, FontAsset},
};

use crate::config::GameSettings;
use crate::entities::create_sprite;

pub struct MainMenuState {
    spritesheet: Handle<SpriteSheet>,
    settings: Handle<GameSettings>,
    font: Handle<FontAsset>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        create_test_text(&mut world, &self.font, "TEST TEST TEST TEST");
        create_sprite(&mut world, &self.spritesheet, 5, 21.0, 300.0);
        create_sprite(&mut world, &self.spritesheet, 5, 640.0, 21.0);
        create_sprite(&mut world, &self.spritesheet, 5, -320.0, 90.0);
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
        Trans::None
    }
}

impl MainMenuState {
    pub fn new(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>) -> Self {
        MainMenuState {
            spritesheet,
            settings,
            font,
        }
    }
    pub fn new_boxed(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>) -> Box<Self> {
        Box::new(MainMenuState::new(spritesheet, settings, font))
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
