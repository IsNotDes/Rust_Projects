use bevy::prelude::*;
use crate::movement::CameraLook;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    use bevy::prelude::*;
use crate::movement::CameraLook;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    let initial_transform = Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::new(1.0, 1.0, 1.0), Vec3::Y);
    let (initial_yaw, initial_pitch, _) = initial_transform.rotation.to_euler(EulerRot::YXZ);

    commands.spawn((Camera3dBundle {
        transform: initial_transform,
        ..default()
    }, CameraLook { pitch: initial_pitch, yaw: initial_yaw }));
}
}