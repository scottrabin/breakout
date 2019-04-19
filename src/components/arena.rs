use amethyst::ecs::{Component, HashMapStorage};

#[derive(Debug)]
pub struct Arena {
    pub width: u8,
    pub height: u8,
}

pub struct ArenaBounded;

impl Component for ArenaBounded {
    type Storage = HashMapStorage<Self>;
}
