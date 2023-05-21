use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct ShipGraphics;

#[derive(Component, Clone)]
pub struct Ship {
    pub roll: f32
}