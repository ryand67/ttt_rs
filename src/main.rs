use bevy::prelude::*;
use bevy::window::WindowTheme;

use setup::*;
mod setup;

const RESOLUTION: (f32, f32) = (900., 900.);
const THEME: Option<WindowTheme> = Some(WindowTheme::Dark);

enum AppSate {
    MainMenu,
    Playing,
    Settings,
}

fn main() {
    let game_settings = GameSettings::new(RESOLUTION, THEME);

    App::new()
        .add_plugins(DefaultPlugins.set(game_settings.create_window()))
        .add_systems(Startup, game_settings.setup_world())
        .run();
}
