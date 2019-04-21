use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Component, Entity, HashMapStorage},
    prelude::*,
    renderer::SpriteRender,
};

use crate::components::{Arena, ArenaBounded, Velocity};

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = HashMapStorage<Self>;
}

impl Ball {
    pub fn new(world: &mut World, sprite_render: SpriteRender, arena: &Arena) -> Entity {
        let trans = Transform::from(Vector3::new(
            arena.width as f32 / 2.,
            arena.height as f32 / 2.,
            0.,
        ));
        info!("ball location: {:?}", trans.translation().as_slice());
        world
            .create_entity()
            .with(trans)
            .with(sprite_render)
            .with(Ball { radius: 1. })
            .with(ArenaBounded)
            .with(Velocity::new(4., 8.)) // XXX hack TODO remove me
            .build()
    }
}
