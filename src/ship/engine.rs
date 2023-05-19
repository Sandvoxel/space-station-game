use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Label, Mesh, Res, ResMut, Transform, Component};
use bevy::prelude::shape::{Box};
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};

#[derive(Component, Clone)]
pub struct Valve{
    pub value: f32,
    pub bounds: Vec2,
    pub id: u32
}

pub fn spawn_engine_room(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
){
    let mesh: Handle<Mesh> = server.load("valve.glb#Mesh0/Primitive0");

    commands.spawn(PbrBundle {
        mesh,
        material: materials.add(Color::RED.into()),
        transform: Transform::default()
            .with_translation(Vec3::new(5.,2.,0.))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, f32::to_radians(90.), 0., 0.))
            .with_scale(Vec3::new(0.2,0.2,0.2)),
        ..default()
    })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cylinder(0.3,2.5))
        .insert(Valve {
            value: 0.0,
            bounds: Default::default(),
            id: 0
        });

/*    commands.spawn(PbrBundle {
        mesh: untitled_spoke.clone(),
        material: materials.add(Color::RED.into()),
        transform: Transform::default().with_translation(Vec3::new(0.,5.,5.)).with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.3,2.5));



    commands.spawn(PbrBundle {
        mesh: sams_stinky_valve.clone(),
        material: materials.add(Color::BEIGE.into()),
        transform: Transform::default().with_translation(Vec3::new(5.,5.,0.)).with_rotation(Quat::from_euler(EulerRot::XYZ, 10., 0., 0.)),
        ..default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cylinder(0.3,2.5));*/
}