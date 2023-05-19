use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Mesh, Res, ResMut, Transform};
use bevy::prelude::shape::{Box};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};

pub fn spawn_engine_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
){
    let mesh: Handle<Mesh> = server.load("valve.glb#Mesh0/Primitive0");
    let untitled_spoke: Handle<Mesh> = server.load("untitled_spoke.glb#Mesh0/Primitive0");
    let sams_stinky_valve: Handle<Mesh> = server.load("Valve_Sam.glb#Mesh0/Primitive0");
    let cube = Box::new(10.0, 1.0, 10.0);

    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: materials.add(Color::RED.into()),
        transform: Transform::default().with_translation(Vec3::new(0.,10.,0.)).with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.3,2.5));

    for i in 0..1000 {
        commands.spawn(PbrBundle {
            mesh: untitled_spoke.clone(),
            material: materials.add(Color::RED.into()),
            transform: Transform::default().with_translation(Vec3::new(0.,5. + i as f32,5.)).with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
            ..default()
        })
            .insert(RigidBody::Dynamic)
            .insert(Collider::cylinder(0.3,2.5));
    }


    commands.spawn(PbrBundle {
        mesh: sams_stinky_valve.clone(),
        material: materials.add(Color::BEIGE.into()),
        transform: Transform::default().with_translation(Vec3::new(5.,5.,0.)).with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.3,2.5));
}