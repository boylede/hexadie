use amethyst::{
    assets::Handle,
    core::ecs::Entity,
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

use std::collections::HashMap;

pub struct SettingsState {
    menu_items: HashMap<Entity, MenuFunction>,
    my_ui: Vec<Entity>,
}

impl SimpleState for SettingsState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let assets = (*world.read_resource::<HexAssets>()).clone();

        let title = create_title_text(world, &assets.font, "a settings screen");
        self.my_ui.push(title);
        let menu = MenuBuilder::new(30.0, &assets.font)
            .add_button(world, "no settings", Box::new(back))
            .add_button(world, "exist, yet!", Box::new(backa))
            .add_button(world, "go back", Box::new(backb));
        let mut uis = menu.get_entities();
        println!("inserting menu entries for {:?}", uis);
        self.my_ui.append(&mut uis);
        self.menu_items = menu.get_bindings();
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_entities(&self.my_ui);
        // self.my_ui.iter().for_each(|e| {
        //     data.world.entities_mut().delete(*e).expect("tried to delete item twice.");
        // });
        self.my_ui.clear();
        data.world.maintain();
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
                    println!("Clicked! {:?}", target);
                    if let Some(transition) = self.menu_items.get_mut(target) {
                        return transition(&mut data.world);
                    } else {
                        println!("clicked on something with no function? items available: {:?}\n\n", self.menu_items.keys());
                    }
                }
                HoverStart => {
                    println!("hovered on {:?}", target);
                }
                HoverStop => {
                    println!("hovered off {:?}", target);
                }
                _ => {
                    //
                }
            }
        }
        Trans::None
    }
}

impl SettingsState {
    pub fn new() -> Self {
        SettingsState {
            menu_items: Default::default(),
            my_ui: vec![],
        }
    }
    pub fn new_boxed() -> Box<Self> {
        Box::new(SettingsState::new())
    }
}

type MenuFunction = Box<fn(&mut World) -> SimpleTrans>;

fn back(_w: &mut World) -> SimpleTrans {
    println!("leaving settings screen");
    Trans::Pop
}

fn backa(_w: &mut World) -> SimpleTrans {
    println!("leaving settings screen");
    Trans::Pop
}

fn backb(_w: &mut World) -> SimpleTrans {
    println!("leaving settings screen");
    Trans::Pop
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
