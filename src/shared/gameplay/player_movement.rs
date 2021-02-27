use bevy::{core::Time, math::Vec3, prelude::*};
use heron::{AxisAngle, Velocity};

use crate::shared::gameplay::PlayerInput;

pub fn move_player(
    time: Res<Time>,
    mut transform_query: Query<&mut Transform>,
    mut player_query: Query<(Entity, &Children, &mut Velocity, &PlayerInput)>,
) {
    for (entity, children, mut velocity, input) in player_query.iter_mut() {
        if let Ok(transform) = transform_query.get_mut(entity) {
            // POSITION
            let mut input_vector = Vec3::default();
            let movement_speed = 5.0;
            let mouse_speed = 1.0;

            if input.move_right {
                input_vector.x += 1.0;
            }
            if input.move_left {
                input_vector.x -= 1.0;
            }
            if input.move_forward {
                input_vector.y -= 1.0;
            }
            if input.move_back {
                input_vector.y += 1.0;
            }

            if input_vector.length() > 0.0 {
                input_vector.normalize();
            }

            input_vector *= movement_speed;

            let forward_direction = transform.rotation * Vec3::unit_z();
            let strafing_direction = transform.rotation * Vec3::unit_x();
            let movement_vector =
                forward_direction * input_vector.y + strafing_direction * input_vector.x;

            velocity.linear.x = movement_vector.x;
            velocity.linear.z = movement_vector.z;
            velocity.angular = AxisAngle::new(
                Vec3::new(0.0, 1.0, 0.0),
                input.mouse_horizontal * mouse_speed,
            );
        }

        if let Ok(mut transform) = transform_query.get_mut(children[0]) {
            transform.rotate(Quat::from_rotation_x(
                input.mouse_vertical * time.delta_seconds(),
            ));
        }
    }
}
