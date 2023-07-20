use bevy::prelude::*;
use bevy::window::WindowTheme;

use setup::*;
use util::*;

mod setup;
mod util;

const RESOLUTION: (f32, f32) = (900., 900.);
const THEME: Option<WindowTheme> = Some(WindowTheme::Dark);
const MARGIN_SIZE: f32 = 30.;
const BOX_PADDING: f32 = 30.;

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    MainMenu,
    Playing,
    EndGame,
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TurnState {
    #[default]
    P1,
    P2,
}

fn main() {
    let mut game_settings = GameSettings::new(RESOLUTION, THEME, MARGIN_SIZE, BOX_PADDING);

    App::new()
        .add_plugins(DefaultPlugins.set(game_settings.create_window()))
        .add_state::<AppState>()
        .add_state::<TurnState>()
        .add_systems(OnEnter(AppState::MainMenu), game_settings.setup_menu())
        .add_systems(OnEnter(AppState::Playing), game_settings.setup_board())
        .add_systems(
            Update,
            handle_playing_click.run_if(in_state(AppState::Playing)),
        )
        .run();
}
