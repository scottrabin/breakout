use crate::components::{Ball, Block, Paddle, Velocity};
use amethyst::ecs::{ReadStorage, System};

// This system just `uses` all of our components so that we don't need to manually register any of
// them.
pub struct DummySystem;

impl<'a> System<'a> for DummySystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, _: Self::SystemData) {}
}
