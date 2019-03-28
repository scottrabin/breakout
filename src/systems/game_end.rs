use amethyst::{
    core::Transform,
    ecs::{System, ReadStorage},
};
use crate::components::{Ball, Block};

pub struct GameWinSystem;

impl<'a> System<'a> for GameWinSystem {
    type SystemData = (
        ReadStorage<'a, Block>,
    );

    fn run(&mut self, (blocks): Self::SystemData) {

    }
}

pub struct GameLoseSystem;

impl<'a> System<'a> for GameLoseSystem {
    type SystemData = (
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (balls, transforms): Self::SystemData) {

    }
}
