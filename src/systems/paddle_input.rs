use amethyst::{
    core::Transform,
    ecs::{System, ReadStorage, WriteStorage},
    input::InputHandler,
}
use crate::components::{Paddle};

pub struct PaddleInputSystem;

impl<'a> System<'a> for PaddleInputSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transform, paddles, input): Self::SystemData) {

    }
}
