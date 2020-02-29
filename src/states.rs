use amethyst::{
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    window::ScreenDimensions,
};

pub struct InitialState;

impl SimpleState for InitialState {
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
                unimplemented!()
            }
        }
        Trans::None
    }
}
