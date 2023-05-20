use bevy::math::{EulerRot, Quat};
use bevy::prelude::{Query, Transform, Vec2, With, Component};
use lerp::Lerp;

#[derive(Component, Clone)]
pub struct Valve{
    pub current_value: f32,
    pub sensitivity: f32,
    pub bounds: Vec2,
    pub id: u32,

    current_rotation_angle: f32
}

pub fn valve_controller(
    mut valves: Query<(&mut Valve, &mut Transform), With<Valve>>,
){
    for (mut valve, mut transform) in valves.iter_mut(){
        //TODO: Has but where if spun to fast it jumps. This could be solved by soring the current rotation angle and Slurping the value.
        let old_angle = valve.current_rotation_angle;
        valve.current_rotation_angle = valve.current_rotation_angle.lerp(valve.current_value, 0.1);
        let difference = valve.current_rotation_angle - old_angle;

        transform.rotation *= Quat::from_euler(EulerRot::XYZ, 0., difference, 0.);
    }
}


impl Default for Valve {
    fn default() -> Self {
        Valve{
            current_value: 0.0,
            sensitivity: 0.0,
            bounds: Vec2::ZERO,
            id: 0,
            current_rotation_angle: 0.0,
        }
    }
}