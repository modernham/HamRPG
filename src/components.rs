// Components for the Radio RPG game
use bevy::prelude::*;
use std::time::Instant;

// Marker component for the local player entity
#[derive(Component)]
pub struct Entity;
// Animation frame range within a sprite sheet
#[derive(Component, Copy, Clone, PartialEq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}
// Remote player component with smooth interpolation data
// Maintains position history to create smooth movement between infrequent radio updates
#[derive(Component)]
pub struct RemotePlayer {
    pub callsign: String,
    pub last_update: Instant,
    pub target_position: Vec3,
    pub start_position: Vec3,
    pub move_start_time: Instant,
    pub move_duration: f32, // Time to complete the movement
    pub is_moving: bool,
}
// Collection of 8-directional animation ranges for player sprites
#[derive(Component)]
pub struct Animations {
    pub walk_west: AnimationIndices,
    pub walk_east: AnimationIndices,
    pub walk_north: AnimationIndices,
    pub walk_south: AnimationIndices,
    pub idle_west: AnimationIndices,
    pub idle_east: AnimationIndices,
    pub idle_north: AnimationIndices,
    pub idle_south: AnimationIndices,
}
// Timer for controlling animation frame rate
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
