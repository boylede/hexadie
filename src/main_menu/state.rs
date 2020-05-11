use amethyst::{
    assets::Handle,
    core::{
        ecs::{Entity, ReadStorage, WriteStorage},
        Hidden,
    },
    ecs::{Entities, Join},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::SpriteSheet,
    ui::{Anchor, FontAsset, Interactable, UiEvent, UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::assets::HexAssets;
use crate::config::GameSettings;
use crate::entities::create_sprite;
use crate::map_selection::MapSelectionState;
use crate::settings_screen::SettingsState;

use std::collections::HashMap;

pub struct MainMenuState {
    menu_items: HashMap<Entity, MenuFunction>,
    my_ui: Vec<Entity>,
}

const EXPECTED: &'static str = "Expected UI entity or component not found (main_menu).";
impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let assets = (*world.read_resource::<HexAssets>()).clone();

        let title = create_title_text(world, &assets.font, "HEXADIE");
        self.my_ui.push(title);
        let menu = MenuBuilder::new(50.0, &assets.font)
            .add_button(world, "new game", Box::new(new_game))
            .add_button(world, "settings", Box::new(settings))
            .add_button(world, "quit", Box::new(quit));
        self.my_ui.append(&mut menu.get_entities());
        self.menu_items = menu.get_bindings();
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.my_ui.iter().for_each(|e| {
            data.world.entities_mut().delete(*e).expect(EXPECTED);
        });
    }
    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(
            |(mut transform, mut hidden): (WriteStorage<UiTransform>, WriteStorage<Hidden>)| {
                self.my_ui.iter().for_each(|e| {
                    hidden.insert(*e, Hidden).expect(EXPECTED);
                    transform.get_mut(*e).expect(EXPECTED).opaque = false;
                });
            },
        );
    }
    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(
            |(mut transform, mut hidden): (WriteStorage<UiTransform>, WriteStorage<Hidden>)| {
                self.my_ui.iter().for_each(|e| {
                    hidden.remove(*e);
                    let t = transform.get_mut(*e).expect(EXPECTED);
                    t.opaque = true;
                });
            },
        );
    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
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
            use amethyst::ui::UiEventType::*;
            let UiEvent { event_type, target } = event;
            match event_type {
                Click => {
                    if let Some(transition) = self.menu_items.get_mut(target) {
                        return transition(&mut data.world);
                    }
                }
                HoverStart => {
                }
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
    pub fn new() -> Self {
        MainMenuState {
            menu_items: Default::default(),
            my_ui: vec![],
        }
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(MainMenuState::new())
    }
}

type MenuFunction = Box<fn(world: &mut World) -> SimpleTrans>;

fn new_game(_w: &mut World) -> SimpleTrans {
    let map = MapSelectionState::new_boxed();
    Trans::Switch(map)
}

fn settings(_w: &mut World) -> SimpleTrans {
    Trans::Push(SettingsState::new_boxed())
}

fn quit(_w: &mut World) -> SimpleTrans {
    Trans::Quit
}

struct MenuBuilder {
    y: f32,
    item_height: f32,
    font: Handle<FontAsset>,
    bindings: Vec<(Entity, MenuFunction)>,
}

impl MenuBuilder {
    pub fn new(height: f32, font: &Handle<FontAsset>) -> MenuBuilder {
        MenuBuilder {
            y: 0.0,
            item_height: height,
            font: font.clone(),
            bindings: vec![],
        }
    }
    pub fn add_button(
        mut self,
        world: &mut World,
        text: &str,
        function: MenuFunction,
    ) -> MenuBuilder {
        // needed components
        // interactable
        let interactable = Interactable;
        // uitransform
        let transform = UiTransform::new(
            text.to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.0,
            self.y,
            0.0,
            200.0,
            50.0,
        );
        // uitext
        let text = UiText::new(
            self.font.clone(),
            text.to_string(),
            [0.1, 0.1, 0.1, 1.0],
            20.0,
        );
        let entity = world
            .create_entity()
            .with(interactable)
            .with(transform)
            .with(text)
            .build();
        self.y = self.y - self.item_height;
        self.bindings.push((entity, function));
        self
    }
    pub fn get_entities(&self) -> Vec<Entity> {
        self.bindings.iter().map(|(e, _)| *e).collect()
    }
    pub fn get_bindings(self) -> HashMap<Entity, MenuFunction> {
        self.bindings.into_iter().collect()
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
