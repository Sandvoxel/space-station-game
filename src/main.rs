mod world;
mod player;
mod ship;
mod Particles;

use std::f32::consts::PI;
use std::thread::spawn;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::schedule::NodeId::System;
use bevy::pbr::{CascadeShadowConfigBuilder};
use bevy_hanabi::prelude::*;
use bevy::prelude::*;
use bevy::prelude::shape::{Plane,Box};
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::Particles::{ setup_particles, update_particles, Particle };
use crate::player::player_controller::{player_controller, player_internation};
use crate::player::player_spawner::spawn_player;
use crate::player::camera_controller::camera_controller;
use crate::ship::engine::{load_scene, LevelAsset, spawn_scene, turn_shaft};
use crate::ship::interactables_controllers::valve_controller;
use crate::world::ship::ship_manager;
use crate::world::water::{animate_water, spawn_wave_mesh};

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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(HanabiPlugin)
        .add_state::<AppState>()
        .init_resource::<LevelAsset>()
        .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(spawn_player.in_schedule(OnEnter(AppState::InGame)))
        .add_system(spawn_wave_mesh.in_schedule(OnEnter(AppState::InGame)))
        .add_system(setup_particles.in_schedule(OnEnter(AppState::InGame)))
        .add_system(load_scene.in_schedule(OnEnter(AppState::Loading)))
        .add_system(spawn_scene.in_set(OnUpdate(AppState::Loading)))
        .add_system(update_particles.in_set(OnUpdate(AppState::InGame)))
        .add_system(camera_controller.in_set(OnUpdate(AppState::InGame)))
        .add_system(player_internation.in_set(OnUpdate(AppState::InGame)))
        .add_system(valve_controller.in_set(OnUpdate(AppState::InGame)))
        .add_system(ship_manager.in_set(OnUpdate(AppState::InGame)))
        .add_system(turn_shaft.in_set(OnUpdate(AppState::InGame)))
        .add_system(player_controller.in_set(OnUpdate(AppState::InGame)))
        .add_system(animate_water.in_set(OnUpdate(AppState::InGame)))
        .run();
}

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
        transform: Transform::from_translation(Vec3::new(10.0,10.0,0.0)),
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
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz( 0.,5.,0.),
        ..default()
    }).insert(Name::new("shaft"));;

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