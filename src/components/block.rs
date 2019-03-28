use amethyst::ecs::{Component, VecStorage};

pub struct Block {
    health: u16,
}

impl Component for Block {
    type Storage = VecStorage<Self>;
}
