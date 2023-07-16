use bevy::prelude::*;
use bevy::window::WindowTheme;

use setup::*;
mod setup;

const RESOLUTION: (f32, f32) = (900., 900.);
const THEME: Option<WindowTheme> = Some(WindowTheme::Dark);
const MARGIN_SIZE: f32 = 30.;
const BOX_PADDING: f32 = 30.;

enum AppSate {
    MainMenu,
    Playing,
    Settings,
}

fn main() {
    let mut game_settings = GameSettings::new(RESOLUTION, THEME, MARGIN_SIZE, BOX_PADDING);

    App::new()
        .add_plugins(DefaultPlugins.set(game_settings.create_window()))
        .add_systems(Startup, game_settings.setup_world())
        .run();
}
