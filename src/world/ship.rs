use bevy::math::EulerRot;
use bevy::prelude::{GlobalTransform, info, Quat, Query, Transform, With, Without};
use crate::world::types::{Ship, ShipGraphics};

pub fn ship_manager(
    mut ship_query: Query<(&mut Transform, &mut Ship), (With<Ship>, Without<ShipGraphics>)>
){
    if let Ok((mut ship_transform, mut ship)) = ship_query.get_single_mut() {

        //info!("{}", ship.roll);
        ship.roll += 0.1;
        ship_transform.rotation = Quat::from_euler(EulerRot::XYZ, ship.roll.sin().to_radians(), 0., 0.)

    }
}