

use bevy::input::Input;

use bevy::math::{vec3};
use bevy::prelude::{KeyCode, Query, Res, Time, Transform, With};

use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};


use crate::player::data::{Player};

const MOVEMENT_SPEED: f32 = 10.;
const GRAVITY: f32 = -0.97;

pub fn player_controller(
    mut players: Query<(&mut Player, &mut KinematicCharacterController, &mut Transform), With<Player>>,
    kinematic_output: Query<&KinematicCharacterControllerOutput, With<Player>>,
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
) {
    if let Ok((mut player,mut player_controller,transform)) = players.get_single_mut() {
        let mut movement_direction = transform.rotation * vec3(0., 0., 0.);

        if key.pressed(KeyCode::W) {
            movement_direction += vec3(0., 0.,-1.)
        }
        if key.pressed(KeyCode::A) {
            movement_direction += vec3(-1., 0.,0.)
        }
        if key.pressed(KeyCode::S) {
            movement_direction += vec3(0., 0.,1.)
        }
        if key.pressed(KeyCode::D) {
            movement_direction += vec3(1., 0.,0.)
        }

        if movement_direction.length() > f32::EPSILON {
            movement_direction = movement_direction.normalize();
        }

        movement_direction *= time.delta_seconds() * MOVEMENT_SPEED;


        if let Ok(output) = kinematic_output.get_single() {
            if output.grounded {
                player.velocity.y = 0.;
            }

            if key.just_pressed(KeyCode::Space) && output.grounded.clone() {
                player.velocity.y = 0.3;
            }

        }
        player.velocity.y += GRAVITY * time.delta_seconds();
        player_controller.translation = Some(transform.rotation * movement_direction + player.velocity);

    }
}


