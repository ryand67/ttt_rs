use bevy::prelude::*;

use crate::{setup::*, AppState, TurnState};
pub fn handle_playing_click(
    button: Res<Input<MouseButton>>,
    mut blocks: Query<(&Transform, &mut Sprite, With<Block>)>,
    windows: Query<&Window>,
    turn_state: ResMut<NextState<TurnState>>,
) {
    if button.just_pressed(MouseButton::Left) {
        let curs_pos = windows.single().cursor_position().unwrap();
        for sp in blocks.iter_mut() {}
    }
}
