use bevy::prelude::*;

pub fn handle_click(button: Res<Input<MouseButton>>) {
    if button.just_pressed(MouseButton::Left) {
        println!("clicked, {:?}", button);
    }
}
