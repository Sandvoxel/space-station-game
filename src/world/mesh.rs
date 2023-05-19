use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

pub fn _generate_mesh(size: usize, cell_size: f32) -> Mesh {
    let mut positions = Vec::with_capacity(size * size);
    let mut indices = Vec::with_capacity(size * size * 6);

    let fbm = Fbm::<Perlin>::default();


    let map = &PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(size, size)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build();


    for y in 0..size {
        for x in 0..size {
            positions.push(Vec3::new(
                (y as f32 - size as f32 / 2.0) * cell_size,
                map.get_value(x, y) as f32,
                (x as f32 - size as f32 / 2.0) * cell_size,
            ));
        }
    }

    // Generate indices
    for y in 0..(size - 1) {
        for x in 0..(size - 1) {
            let i = y * size + x;
            indices.push(i as u32);
            indices.push((i + 1) as u32);
            indices.push((i + size) as u32);
            indices.push((i + size) as u32);
            indices.push((i + 1) as u32);
            indices.push((i + size + 1) as u32);
        }
    }

    let mut normals = vec![Vec3::ZERO; positions.len()];
    for i in 0..indices.len() / 3 {
        let i1 = indices[i * 3] as usize;
        let i2 = indices[i * 3 + 1] as usize;
        let i3 = indices[i * 3 + 2] as usize;
        let v1 = positions[i1];
        let v2 = positions[i2];
        let v3 = positions[i3];
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

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh


}