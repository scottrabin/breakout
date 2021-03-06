#[macro_use]
extern crate log;

pub mod components;
pub mod states;
pub mod systems;

pub mod common {
    use crate::components::Arena;
    use amethyst::{
        assets::Loader,
        core::Transform,
        ecs::Entity,
        prelude::*,
        renderer::{
            Camera, PngFormat, Projection, SpriteSheetFormat, SpriteSheetHandle, TextureMetadata,
        },
    };

    pub fn ortho_camera(world: &mut World, arena: &Arena) -> Entity {
        let mut trans = Transform::default();
        trans.set_translation_z(1.);
        info!("camera location: {:?}", trans.translation().as_slice());
        let lrbt = (
            0.,                  // left
            arena.width as f32,  // right
            0.,                  // bottom
            arena.height as f32, // top
        );
        info!("camera left,right,bottom,top: {:?}", lrbt);
        world
            .create_entity()
            .with(trans)
            .with(Camera::from(Projection::orthographic(
                lrbt.0, lrbt.1, lrbt.2, lrbt.3,
            )))
            .build()
    }

    pub fn load_sprites(
        image_path: &str,
        sheet_path: &str,
        world: &mut World,
    ) -> SpriteSheetHandle {
        let loader = world.read_resource::<Loader>();
        let texture = loader.load(
            image_path,
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource(),
        );
        loader.load(
            sheet_path,
            SpriteSheetFormat,
            texture,
            (),
            &world.read_resource(),
        )
    }
}
