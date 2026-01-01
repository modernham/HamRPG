use bevy::prelude::*;

// Position update interval in seconds
#[derive(Resource)]
pub struct PositionUpdateTime(pub u64);
