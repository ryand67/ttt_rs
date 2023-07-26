use bevy::prelude::*;

use crate::{setup::*, *};

pub fn handle_menu_click(
    button: Res<Input<MouseButton>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if button.just_released(MouseButton::Left) {
        app_state.set(AppState::Playing);
    }
}

/// mouse was 0 -> 900 while the board is -450 -> 450
/// converts mouse to board
fn convert_curs_pos(mut curs: Vec2) -> Vec2 {
    curs.x = curs.x - 450.;

    curs.y = (curs.y * -1.) + 450.;

    curs
}

fn block_is_clicked(curs: Vec2, block: Vec3, size: Option<Vec2>) -> bool {
    let size = size.unwrap();
    let x_lower = block.x - size.x / 2.;
    let x_higher = block.x + size.x / 2.;
    let y_lower = block.y - size.y / 2.;
    let y_higher = block.y + size.y / 2.;

    (curs.x > x_lower && curs.x < x_higher) && (curs.y > y_lower && curs.y < y_higher)
}

pub fn handle_playing_click(
    button: Res<Input<MouseButton>>,
    mut blocks: Query<(&Transform, &mut Sprite, With<Block>)>,
    windows: Query<&Window>,
    mut turn_state: ResMut<TurnTracker>,
) {
    if button.just_released(MouseButton::Left) {
        let mut curs_pos = windows.single().cursor_position().unwrap();
        curs_pos = convert_curs_pos(curs_pos);

        for mut sp in blocks.iter_mut() {
            if block_is_clicked(curs_pos, sp.0.translation, sp.1.custom_size) {
                // sp.1.color = Color::RED;

                println!("{:?}", turn_state.0);
                match turn_state.0 {
                    TurnState::P1 => {
                        sp.1.color = Color::RED;
                        turn_state.0 = TurnState::P2;
                        return;
                    }
                    TurnState::P2 => {
                        sp.1.color = Color::BLUE;
                        turn_state.0 = TurnState::P1;
                        return;
                    }
                }
            }
        }
    }
}
