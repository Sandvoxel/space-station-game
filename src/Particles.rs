use bevy::ecs::system::Spawn;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::ship::interactables_controllers::Valve;



pub fn update_particles(
    valve_query: Query<(&Valve)>,
    mut particles: Query<&mut ParticleEffect>,
) {
    if let Some(valve) = valve_query.iter().find(|valve| valve.identifier == 0) {
        if let Ok(mut spawner) = particles.get_single_mut() {
            //spawner.maybe_spawner().unwrap().set_active(valve.current_value > 1.);
        }
    }
}

#[derive(Component, Clone)]
pub struct Particle;

pub fn setup_particles(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut gradient = Gradient::new();

    gradient.add_key(0.0, Vec4::from_array(Color::BLUE.as_rgba_f32())); // Red;
    gradient.add_key(0.5, Vec4::from_array(Color::WHITE.as_rgba_f32())); // White
    gradient.add_key(1.0, Vec4::ZERO); // Transparent Black

    let effect = effects.add(EffectAsset {
        name: "Water".to_string(),
        capacity: 30000,
        spawner: Spawner::rate(100.0.into()),
        ..default()
    }.init(InitPositionCone3dModifier {
        height: 5.0,
        base_radius: 3.0,
        top_radius: 1.0,
        dimension: ShapeDimension::Surface,
    }
    ).init(InitVelocitySphereModifier {
        center: Vec3::ZERO,
        speed: 5.0_f32.into(),
    }
    ).init(InitLifetimeModifier { lifetime: 10_f32.into() })
     .update(AccelModifier::constant(Vec3::new(0., -9.8, 0.)))
     .render(ColorOverLifetimeModifier { gradient})
        .render(BillboardModifier)
    );

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect),
        transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
        ..default()
    })
        .insert(Particle{});
}