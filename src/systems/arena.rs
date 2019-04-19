use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    //prelude::*,
};

use crate::components::{Arena, ArenaBounded, Velocity};

pub struct ArenaCollisionSystem;

impl<'a> System<'a> for ArenaCollisionSystem {
    type SystemData = (
        Read<'a, Arena>,
        ReadStorage<'a, ArenaBounded>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (arena, arena_bounded, transform, mut velocity): Self::SystemData) {
        for (_, transform, velocity) in (&arena_bounded, &transform, &mut velocity).join() {
            // FIXME: i'm just using the Transform directly as the position.. should we have a
            // Position component which translates to a Transform? this would allow us to
            // reposition the whole arena by updating Transforms according to Positions
            // FIXME: this assumes the arena is expressed in "transform units"

            // FIXME: find a concise mathematical way of testing "in a box?" and "get in the box!"

            // XXX: this doesn't actually move that are already outside of the arena back in; this
            // just adjusts the velocity so the object heads back toward the arena

            let Velocity(vel) = velocity;
            let translation = transform.translation();

            if translation.x < 0. && vel.x < 0. {
                vel.x *= -1.;
                debug!("LEFT WALL");
            } else if translation.x > arena.width as f32 && vel.x > 0. {
                vel.x *= -1.;
                debug!("RIGHT WALL");
            }

            if translation.y < 0. && vel.y < 0. {
                vel.y *= -1.;
                debug!("BOTTOM WALL");
            } else if translation.y > arena.height as f32 && vel.y > 0. {
                vel.y *= -1.;
                debug!("TOP WALL");
            }
        }
    }
}
