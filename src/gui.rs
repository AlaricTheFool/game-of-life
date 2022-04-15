use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::control::SimControls;

pub fn add_gui_to_app(mut app: App) -> App {
    app.add_plugin(EguiPlugin)
        .add_system(render_sim_controls);

    app
}

fn render_sim_controls(mut egui_context: ResMut<EguiContext>, mut controls: ResMut<SimControls>) {
    egui::Window::new("Simulator Controls").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Restart").clicked() {
            controls.should_restart = true;
        }

        if ui.button("Clear").clicked() {
            controls.should_clear = true;
        }

        if ui.button("Toggle Pause").clicked() {
            controls.is_paused = !controls.is_paused;
        }
    });
}