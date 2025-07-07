//! Camera movement system for Chimera//Echo
//! Provides first-person camera controls with mouse look and keyboard movement

use std::f32::consts::FRAC_PI_2;
use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub sprint_speed: f32,
    pub sensitivity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sprint_speed: 10.0,
            sensitivity: 0.002, // Reasonable sensitivity
        }
    }
}

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
}

/// Move camera based on keyboard input (WASD layout)
pub fn move_camera(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player), With<Camera3d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        if window.cursor.grab_mode != CursorGrabMode::Confined {
            return;
        }
    }

    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let forward: Vec3 = transform.forward().into();
        let right: Vec3 = transform.right().into();

        // Movement input (WASD layout)
        if input.pressed(KeyCode::W) {
            direction += forward; // W = forward
        }
        if input.pressed(KeyCode::S) {
            direction -= forward; // S = backward
        }
        if input.pressed(KeyCode::A) {
            direction -= right; // A = left
        }
        if input.pressed(KeyCode::D) {
            direction += right; // D = right
        }
        if input.pressed(KeyCode::Space) {
            direction += Vec3::Y; // Space = up
        }
        if input.pressed(KeyCode::ShiftLeft) {
            direction -= Vec3::Y; // Left Shift = down
        }

        // Normalize and apply movement
        if direction.length() > 0.0 {
            direction = direction.normalize();

            // Check for sprint (Ctrl key)
            let current_speed = if input.pressed(KeyCode::ControlLeft) {
                player.sprint_speed
            } else {
                player.speed
            };

            transform.translation += direction * current_speed * time.delta_seconds();
        }
    }
}

/// Handle mouse look for camera rotation
pub fn look_around(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Player, &mut CameraController), With<Camera3d>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window_query.get_single() {
        // If cursor is not confined, clear any pending mouse motion and exit
        if window.cursor.grab_mode != CursorGrabMode::Confined {
            // This is a bit of a hack, but it clears the event queue
            mouse_motion.clear(); 
            return;
        }
    }

    for (mut transform, player, mut controller) in query.iter_mut() {
        // Collect all mouse motion for this frame
        let mut total_delta = Vec2::ZERO;
        for motion in mouse_motion.read() {
            total_delta += motion.delta;
        }

        // Only process if there was actual mouse movement
        if total_delta.length() > 0.0 {
            // Calculate rotation deltas
            let delta_yaw = -total_delta.x * player.sensitivity;
            let delta_pitch = -total_delta.y * player.sensitivity;

            // Update controller rotation
            controller.yaw += delta_yaw;
            controller.pitch += delta_pitch;

            // Clamp pitch to prevent camera flipping (don't wrap yaw to avoid jumps)
            controller.pitch = controller.pitch.clamp(-FRAC_PI_2 + 0.1, FRAC_PI_2 - 0.1);

            // Apply rotation to transform - this is the key fix!
            // Build the rotation from scratch each frame instead of accumulating
            transform.rotation = Quat::from_axis_angle(Vec3::Y, controller.yaw) 
                * Quat::from_axis_angle(Vec3::X, controller.pitch);
        }
    }
}

/// Handle cursor grab/release on mouse click and escape key
pub fn cursor_grab(
    mouse_button_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            if window.cursor.grab_mode == CursorGrabMode::None {
                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
            }
        }

        if key_input.just_pressed(KeyCode::Escape) {
            if window.cursor.grab_mode != CursorGrabMode::None {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    }
}

/// Initialize camera controller for existing cameras
pub fn setup_camera_controller(
    mut commands: Commands,
    camera_query: Query<(Entity, &Transform), (With<Camera3d>, Without<CameraController>)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for (entity, transform) in camera_query.iter() {
        // Extract current rotation to initialize controller properly
        let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // If pitch is at extreme values, reset to a reasonable starting point
        let safe_pitch = if pitch.abs() > FRAC_PI_2 - 0.1 {
            0.0 // Reset to looking straight ahead
        } else {
            pitch
        };

        commands.entity(entity).insert((
            Player::default(),
            CameraController {
                yaw,
                pitch: safe_pitch,
            },
        ));
    }

    // Don't auto-grab cursor - let user control it with Escape
    if let Ok(mut window) = window_query.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

/// Debug system to print camera info (optional, for development)
#[allow(dead_code)]
pub fn debug_camera_info(
    input: Res<Input<KeyCode>>,
    query: Query<(&Transform, &CameraController), With<Camera3d>>,
) {
    if input.just_pressed(KeyCode::F1) {
        for (transform, controller) in query.iter() {
            println!(
                "Camera Debug Info:\n  Position: {:.2}\n  Yaw: {:.2}°\n  Pitch: {:.2}°",
                transform.translation,
                controller.yaw.to_degrees(),
                controller.pitch.to_degrees()
            );
        }
    }
}
