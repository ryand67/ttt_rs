use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution, WindowTheme},
};

#[derive(Default)]
pub struct GameSettings {
    resolution: (f32, f32),
    window_theme: Option<WindowTheme>,
}

impl GameSettings {
    pub fn new(resolution: (f32, f32), window_theme: Option<WindowTheme>) -> Self {
        Self {
            resolution,
            window_theme,
        }
    }

    pub fn get_resolution(&self) -> WindowResolution {
        self.resolution.into()
    }

    pub fn get_theme(&self) -> Option<WindowTheme> {
        match self.window_theme {
            Some(_) => {
                return self.window_theme;
            }
            None => return Some(WindowTheme::Dark),
        }
    }

    pub fn create_window(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                resolution: self.get_resolution(),
                window_theme: self.get_theme(),
                ..default()
            }),
            ..default()
        }
    }

    pub fn setup_world(&self) -> impl Fn(Commands) {
        move |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        }
    }
}
