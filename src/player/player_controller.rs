

use bevy::input::Input;
use bevy::input::mouse::MouseWheel;
use bevy::math::{Quat, vec3, Vec3};
use bevy::prelude::{Camera, Entity, EventReader, KeyCode, Query, Res, ResMut, Time, Transform, With, Without};
use bevy_rapier3d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier3d::na::clamp;
use bevy_rapier3d::plugin::RapierContext;
use bevy_rapier3d::prelude::{QueryFilter};
use crate::player::data::{Player};
use crate::ship::interactables_controllers::Valve;

const MOVEMENT_SPEED: f32 = 10.0;
const GRAVITY: f32 = -0.7;

pub fn player_controller(
    mut players: Query<(&mut Player, &mut KinematicCharacterController, &mut Transform), With<Player>>,
    kinematic_output: Query<&KinematicCharacterControllerOutput, With<Player>>,
    time: Res<Time>,
    key: Res<Input<KeyCode>>,
) {
    if let Ok((mut player,mut player_controller,transform)) = players.get_single_mut() {
        let mut movement_direction = Vec3::ZERO;

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

            if key.just_pressed(KeyCode::Space) && output.grounded {
                player.velocity.y = 0.3;
            }

        }
        player.velocity.y += GRAVITY * time.delta_seconds();
        player.velocity.y = clamp(player.velocity.y, -5.0, 10.0);
        player_controller.translation = Some(transform.rotation * movement_direction + player.velocity);

    }
}

pub fn player_internation(
    cameras: Query<&Transform, (With<Camera>, Without<Player>)>,
    kinematic_output: Query<(&KinematicCharacterControllerOutput), With<Player>>,
    player_collider: Query<Entity, With<Player>>,
    mut valves: Query<(&mut Valve, Entity), (With<Valve>, Without<Camera>, Without<Player>)>,
    mut scroll_evr: EventReader<MouseWheel>,
    rapier_context: Res<RapierContext>,
) {
    if let Ok(collider) = player_collider.get_single() {
        if let Ok(camera) = cameras.get_single() {
            let ray_pos = if let Ok(output) = kinematic_output.get_single() {
                camera.translation + output.effective_translation
            } else {
                camera.translation
            };
            let ray_dir = camera.rotation * Vec3::NEG_Z;

            let max_toi = 10.0;
            let solid = true;
            let filter = QueryFilter::default()
                .exclude_collider(collider);


            if let Some((entity, _toi)) = rapier_context.cast_ray(
                ray_pos, ray_dir, max_toi, solid, filter
            ) {
                for (mut valve, valve_entity) in valves.iter_mut() {
                    if valve_entity == entity {
                        for ev in scroll_evr.iter() {
                            valve.current_value += ev.y * valve.sensitivity;
                        }
                    }
                }
            }
        }
    }
}


