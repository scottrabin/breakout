use amethyst::ecs::{Component, HashMapStorage};

#[derive(Debug)]
pub struct Arena {
    pub width: u8,
    pub height: u8,
}

impl Default for Arena {
    fn default() -> Self {
        Arena {
            width: 0,
            height: 0,
        }
    }
}

pub struct ArenaBounded;

impl Component for ArenaBounded {
    type Storage = HashMapStorage<Self>;
}
