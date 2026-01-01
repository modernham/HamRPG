use crate::components::{AnimationIndices, AnimationTimer, Animations};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

// System to animate sprites based on their animation indices and timers.
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (current_animation, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= current_animation.last {
                    atlas.index = current_animation.first
                } else if atlas.index < current_animation.first {
                    atlas.index = current_animation.first
                } else {
                    atlas.index = atlas.index + 1
                };
            }
        }
    }
}

// System to update the current animation based on the entity's linear velocity.
pub fn update_animation(
    mut query: Query<(&Animations, &mut AnimationIndices, &mut LinearVelocity)>,
) {
    for (animations, mut current_animation, linear_velocity) in &mut query {
        if linear_velocity.0.x > 0.0 {
            *current_animation = animations.walk_east;
        } else if linear_velocity.0.x < 0.0 {
            *current_animation = animations.walk_west;
        } else if linear_velocity.0.y > 0.0 {
            *current_animation = animations.walk_north;
        } else if linear_velocity.0.y < 0.0 {
            *current_animation = animations.walk_south;
        } else if linear_velocity.0.x == 0.0 && linear_velocity.0.y == 0.0 {
            if *current_animation == animations.walk_east {
                *current_animation = animations.idle_east;
            } else if *current_animation == animations.walk_west {
                *current_animation = animations.idle_west;
            } else if *current_animation == animations.walk_north {
                *current_animation = animations.idle_north;
            } else if *current_animation == animations.walk_south {
                *current_animation = animations.idle_south;
            }
        }
    }
}
