mod world;
mod player;
mod ship;

use std::f32::consts::PI;
use std::thread::spawn;
use bevy::ecs::schedule::NodeId::System;
use bevy::pbr::{CascadeShadowConfigBuilder};
use bevy::prelude::*;
use bevy::prelude::shape::{Plane,Box};
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::player::player_controller::{player_controller, player_internation};
use crate::player::player_spawner::spawn_player;
use crate::player::camera_controller::camera_controller;
use crate::ship::engine::{load_scene, LevelAsset, spawn_engine_room, spawn_scene, turn_shaft};
use crate::ship::interactables_controllers::valve_controller;

#[derive(States ,Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .init_resource::<LevelAsset>()
        .add_startup_systems((spawn_engine_room, setup, spawn_player).chain())
        .add_system(load_scene.in_schedule(OnEnter(AppState::Loading)))
        .add_system(spawn_scene.in_set(OnUpdate(AppState::Loading)))
        .add_system(camera_controller)
        .add_system(player_internation)
        .add_system(valve_controller)
        .add_system(turn_shaft)
        .add_system(player_controller.before(camera_controller))
        .run();
}
/*fn rotate(mut keyboard_input_events: EventReader<KeyboardInput>, mut camera: Query<&mut Transform, With<Camera>>, mut data: Query<&mut Position>, mut object: Query<&mut Transform, (With<TargetObject>, Without<Camera>)>){
    for mut pos in &mut data {
        for mut transform in camera.iter_mut() {

            for event in keyboard_input_events.iter() {
                if let Some(key) = event.key_code {
                    match key {
                        KeyCode::A => {
                            pos.rotation += -1.0 * PI/180.0;
                        }
                        KeyCode::D => {
                            pos.rotation += 1.0 * PI/180.0;
                        }
                        _ => {}
                    }
                }
            }
            let sin_angle = pos.angle.sin();
            let cos_angle = pos.angle.cos();

            let mult: f32 = 20.0;

            transform.translation.x = sin_angle * mult;
            transform.translation.z = cos_angle * mult;

            if let Ok(mut target_transform) = object.get_single_mut() {
                let camera_rotation = target_transform.translation;
                let up_direction = Vec3::Y; // Set the up direction of the camera (default is y-axis)

                let dir = transform.looking_at(camera_rotation, up_direction);

                target_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, pos.rotation, 0.0);
                transform.rotation = dir.rotation;
            }
            //pos.angle += 0.01;

        }
    }
}*/

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut window: Query<&mut Window>,
) {
    let mesh = Plane::from_size(100 as f32);
    let companion_cube = Box::new(1.0,1.0,1.0);

 /*   // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(mesh.clone())),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
        .insert(RigidBody::Fixed)
        .insert(Collider::from_bevy_mesh(&Mesh::from(mesh), &ComputedColliderShape::TriMesh).unwrap());

*/
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(companion_cube)),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_translation(Vec3::new(0.0,10.0,0.0)),
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.5,0.5,0.5));


    /*    commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule { radius: 1.0, rings: 0, depth: 2.0, latitudes: 16, longitudes: 32, uv_profile: Default::default() })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        }).insert(RigidBody::Dynamic)
            .insert(Collider::from_bevy_mesh(&Mesh::from(
                shape::Capsule { radius: 1.0, rings: 0, depth: 2.0, latitudes: 16, longitudes: 32, uv_profile: Default::default() }
            ), &Default::default()).unwrap())
            .insert(Restitution::coefficient(0.001))
            .insert(TargetObject);*/
    // light

    commands.spawn(PointLightBundle{
        point_light: PointLight{
            color: Color::WHITE,
            intensity: 40.0,
            range: 600.0,
            radius: 100.0,
            shadows_enabled: true,
            shadow_depth_bias: 0.0,
            shadow_normal_bias: 0.0,
        },
        transform: Transform::from_xyz( 5.,5.,0.),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 60.0,
            ..default()
        }.into(),
        ..default()
    });

    let mut window = window.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}