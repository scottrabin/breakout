use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Join, ReadStorage, System, WriteStorage},
};

use crate::components::{Ball, Paddle, Velocity};

pub struct BallPaddleCollisionSystem;

impl<'a> System<'a> for BallPaddleCollisionSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
    );

    fn run(&mut self, (mut velocity, transform, ball, paddle): Self::SystemData) {
        for (ball, ball_transform, Velocity(ball_velocity)) in
            (&ball, &transform, &mut velocity).join()
        {
            for (paddle, paddle_transform) in (&paddle, &transform).join() {
                if will_intersect(
                    ball,
                    ball_transform.translation(),
                    paddle,
                    paddle_transform.translation(),
                ) && ball_velocity.y < 0.0
                {
                    ball_velocity.y *= -1.0;
                }
            }
        }
    }
}

fn will_intersect(
    ball: &Ball,
    ball_translation: &Vector3<f32>,
    paddle: &Paddle,
    paddle_translation: &Vector3<f32>,
) -> bool {
    is_between(
        ball_translation.x,
        paddle_translation.x - paddle.width / 2.0 - ball.radius,
        paddle_translation.x + paddle.width / 2.0 + ball.radius,
    ) && is_between(
        ball_translation.y,
        paddle_translation.y - ball.radius,
        paddle_translation.y + ball.radius,
    )
}

fn is_between(n: f32, low: f32, high: f32) -> bool {
    low < n && n < high
}
