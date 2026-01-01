use crate::components::{AnimationIndices, AnimationTimer, Animations, Entity, RemotePlayer};
use crate::constants::SPEED;
use crate::systems::gui::ChatInputState;
use avian2d::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

//Creates a new player entity with animations and physics components.
pub fn add_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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
    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animations.walk_south.first,
            },
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        animations,
        AnimationIndices { first: 0, last: 3 },
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        RigidBody::Kinematic,
        Collider::circle(16.0),
        LinearVelocity(Vec2::ZERO),
        AngularVelocity(0.0),
        GravityScale(0.0),
        Entity,
    ));
}

//Listens for keyboard input to move the player entity.
pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    chat_state: Res<ChatInputState>,
    mut player_query: Query<(&mut LinearVelocity, &mut AnimationTimer), Without<RemotePlayer>>,
) {
    // Don't process movement if chat is active
    if chat_state.active {
        return;
    }

    for (mut linear_velocity, mut animationtimer) in &mut player_query {
        let mut direction = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            if keyboard.just_pressed(KeyCode::KeyA) {
                animationtimer.set_duration(Duration::from_secs_f32(0.15));
            }
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            if keyboard.just_pressed(KeyCode::KeyD) {
                animationtimer.set_duration(Duration::from_secs_f32(0.15));
            }
        }
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
            if keyboard.just_pressed(KeyCode::KeyW) {
                animationtimer.set_duration(Duration::from_secs_f32(0.15));
            }
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
            if keyboard.just_pressed(KeyCode::KeyS) {
                animationtimer.set_duration(Duration::from_secs_f32(0.15));
            }
        }
        // Normalize direction to ensure consistent speed
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        // Update linear velocity with capped speed
        linear_velocity.0 = direction * SPEED;
    }
}
