use bevy::{prelude::*, input::mouse::MouseMotion, window::PrimaryWindow};

#[derive(Component)]
pub struct CameraLook {
    pub pitch: f32,
    pub yaw: f32,
}

pub fn move_camera(mut query: Query<&mut Transform, With<Camera>>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let mut transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::KeyZ) {
        direction += *transform.forward();
    }
    if input.pressed(KeyCode::KeyQ) {
        direction += *transform.back();
    }
    if input.pressed(KeyCode::KeyS) {
        direction += *transform.left();
    }
    if input.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * time.delta_seconds() * 5.0;
}

pub fn look_around(mut query: Query<(&mut Transform, &mut CameraLook), With<Camera>>, mut motion_evr: EventReader<MouseMotion>, _time: Res<Time>) {
    let (mut transform, mut look) = query.single_mut();
    for ev in motion_evr.read() {
        let window_scale = 0.01;
        look.pitch -= (ev.delta.y * window_scale).to_radians();
        look.yaw -= (ev.delta.x * window_scale).to_radians();

        look.pitch = look.pitch.clamp(-1.5, 1.5);

        transform.rotation = Quat::from_axis_angle(Vec3::Y, look.yaw) * Quat::from_axis_angle(Vec3::X, look.pitch);
    }
}

pub fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = q_windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
        window.cursor.visible = true;
    }
}