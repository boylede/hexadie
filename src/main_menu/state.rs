use amethyst::{
    assets::{
        Handle,
    },
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{SpriteSheet},
    window::ScreenDimensions,
};

use crate::config::GameSettings;

pub struct MainMenuState {
    spritesheet: Handle<SpriteSheet>,
    settings: Handle<GameSettings>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

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
    pub fn new(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>) -> Self {
        MainMenuState {
            spritesheet,
            settings,
        }
    }
    pub fn new_boxed(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>) -> Box<Self> {
        Box::new(MainMenuState::new(spritesheet, settings))
    }
}
