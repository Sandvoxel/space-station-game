use bevy::asset::{Asset, Assets, AssetServer, Handle, LoadState};
use bevy::core::Name;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{PbrBundle, PointLight, StandardMaterial};
use bevy::prelude::{Color, Commands, default, Mesh, Res, ResMut, Transform, Query, With, Without, DynamicSceneBundle, SceneBundle, info, NextState, TransformBundle, BuildChildren, ComputedVisibility, Visibility, SpatialBundle};
use bevy::prelude::shape::{Cylinder};
use bevy_rapier3d::dynamics::{CoefficientCombineRule, RigidBody};
use bevy_rapier3d::geometry::{Collider, VHACDParameters};

use bevy_rapier3d::prelude::{CollisionGroups, ComputedColliderShape, Friction, Group};
use serde::Deserialize;
use crate::AppState;
use crate::ship::interactables_controllers::Valve;
use crate::world::types::{Ship, ShipGraphics};

#[derive(bevy::ecs::system::Resource, Default)]
pub struct LevelAsset(pub Handle<bevy::gltf::Gltf>);

pub fn load_scene(
    mut commands: Commands,
    server: Res<AssetServer>,
) {
    let scene = server.load("EngineRoom.gltf");
    commands.insert_resource(LevelAsset(scene));
}

#[derive(Deserialize, Copy, Clone, Debug)]
struct ColliderType {
    collider_type: ColliderTypes
}
#[derive(Deserialize, Copy, Clone, Debug)]
enum ColliderTypes {
    Box {dim: [f32; 3]},
    Cylinder {dim: [f32; 2]},
    Valve {}
}

pub fn spawn_scene(
    mut commands: Commands,
    my: Res<LevelAsset>,
    assets: Res<Assets<bevy::gltf::Gltf>>,
    node_assets: Res<Assets<bevy::gltf::GltfNode>>,
    mesh_assets: Res<Assets<bevy::gltf::GltfMesh>>,
    server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if server.get_load_state(&my.0) == LoadState::Loaded {
        if let Some(gltf) = assets.get(&my.0) {
            for test in gltf.named_nodes.clone() {
                if let Some(node) = node_assets.get(&test.1) {
                    if let Some(mesh) = &node.mesh {
                        if let Some(mesh) = mesh_assets.get(mesh) {
                            for prim in &mesh.primitives {
                                if let Some(extra) = mesh.extras.clone() {

                                    let collider: ColliderTypes = serde_json::from_str(extra.value.as_str()).unwrap();

                                    info!("{:?}", node.transform);
                                    info!("{:?}", collider);

                                    match collider {
                                        ColliderTypes::Box{ dim } => {
                                            commands.spawn(RigidBody::Fixed)
                                                .insert(TransformBundle::from_transform(node.transform))
                                                .insert(
                                                    Collider::cuboid(dim[0]/2., dim[2]/2., dim[1]/2.));
                                        },
                                        ColliderTypes::Valve{} => {
                                            commands.spawn(PbrBundle {
                                                mesh: prim.mesh.clone(),
                                                material: prim.material.clone().unwrap(),
                                                transform: node.transform,
                                                ..default()
                                            })
                                                .insert(RigidBody::KinematicPositionBased)
                                                .insert(Collider::cylinder(0.3,2.5))
                                                .insert(CollisionGroups::new(Group::ALL ^ Group::GROUP_1, Group::ALL))
                                                .insert(Valve::new(1., 0));


                                        }
                                        ColliderTypes::Cylinder { dim } => {
                                            commands.spawn(RigidBody::Fixed)
                                                .insert(TransformBundle::from_transform(node.transform))
                                                .insert(
                                                    Collider::cylinder(dim[0]/2., dim[1]/2.));
                                        }
                                    }

                                } else {
                                    commands.spawn(PbrBundle{
                                        mesh: prim.mesh.clone(),
                                        material: prim.material.clone().unwrap(),
                                        transform: node.transform,
                                        visibility: Visibility::Visible,
                                        ..default()
                                    });
                                }
                            }

                        }
                    }
                }


                next_state.set(AppState::InGame)
            }
        }
    }
}


pub fn turn_shaft(
    valves: Query<&Valve, With<Valve>>,
    mut shaft: Query<&mut PointLight, (With<Name>, Without<Valve>)>
){

    if let Some(valve) = valves.iter().find(|valve| valve.identifier == 0) {
        if let Ok(mut light) = shaft.get_single_mut(){
            light.intensity = valve.current_value * 100.0;
        }
    }
}