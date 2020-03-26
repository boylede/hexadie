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
use crate::map_selection::MapSelectionState;

use std::collections::HashMap;

pub struct MainMenuState {
    spritesheet: Handle<SpriteSheet>,
    settings: Handle<GameSettings>,
    font: Handle<FontAsset>,
    hex_sprites: Handle<SpriteSheet>,
    menu_items: HashMap<Entity, MenuFunction>,
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;
        let _dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        let menu = create_menu(&mut world, &self.font);
        self.menu_items = menu;
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
            let rotation = 60.0 * i;
            create_sprite(&mut world, &self.hex_sprites, sprite, 128.0 * i - offset, -64.0, color, rotation, 0.5);
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
            use amethyst::ui::UiEventType::*;
            let UiEvent{event_type, target} = event;
            match event_type {
                Click => {
                    println!("Clicked! {:?}", target);
                    if let Some(transition) = self.menu_items.get_mut(target) {
                        //todo: stop carrying around these assets because it is getting difficult to pass them to new states
                        let dummy_menu = MainMenuState::new(self.spritesheet.clone(), self.settings.clone(), self.font.clone(), self.hex_sprites.clone());
                        return transition(&dummy_menu);
                    }
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
            menu_items: Default::default(),
        }
    }
    pub fn new_boxed(spritesheet: Handle<SpriteSheet>, settings: Handle<GameSettings>, font: Handle<FontAsset>, hex_sprites: Handle<SpriteSheet>) -> Box<Self> {
        Box::new(MainMenuState::new(spritesheet, settings, font, hex_sprites))
    }
}

type MenuFunction = Box<(Fn(&MainMenuState) -> SimpleTrans)>;

fn new_game(main_menu: &MainMenuState) -> SimpleTrans {
    // todo remove main menu's items from the world
    let map = MapSelectionState::new_boxed(
        main_menu.spritesheet.clone(),
        main_menu.settings.clone(),
        main_menu.font.clone(),
        main_menu.hex_sprites.clone(),
    );
    Trans::Switch(map)
}

fn settings(main_menu: &MainMenuState) -> SimpleTrans {
    Trans::Quit
}

fn quit(main_menu: &MainMenuState) -> SimpleTrans {
    Trans::Quit
}

fn create_menu(world: &mut World, font: &Handle<FontAsset>) -> HashMap<Entity, MenuFunction> {
    create_title_text(world, font, "HEXADIE");
    let menu_items = MenuBuilder::new(50.0, font)
        .add_button(world, "new game", Box::new(new_game))
        .add_button(world, "settings", Box::new(settings))
        .add_button(world, "quit", Box::new(quit))
        .get_bindings();
    menu_items
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
    pub fn add_button(mut self, world: &mut World, text: &str, function: MenuFunction) -> MenuBuilder {
        // needed components
        // interactable
        let interactable = Interactable;
        // uitransform
        let transform = UiTransform::new(text.to_string(), Anchor::Middle, Anchor::Middle, 0.0, self.y, 0.0, 200.0, 50.0);
        // uitext
        let text = UiText::new(self.font.clone(), text.to_string(), [1.0, 1.0, 1.0, 1.0], 20.0);
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
    pub fn get_bindings(self) -> HashMap<Entity, MenuFunction> {
        self.bindings.into_iter().collect()
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

fn create_button(world: &mut World, text: &str, font: &Handle<FontAsset>) {
    // interactable
    
    // uitransform
    let transform = UiTransform::new(text.to_string(), Anchor::Middle, Anchor::Middle, 0.0, 0.0, 0.0, 200.0, 50.0);
    // uitext
    let text = UiText::new(font.clone(), text.to_string(), [1.0, 1.0, 1.0, 1.0], 20.0);
    world
        .create_entity()
        .with(Interactable)
        .with(transform)
        .with(text)
        .build();
}