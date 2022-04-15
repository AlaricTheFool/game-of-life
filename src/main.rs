use bevy::{prelude::*};

mod tiles;
mod gui;
mod control;

const BACKGROUND_COLOR  : Color = Color::rgb(026. / 255., 009. / 255., 013. / 255.);
//const FILL_COLOR        : Color = Color::rgb(187. / 255., 068. / 255., 048. / 255.);
//const EMPTY_COLOR       : Color = Color::rgb(074. / 255., 049. / 255., 077. / 255.);

fn main() {
    let mut app = App::new();

    app = add_base_to_bevy_app(app);
    app = tiles::add_conway_tiles_to_app(app);
    app = gui::add_gui_to_app(app);
    app = control::add_control_to_app(app);

    app.add_startup_system(spawn_camera);
    
    app.run();
}

fn add_base_to_bevy_app(mut app: App) -> App {
    app.insert_resource(WindowDescriptor {
        width: 1280.0,
        height: 720.0,
        title: String::from("Cellular Automata"),
        ..Default::default()
    })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins);
    
    app
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}