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

#[derive(Debug)]
pub enum TurnState {
    P1,
    P2,
}

#[derive(Debug, Resource)]
pub struct TurnTracker(TurnState);

fn main() {
    let mut game_settings = GameSettings::new(RESOLUTION, THEME, MARGIN_SIZE, BOX_PADDING);

    App::new()
        .insert_resource(TurnTracker(TurnState::P1))
        .add_plugins(DefaultPlugins.set(game_settings.create_window()))
        .add_state::<AppState>()
        .add_systems(Startup, game_settings.startup())
        .add_systems(OnEnter(AppState::MainMenu), game_settings.setup_menu())
        .add_systems(OnEnter(AppState::Playing), game_settings.setup_board())
        .add_systems(
            Update,
            handle_menu_click.run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            handle_playing_click.run_if(in_state(AppState::Playing)),
        )
        .run();
}
