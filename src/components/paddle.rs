use amethyst::ecs::{Component, HashMapStorage};

pub struct Paddle {
    pub width: f32,
}

impl Component for Paddle {
    type Storage = HashMapStorage<Self>;
}

impl Paddle {
    pub fn new(width: usize) -> Paddle {
        Paddle {
            width: width as f32,
        }
    }
}
