use bevy::asset::{Assets, AssetServer, Handle};
use bevy::core::Name;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Mesh, Res, ResMut, Transform, Query, With, Without, DynamicSceneBundle};
use bevy::prelude::shape::{Cylinder};
use bevy::scene::Scene;
use bevy_rapier3d::dynamics::{CoefficientCombineRule, RigidBody};
use bevy_rapier3d::geometry::{Collider};

use bevy_rapier3d::prelude::{CollisionGroups, Friction, Group};
use crate::ship::interactables_controllers::Valve;


pub fn spawn_engine_room(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
){
    let mesh: Handle<Mesh> = server.load("valve.glb#Mesh0/Primitive0");
    let _untitled_spoke: Handle<Mesh> = server.load("untitled_spoke.glb#Mesh0/Primitive0");

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
        .insert(Friction{
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min
        })
        .insert(CollisionGroups::new(Group::ALL ^ Group::GROUP_1, Group::ALL))
        .insert(Valve::new(1., 0));

/*    commands.spawn(PbrBundle {
        mesh: untitled_spoke.clone(),
        material: materials.add(Color::RED.into()),
        transform: Transform::default().with_translation(Vec3::new(0.,2.,5.))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 0., 180.0_f32.to_radians())),
        ..default()
    })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cylinder(0.3,2.5))
        .insert(CollisionGroups::new(Group::ALL ^ Group::GROUP_1, Group::ALL))
        .insert(Valve::new(10., "valve2"));*/


    let shaft = Cylinder{
        radius: 1.,
        height: 20.,
        ..default()
    };

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shaft)),
        material: materials.add(Color::RED.into()),
        transform: Transform::default().with_translation(Vec3::new(0.,2.,5.))
            .with_rotation(Quat::from_euler(EulerRot::XYZ, 0., 0., 90.0_f32.to_radians())),
        ..default()
    }).insert(Name::new("shaft"));

}



pub fn turn_shaft(
    valves: Query<&Valve, With<Valve>>,
    mut shaft: Query<&mut Transform, (With<Name>, Without<Valve>)>
){
    if let Some(valve) = valves.iter().find(|valve| valve.identifier == 0) {
        if let Ok(mut transform) = shaft.get_single_mut(){
            transform.rotation *= Quat::from_euler(EulerRot::XYZ, 0., 0.,valve.current_value.to_radians());
        }
    }
}