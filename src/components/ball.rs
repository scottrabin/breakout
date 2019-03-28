use amethyst::ecs::{Component, HashMapStorage};

pub struct Ball {
    radius: f32,
}

impl Component for Ball {
    type Storage = HashMapStorage<Self>;
}
