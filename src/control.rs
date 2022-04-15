use bevy::prelude::*;

pub struct SimControls {
    pub should_restart: bool,
    pub should_clear: bool,
    pub is_paused: bool,
}

impl SimControls {
    fn new() -> Self {
        SimControls {
            should_restart: false,
            should_clear: false,
            is_paused: false,
        }
    }
}

pub fn add_control_to_app(mut app: App) -> App {
    app.insert_resource(SimControls::new());

    app
}