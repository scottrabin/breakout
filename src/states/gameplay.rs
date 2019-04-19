use amethyst::{ecs::Entity, prelude::*, renderer::SpriteRender};

// FIXME: this `super::` nonsense seems like an antipattern; what's the idiomatic way?
use crate::common;
use crate::components::{Arena, Ball};

#[derive(Debug)]
pub enum Gameplay {
    Level {
        resource_dir: String,
        level: u8,
    },
    Initialized {
        resource_dir: String,
        entities: Vec<Entity>,
    },
}

impl SimpleState for Gameplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        match self {
            Gameplay::Level {
                resource_dir,
                level,
            } => {
                warn!("todo: implement level selection for level {}", level);

                let spritesheet = common::load_sprites(
                    format!("{}/sprites/sheet0.png", resource_dir),
                    format!("{}/sprites/sheet0.ron", resource_dir),
                    data.world,
                );
                let ball_sprite = SpriteRender {
                    sprite_sheet: spritesheet.clone(),
                    sprite_number: 0,
                };

                let arena = Arena {
                    width: 80,
                    height: 60,
                };
                info!("arena: {:?}", arena);

                *self = Gameplay::Initialized {
                    resource_dir: resource_dir.to_string(),
                    entities: vec![
                        common::ortho_camera(data.world, &arena),
                        Ball::new(data.world, ball_sprite, &arena),
                    ],
                };

                data.world.add_resource(arena);
            }
            _ => warn!("Gameplay state started with: {:?}", self),
        }
    }
    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        error!("todo: destroy the components used in gameplay");
    }
    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        error!("todo: tell the ball to stop moving and stop any countdowns");
    }
    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        error!("todo: resume the ball's motion and level countdowns");
    }
    // consider implemeting fixed_update/update or shadow_fixed_update/shadow_update for animations
    // that don't impact the gameplay .. don't put those things into systems!
}
