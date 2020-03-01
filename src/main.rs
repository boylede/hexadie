use amethyst::{
    assets::Processor,
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    window::DisplayConfig,
    utils::application_root_dir,
    LoggerConfig, StdoutLog,
};

mod config;
mod loading_screen;
mod main_menu;

use config::GameSettings;

fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;

    let mut logger: LoggerConfig = Default::default();
    logger.log_file = Some(app_root.join("log.txt"));
    logger.stdout = StdoutLog::Off;
    amethyst::start_logger(logger);

    let assets_path = app_root.join("assets");

    let icon_path = assets_path.join("boardgamepack/dice/dieRed6.png");

    let display_config = DisplayConfig {
        title: "Hexadie".to_string(),
        fullscreen: None,
        dimensions: Some((800, 800)),
        min_dimensions: Some((300, 300)),
        max_dimensions: None,
        visibility: true,
        icon: Some(icon_path),
        always_on_top: false,
        decorations: true,
        maximized: false,
        multitouch: false,
        resizable: true,
        transparent: false,
        loaded_icon: None,
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
        )?
        .with(
            Processor::<GameSettings>::new(),
            "settings_processor",
            &[],
        );

    let mut game = Application::new(assets_path, loading_screen::InitialState::new(), game_data)?;
    game.run();

    Ok(())
}