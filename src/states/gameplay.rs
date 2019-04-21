use amethyst::{core::Transform, ecs::Entity, prelude::*, renderer::SpriteRender};

// FIXME: this `super::` nonsense seems like an antipattern; what's the idiomatic way?
use crate::common;
use crate::components::{Arena, ArenaBounded, Ball, Paddle, Velocity};

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

                let arena = Arena {
                    width: 80.0,
                    height: 60.0,
                };
                info!("arena: {:?}", arena);
                initialize_ball(data.world, &arena);
                initialize_paddle(data.world, &arena);

                *self = Gameplay::Initialized {
                    resource_dir: resource_dir.to_string(),
                    entities: vec![common::ortho_camera(data.world, &arena)],
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

fn initialize_ball(world: &mut World, arena: &Arena) {
    let sprite_sheet =
        common::load_sprites("sprites/ball/ball.png", "sprites/ball/ball.ron", world);
    let ball_radius = 1.0;

    world
        .create_entity()
        .with(Ball::new(ball_radius))
        .with({
            let mut transform = Transform::default();
            transform.set_translation_xyz(arena.width / 2.0, arena.height / 2.0, 0.0);
            transform.set_scale(ball_radius / 16.0, ball_radius / 16.0, 1.0);
            info!("ball location: {:?}", transform.translation().as_slice());
            transform
        })
        .with(Velocity::new(4., 8.))
        .with(SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 0,
        })
        .with(ArenaBounded)
        .build();
}

fn initialize_paddle(world: &mut World, arena: &Arena) {
    let sprite_sheet = common::load_sprites(
        "sprites/paddle/paddle.png",
        "sprites/paddle/paddle.ron",
        world,
    );
    let paddle_width = arena.width / 10.0;

    world
        .create_entity()
        .with(Paddle::new(paddle_width))
        .with({
            let mut transform = Transform::default();
            transform.set_translation_xyz(arena.width / 2.0, arena.height / 10.0, 0.0);
            transform.set_scale(paddle_width / 16.0, 0.2, 1.0);
            transform
        })
        .with(SpriteRender {
            sprite_sheet: sprite_sheet,
            sprite_number: 0,
        })
        .build();
}
