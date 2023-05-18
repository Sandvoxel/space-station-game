use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3, vec3};
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Camera3dBundle, Color, Commands, Mesh, ResMut, shape, Transform, Window};
use bevy::utils::default;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::dynamics::{RigidBody, Sleeping, Velocity};
use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};
use bevy_rapier3d::prelude::Ccd;
use crate::player::data::{CameraRotation, Player, PlayerBundle};
use crate::TargetObject;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {


    let mesh = shape::Capsule {
        radius: 1.0,
        rings: 0,
        depth: 2.0,
        latitudes: 16,
        longitudes: 32,
        uv_profile: Default::default()
    };

    commands.spawn(PlayerBundle{
        mesh: meshes.add(Mesh::from(mesh.clone())),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0,10.0,0.0),
        collider: Collider::capsule( vec3(0.,0.5,0.0), vec3(0.,-0.5,0.0), 1.),
        ..default()
    })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Player::default());

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(10.0,10.0,10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }).insert(CameraRotation {
        look_dir: Vec2::ZERO,
    });

}