use std::f32::consts::PI;
use bevy::input::Input;
use bevy::input::mouse::MouseMotion;
use bevy::math::{EulerRot, Quat, Vec2, vec3, Vec3};
use bevy::prelude::{Camera, EventReader, info, KeyCode, MouseButton, Query, Res, Time, Transform, Window, With, Without};
use bevy::window::CursorGrabMode;
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier3d::dynamics::Velocity;
use bevy_rapier3d::na::clamp;
use crate::player::data::{CameraRotation, Player};

const SCALE: f32 = 0.0025;
const MOVEMENT_SPEED: f32 = 10.;
const GRAVITY: f32 = -0.97;

pub fn player_controller(
    mut players: Query<(&mut Player, &mut KinematicCharacterController, &mut Transform), With<Player>>,
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

        movement_direction *= time.delta_seconds() * MOVEMENT_SPEED;

        player.velocity.y += GRAVITY * time.delta_seconds();

        if key.just_pressed(KeyCode::Space) {
            player.velocity.y = 1.0;
        }

        info!("{}", player.velocity.y);

        player_controller.translation = Some(transform.rotation * movement_direction + player.velocity);

    }
}

pub fn player_grounded(
    mut players: Query<(&mut Player, &KinematicCharacterControllerOutput), With<Player>>,
){
    if let Ok((mut player,output)) = players.get_single_mut() {
        player.grounded = output.grounded.clone();
        if player.grounded {
            player.velocity.y = 0.;
        }
    }
}


