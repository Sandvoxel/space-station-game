use bevy::ecs::system::Spawn;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::ship::interactables_controllers::Valve;



pub fn update_particles(
    valve_query: Query<(&Valve)>,
    mut particles: Query<(&mut CompiledParticleEffect,&mut EffectSpawner)>,
) {
    if let Some(valve) = valve_query.iter().find(|valve| valve.identifier == 0) {
        if let Ok((mut cpe, mut spawner)) = particles.get_single_mut() {
            cpe.set_property("my_value", (valve.current_value/2.0).into());
            //spawner.set_active(valve.current_value == 0.0);
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


    let spawner = Spawner::rate(30.0.into());

    let effect = effects.add(EffectAsset {
        name: "Water".to_string(),
        capacity: 30000,
        spawner,
        ..default()
    }.with_property("my_value", graph::Value::Float(5.0))
        .init(InitAttributeModifier{
            attribute: Attribute::LIFETIME,
            value: "my_value".into(),
        })
        .init(InitPositionCone3dModifier {
        height: 10.1,
        base_radius: 10.1,
        top_radius: 1.0,
        dimension: ShapeDimension::Surface,
    }
    ).init(InitVelocitySphereModifier {
        center: Vec3::ZERO,
        speed: (-10.0_f32).into(),
    })
     .update(AccelModifier::constant(Vec3::new(0., -9.8, 0.)))
     .render(ColorOverLifetimeModifier { gradient})
        .render(BillboardModifier)
    );

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect).with_spawner(spawner),
        transform: Transform::from_translation(Vec3::new(0., 5., -10.)),
        ..default()
    })
        .insert(Particle{});
}
