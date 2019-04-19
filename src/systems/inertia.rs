use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    //prelude::*,
};

use crate::components::Velocity;

pub struct InertiaSystem;

impl<'a> System<'a> for InertiaSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (time, velocity, mut transform): Self::SystemData) {
        let elapsed_sec = time.delta_seconds();
        for (velocity, transform) in (&velocity, &mut transform).join() {
            let Velocity(v2) = velocity;
            transform.translate_x(v2.x * elapsed_sec);
            transform.translate_y(v2.y * elapsed_sec);
        }
    }
}
