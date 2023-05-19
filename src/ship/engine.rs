use bevy::asset::Assets;
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Mesh, ResMut, Transform};
use bevy::prelude::shape::{Box};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::Collider;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let cube = Box::new(10.0, 1.0, 10.0);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(cube)),
        material: materials.add(Color::RED.into()),
        transform: Transform::default().with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
        ..default()
    })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 0.5, 5.0));
}