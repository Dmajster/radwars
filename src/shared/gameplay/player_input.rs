use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Debug, Default)]
pub struct PlayerInput {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_back: bool,
 
    pub mouse_horizontal: f32,
    pub mouse_vertical: f32,
}

pub fn player_local_input(
    mut query: Query<&mut PlayerInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut mouse_motion_vector = Vec2::default();

    for event in mouse_motion_events.iter() {
        mouse_motion_vector -= event.delta;
    }

    for mut player_input in query.iter_mut() {
        player_input.move_forward = keyboard_input.pressed(KeyCode::W);
        player_input.move_back = keyboard_input.pressed(KeyCode::S);
        player_input.move_left = keyboard_input.pressed(KeyCode::A);
        player_input.move_right = keyboard_input.pressed(KeyCode::D);

        player_input.mouse_horizontal = mouse_motion_vector.x;
        player_input.mouse_vertical = mouse_motion_vector.y;
    }
}
