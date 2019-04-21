use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::components::Paddle;

pub struct PaddleInputSystem;

impl<'a> System<'a> for PaddleInputSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Paddle>,
        Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            if let Some(mv) = input.axis_value("paddle") {
                if mv != 0.0 {
                    transform.set_translation_x(
                        (transform.translation().x + mv as f32)
                            .min(80.0 - paddle.width / 2.0)
                            .max(paddle.width / 2.0),
                    );
                }
            }
        }
    }
}
