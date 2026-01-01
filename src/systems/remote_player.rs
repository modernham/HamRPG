use crate::components::{AnimationIndices, AnimationTimer, Animations, RemotePlayer};
use crate::connection::tnc_integration::GameState;
use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

//Used to create smooth movement for rmeote players with infrequent updates.
#[derive(Serialize, Deserialize)]
pub struct PlayerPositionData {
    pub callsign: String,
    pub x: f32,
    pub y: f32,
    pub direction: String,
}

// System to remove inactive players
//Players that have not updated in 2 minutes  are removed form the game state.
pub fn cleanup_inactive_players(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    query: Query<(Entity, &RemotePlayer)>,
) {
    let now = Instant::now();
    let timeout = Duration::from_secs(120); // Remove after 1 minute of inactivity

    for (entity, player) in query.iter() {
        if now.duration_since(player.last_update) > timeout {
            // Remove player from game state tracking
            game_state.player_entities.remove(&player.callsign);
            game_state.known_players.retain(|p| p != &player.callsign);

            // Despawn the entity
            commands.entity(entity).despawn();
            println!("Player timed out: {}", player.callsign);
        }
    }
}

pub fn spawn_player_remote(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    pos_data: &PlayerPositionData,
) -> bevy::prelude::Entity {
    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 8, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animations = Animations {
        walk_south: AnimationIndices { first: 0, last: 3 },
        walk_north: AnimationIndices { first: 5, last: 8 },
        walk_east: AnimationIndices {
            first: 10,
            last: 13,
        },
        walk_west: AnimationIndices {
            first: 15,
            last: 18,
        },
        idle_north: AnimationIndices {
            first: 25,
            last: 26,
        },
        idle_south: AnimationIndices {
            first: 20,
            last: 21,
        },
        idle_west: AnimationIndices {
            first: 35,
            last: 36,
        },
        idle_east: AnimationIndices {
            first: 30,
            last: 31,
        },
    };
    let spawn_position = Vec3::new(pos_data.x, pos_data.y, 2.0);

    // Spawn and return the entity ID
    commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animations.walk_south.first,
                },
            ),
            animations,
            AnimationIndices { first: 0, last: 3 },
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            RigidBody::Kinematic,
            Collider::circle(16.0),
            LinearVelocity(Vec2::ZERO),
            AngularVelocity(0.0),
            GravityScale(0.0),
            Transform::from_translation(Vec3::new(pos_data.x, pos_data.y, 2.0)),
            RemotePlayer {
                callsign: pos_data.callsign.clone(),
                last_update: Instant::now(),
                target_position: spawn_position,
                start_position: spawn_position,
                move_start_time: Instant::now(),
                move_duration: 4.0,
                is_moving: false,
            },
        ))
        .id()
}

// In tcp_integration.rs or a new file like remote_player.rs
pub fn update_remote_player_movement(
    mut query: Query<(
        &mut Transform,
        &mut RemotePlayer,
        &mut AnimationIndices,
        &Animations,
    )>,
) {
    let current_time = Instant::now();

    for (mut transform, mut remote_player, mut animation_indices, animations) in query.iter_mut() {
        if remote_player.is_moving {
            // Calculate how much time has passed since movement started
            let elapsed = current_time
                .duration_since(remote_player.move_start_time)
                .as_secs_f32();
            let progress = (elapsed / remote_player.move_duration).clamp(0.0, 1.0);

            // Calculate movement direction for animation
            let movement_vector = remote_player.target_position - remote_player.start_position;

            // Update animation based on primary movement direction
            if movement_vector.x.abs() > movement_vector.y.abs() {
                // Horizontal movement is dominant
                if movement_vector.x > 0.0 {
                    // Moving right (east)
                    *animation_indices = animations.walk_east;
                } else {
                    // Moving left (west)
                    *animation_indices = animations.walk_west;
                }
            } else {
                // Vertical movement is dominant
                if movement_vector.y > 0.0 {
                    // Moving up (north)
                    *animation_indices = animations.walk_north;
                } else {
                    // Moving down (south)
                    *animation_indices = animations.walk_south;
                }
            }

            if progress >= 1.0 {
                // Movement complete - snap to final position
                transform.translation = remote_player.target_position;
                remote_player.is_moving = false;

                // Set to idle animation based on last direction
                if movement_vector.x.abs() > movement_vector.y.abs() {
                    if movement_vector.x > 0.0 {
                        *animation_indices = animations.idle_east;
                    } else {
                        *animation_indices = animations.idle_west;
                    }
                } else {
                    if movement_vector.y > 0.0 {
                        *animation_indices = animations.idle_north;
                    } else {
                        *animation_indices = animations.idle_south;
                    }
                }
            } else {
                // Interpolate between start and target positions
                // Using smooth easing (ease-out)
                let smooth_progress = 1.0 - (1.0 - progress).powi(3);

                transform.translation = remote_player
                    .start_position
                    .lerp(remote_player.target_position, smooth_progress);
            }
        }
    }
}
