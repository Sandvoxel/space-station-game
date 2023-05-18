use std::f32::consts::PI;
use bevy::input::Input;
use bevy::input::mouse::MouseMotion;
use bevy::math::{EulerRot, Quat, Vec2, vec3, Vec3};
use bevy::prelude::{Camera, EventReader, info, KeyCode, MouseButton, Query, Res, Transform, Window, With, Without};
use bevy::window::CursorGrabMode;
use bevy_rapier3d::na::clamp;
use crate::player::data::{CameraRotation, Player};

const SCALE: f32 = 0.0025;

pub fn camera_controller(
    mut windows: Query<&mut Window>,
    mut players: Query<&mut Transform, With<Player>>,
    mut cameras: Query<(&mut Transform, &mut CameraRotation), (With<Camera>, Without<Player>)>,
    mut motion_evr: EventReader<MouseMotion>,
    key: Res<Input<KeyCode>>,
){
    if let Ok(mut player) = players.get_single_mut() {
        if let Ok((mut camera, mut rotation)) = cameras.get_single_mut() {
            let camera_pos = player.translation.clone() + vec3(0.0, 0.5, 0.0);
            camera.translation = camera_pos;

            for ev in motion_evr.iter() {
                rotation.look_dir.x = clamp(&rotation.look_dir.x + -&ev.delta.y * SCALE, -90f32.to_radians(),90f32.to_radians() );
                rotation.look_dir.y = &rotation.look_dir.y + &ev.delta.x * SCALE;
            }

            let look_dir =  rotation.look_dir.clone();
            let player_rotation = Quat::from_euler(EulerRot::XYZ, 0.0, look_dir.y.clone(), 0.0);

            player.rotation = player_rotation;

            let quat = Quat::from_euler(EulerRot::XYZ, look_dir.x , 0.0, 0.0);
            camera.rotation = player.rotation * quat;
        }
    }

    // if key.just_pressed(KeyCode::Q) {
    //     let mut window = windows.single_mut();
    //     window.cursor.visible = false;
    //     window.cursor.grab_mode = CursorGrabMode::Locked;
    // }
    if key.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}