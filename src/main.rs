//! Pong Tutorial 1
use amethyst::core::transform::TransformBundle;
use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
mod pong; 
use crate::pong::Pong;

extern crate amethyst;
use amethyst::prelude::*;
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    // We'll put the rest of the code here.

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
        // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
            .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
        .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(TransformBundle::new())?;
    
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
