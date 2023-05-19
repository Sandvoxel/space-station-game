use bevy::prelude::*;
use bevy_rapier3d::control::{KinematicCharacterController};

use bevy_rapier3d::prelude::{Collider};


#[derive(Component, Clone)]
pub struct Player{
    pub velocity: Vec3,
    pub grounded: bool
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity: Vec3::ZERO,
            grounded: false
        }
    }
}

#[derive(Component, Clone)]
pub struct CameraRotation {
    pub(crate) look_dir: Vec2
}

#[derive(Bundle, Clone)]
pub struct PlayerBundle<M: Material> {
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,

    pub rigid_body_type: KinematicCharacterController,
    pub collider: Collider,

}

impl<M: Material> Default for PlayerBundle<M> {
    fn default() -> Self {
        Self {
            mesh: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
            collider: Default::default(),
            rigid_body_type: KinematicCharacterController::default(),
        }
    }
}