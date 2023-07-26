use bevy::{
    prelude::*,
    window::{WindowPlugin, WindowResolution, WindowTheme},
};

#[derive(Component, Debug)]
pub struct Block {
    x: f32,
    y: f32,
    state: Option<BlockState>,
}

#[derive(Debug)]
enum BlockState {
    X,
    O,
}

#[derive(Default)]
pub struct GameSettings {
    resolution: (f32, f32),
    window_theme: Option<WindowTheme>,
    margin_size: f32,
    box_padding: f32,
}

impl GameSettings {
    pub fn new(
        resolution: (f32, f32),
        window_theme: Option<WindowTheme>,
        margin_size: f32,
        box_padding: f32,
    ) -> Self {
        Self {
            resolution,
            window_theme,
            margin_size,
            box_padding,
        }
    }

    pub fn get_resolution(&self) -> (f32, f32) {
        self.resolution
    }

    pub fn get_window_resolution(&self) -> WindowResolution {
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
                resolution: self.get_window_resolution(),
                window_theme: self.get_theme(),
                resizable: false,
                ..default()
            }),
            ..default()
        }
    }

    fn get_square_size(&self) -> (f32, f32) {
        let x = (self.resolution.0 - (self.margin_size * 2.) - (self.box_padding / 2.)) / 3.;
        let y = (self.resolution.1 - (self.margin_size * 2.) - (self.box_padding / 2.)) / 3.;

        (x, y)
    }

    pub fn startup(&mut self) -> impl Fn(Commands) {
        return move |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        };
    }

    pub fn setup_menu(&mut self) -> impl Fn(Commands) {
        return move |mut commands: Commands| {
            println!("menu");
        };
    }

    pub fn setup_board(&mut self) -> impl Fn(Commands) {
        let square_size = self.get_square_size();
        let res = self.get_resolution();

        return move |mut commands: Commands| {
            let mut spawn_block = |x: f32, y: f32| {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(square_size.0, square_size.1)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(x, y, 0.)),
                        ..default()
                    },
                    Block { x, y, state: None },
                ));
            };

            // cols
            for i in 1..=3 {
                //rows
                for j in 1..=3 {
                    // 300, -150
                    let x = ((res.0 / 3.) * i as f32) - (res.0 / 1.5);
                    let y = ((res.1 / 3.) * j as f32) - (res.1 / 1.5);

                    spawn_block(x, y);
                    spawn_block(x, y);
                }
            }
        };
    }
}
