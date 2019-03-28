use amethyst::{
    ecs::{System, Read, ReadStorage, WriteStorage},
    core::timing::Time,
    core::Transform,
    prelude::Entities,
};
use crate::components::{Ball, Block, Paddle, Velocity};

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    // TODO too many entries in this tuple
    type SystemData = (
        entities: Entities<'a>,
        balls: ReadStorage<'a, Ball>,
        blocks: ReadStorage<'a, Block>,
        paddles: ReadStorage<'a, Paddle>,
        transform: ReadStorage<'a, Transform>,
        velocity: WriteStorage<'a, Velocity>,
        time: Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {

    }
}
