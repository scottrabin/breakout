#[macro_use]
extern crate log;

extern crate amethyst;
use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

use breakout::states::MainMenu;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resource_dir = format!("{}/resources", application_root_dir());
    let config = DisplayConfig::load(format!("{}/display_config.ron", resource_dir));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new())
            .with_pass(DrawUi::new()),
    );

    let start_state = MainMenu::new(resource_dir);
    warn!("starting amethyst in state: {:?}", start_state);
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", start_state, game_data)?;

    game.run();

    Ok(())
}
