use amethyst::{
    core::nalgebra::Vector2,
    ecs::{Component, VecStorage},
};

pub struct Velocity(pub Vector2<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity(Vector2::new(x, y))
    }
}
