use amethyst::{
    core::Transform,
    ecs::{ReadStorage, System, WriteStorage},
};
use crate::components::{Position, Velocity};

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
    }
}
