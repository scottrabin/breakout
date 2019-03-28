use amethyst::ecs::{Component, VecStorage};

pub struct Velocity(f32, f32);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
