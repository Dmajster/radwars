use bevy::{core::Time, math::Vec3, prelude::*};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};
use nalgebra::Vector3;

use crate::shared::gameplay::player_input::PlayerInput;

pub fn player_movement(
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut transform_query: Query<&mut Transform>,
    mut player_query: Query<(Entity, &Children, &PlayerInput, &RigidBodyHandleComponent)>,
) {
    for (entity, children, input, rigid_body_handle) in player_query.iter_mut() {
        if let Ok(mut transform) = transform_query.get_mut(entity) {
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

            let forward_direction = transform.rotation * Vec3::Z;
            let strafing_direction = transform.rotation * Vec3::X;
            let movement_vector =
                forward_direction * input_vector.y + strafing_direction * input_vector.x;

            let test = rigid_bodies.get_mut(rigid_body_handle.handle()).unwrap();
            let current_velocity: &Vector3<f32> = test.linvel();

            test.set_linvel(
                Vector3::<f32>::new(
                    movement_vector.x,
                    current_velocity.y + movement_vector.y,
                    movement_vector.z,
                ),
                true,
            );

            test.set_angvel(Vector3::<f32>::new(0.0, input.mouse_horizontal, 0.0), true);
        }

        if let Ok(mut transform) = transform_query.get_mut(children[0]) {
            transform.rotate(Quat::from_rotation_x(
                input.mouse_vertical * time.delta_seconds(),
            ));
        }
    }
}
