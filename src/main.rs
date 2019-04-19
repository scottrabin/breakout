#[macro_use]
extern crate log;

extern crate amethyst;
use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};

use breakout::{states, systems};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resource_dir = format!("{}/resources", application_root_dir());
    let config = DisplayConfig::load(format!("{}/display_config.ron", resource_dir));

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let start_state = states::MainMenu::new(resource_dir.to_string());
    let _start_state = states::Gameplay::Level {
        resource_dir,
        level: 0,
    };
    warn!("starting amethyst in state: {:?}", start_state);
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with(systems::InertiaSystem, "inertia_system", &[])
        .with(
            systems::ArenaCollisionSystem,
            "arena_collision_system",
            &["inertia_system"],
        )
        .with(systems::DummySystem, "dummy_system", &[]);
    let mut game = Application::new("./", start_state, game_data)?;

    game.run();

    Ok(())
}
