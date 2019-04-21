use amethyst::ecs::{Component, HashMapStorage};

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = HashMapStorage<Self>;
}

impl Ball {
    pub fn new(radius: f32) -> Ball {
        Ball { radius }
    }
}
