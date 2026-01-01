use crate::components::Entity;
use crate::constants::CAMERA_DECAY_RATE;
use bevy::prelude::*;

//Updates every frame to follow the player entity with a smooth camera effect.
pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Entity>)>,
    player: Single<&Transform, (With<Entity>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
