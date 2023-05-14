use bevy::prelude::*;

#[derive(Component, Clone)]
struct World {
    chunk: Chunk,
    time: u64
}

#[derive(Component, Clone)]
struct Chunk {
    voxels: [[[Voxel; 32]; 32]; 32]
}

#[derive(Component, Clone)]
struct Voxel {
    terrain: VoxelType
}

#[derive(Component, Clone)]
enum VoxelType{
    Air,
    Ground
}