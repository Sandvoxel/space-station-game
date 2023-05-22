
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::utils::default;
use crate::world::mesh::_generate_mesh;

#[derive(Component, Clone)]
pub struct Water {
    wave_height: f32,
    wave_frequency: f32,
}


pub fn spawn_wave_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
    let water = Water{
        wave_height: 1.0,
        wave_frequency: 1.0,
    };

    let mesh = _generate_mesh(10, 5.);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::ALICE_BLUE.into()),
        ..default()
    })
        .insert(water);
}

pub fn animate_water(
    mut query: Query<&Handle<Mesh>, With<Water>>,
    mut assets: ResMut<Assets<Mesh>>,
    time: Res<Time>
){
    if let Ok(water_mesh_handle) = query.get_single_mut() {
        if let Some(water_mesh) = assets.get_mut(water_mesh_handle){
            let positions = water_mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let indices = water_mesh.indices().unwrap();
            if let VertexAttributeValues::Float32x3(data) = positions {
                let mut new_vertex_data = Vec::new();
                for i in data {
                    let temp = Vec3::new(i[0], (i[0]+time.elapsed_seconds()+i[2]).sin(), i[2]);
                    new_vertex_data.push(temp);
                }

                let mut normals = vec![Vec3::ZERO; new_vertex_data.len()];
                if let Indices::U32(indices) = indices{
                    for i in 0..indices.len() / 3 {
                        let i1 = indices[i * 3] as usize;
                        let i2 = indices[i * 3 + 1] as usize;
                        let i3 = indices[i * 3 + 2] as usize;
                        let v1 = new_vertex_data[i1];
                        let v2 = new_vertex_data[i2];
                        let v3 = new_vertex_data[i3];
                        let e1 = v2 - v1;
                        let e2 = v3 - v1;
                        let normal = e1.cross(e2).normalize();
                        normals[i1] += normal;
                        normals[i2] += normal;
                        normals[i3] += normal;
                    }
                    for normal in &mut normals {
                        *normal = normal.normalize();
                    }
                }
                water_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_vertex_data);
                water_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

            }
        }

    }
}