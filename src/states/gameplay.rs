use amethyst::prelude::*;

pub struct Gameplay;

impl SimpleState for Gameplay {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        error!("todo: create the necessary entities and components for gameplay");
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
