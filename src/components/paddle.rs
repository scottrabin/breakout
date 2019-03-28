use amethyst::ecs::{Component, HashMapStorage};

pub struct Paddle {
    width: usize,
}

impl Component for Paddle {
    type Storage = HashMapStorage<Self>;
}
