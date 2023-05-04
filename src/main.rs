use std::f32::consts::PI;
use bevy::input::keyboard::KeyboardInput;
use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;

#[derive(Component, Clone)]
struct Position {
    angle: f32,
    rotation: f32
}
#[derive(Component, Clone)]
struct TargetObject;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate)
        .insert_resource(DirectionalLightShadowMap { size: 2048 })
        .run();
}

fn rotate(mut keyboard_input_events: EventReader<KeyboardInput>, mut camera: Query<&mut Transform, With<Camera>>, mut data: Query<&mut Position>, mut object: Query<&mut Transform, (With<TargetObject>, Without<Camera>)>){

    for mut pos in &mut data {
        for mut transform in camera.iter_mut() {

            for event in keyboard_input_events.iter() {
                if let Some(key) = event.key_code {
                    match key {
                        KeyCode::A => {
                            pos.rotation += -1.0 * PI/180.0;
                        }
                        KeyCode::D => {
                            pos.rotation += 1.0 * PI/180.0;
                        }
                        _ => {}
                    }
                }
            }
            let sin_angle = pos.angle.sin();
            let cos_angle = pos.angle.cos();

            let mult: f32 = 5.0;

            transform.translation.x = sin_angle * mult;
            transform.translation.z = cos_angle * mult;

            if let Ok(mut target_transform) = object.get_single_mut() {
                let camera_rotation = target_transform.translation;
                let up_direction = Vec3::Y; // Set the up direction of the camera (default is y-axis)

                let dir = transform.looking_at(camera_rotation, up_direction);

                target_transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, pos.rotation, 0.0);
                transform.rotation = dir.rotation;
            }
            //pos.angle += 0.01;

        }
    }

}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }).insert(TargetObject);
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }.into(),
        ..default()
    });
    let pos = Position {
        angle: 45.0 * PI/180.0,
        rotation: 0.0
    };
    commands.spawn(pos.clone());

    let sin_angle = pos.angle.sin();
    let cos_angle = pos.angle.cos();

    let mult: f32 = 5.0;

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(sin_angle * mult, 5.0, cos_angle * mult).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}