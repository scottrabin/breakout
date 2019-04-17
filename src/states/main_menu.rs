use amethyst::{
    ecs::Entity,
    prelude::*,
    renderer::{Event, WindowEvent},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use super::Gameplay;

#[derive(Debug)]
pub struct MainMenu {
    resource_dir: String,
    ui_root: Option<Entity>,
    play_button: Option<Entity>,
    options_button: Option<Entity>,
    quit_button: Option<Entity>,
}

impl MainMenu {
    pub fn new(resource_dir: String) -> Self {
        MainMenu {
            resource_dir,
            ui_root: None,
            play_button: None,
            options_button: None,
            quit_button: None,
        }
    }
    fn hydrate(&mut self, world: &mut World) {
        if self.ui_root.is_none() {
            self.ui_root = world.exec(|mut creator: UiCreator<'_>| {
                Some(creator.create(format!("{}/ui/main_menu.ron", self.resource_dir), ()))
            })
        }
        if self.play_button.is_none() {
            self.play_button = world.exec(|finder: UiFinder| finder.find("play_button"));
        }
        if self.options_button.is_none() {
            self.options_button = world.exec(|finder: UiFinder| finder.find("options_button"));
        }
        if self.quit_button.is_none() {
            self.quit_button = world.exec(|finder: UiFinder| finder.find("quit_button"));
        }
    }
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.hydrate(data.world)
    }
    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.ui_root
            .take()
            .map(|entity| data.world.delete_entity(entity).expect("delete ui"));
        self.play_button.take();
        self.options_button.take();
        self.quit_button.take();
    }
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            // quit the application when requested
            StateEvent::Window(Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            }) => Trans::Quit,
            // respond to button clicks
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                self.hydrate(data.world);
                match Some(target) {
                    t if t == self.play_button => Trans::Switch(Box::new(Gameplay::Level {
                        resource_dir: self.resource_dir.to_string(),
                        level: 0,
                    })),
                    t if t == self.options_button => {
                        error!("todo: implement options");
                        Trans::None
                    }
                    t if t == self.quit_button => {
                        debug!("quit button pressed");
                        Trans::Quit
                    }
                    _ => Trans::None,
                }
            }
            // unknown event
            _ => Trans::None,
        }
    }
}
