use bevy::{prelude::*, window::PrimaryWindow};

use crate::setup::*;

pub fn handle_click(
    button: Res<Input<MouseButton>>,
    mut query: Query<(
        (&mut Sprite, With<Block>),
        (&mut Window, With<PrimaryWindow>),
    )>,
) {
    if button.just_pressed(MouseButton::Left) {
        for i in query.iter_mut() {
            println!("here");
            // sprite.color = Color::WHITE;
            // if let Some(pos) = window.cursor_position() {
            //     println!("{:?}", pos);
            // }
        }
    }
}
