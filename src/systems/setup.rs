use crate::connection::tnc_integration::GameState;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use iyes_perf_ui::prelude::*;

// Initialize game scene: camera, performance UI, and tilemap
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, 
            ..default()
        },
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: 0.25,
            ..OrthographicProjection::default_3d()
        }),
    ));
    commands.spawn((
        PerfUiRoot {
            display_labels: false,
            layout_horizontal: true,
            values_col_width: 32.0,
            ..default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFPS::default(),
    ));
    commands.spawn((
        TiledMapHandle(asset_server.load("map.tmx")),
        TilemapAnchor::Center,
    ));
}


pub fn adjust_layer_z_ordering(
    mut layer_query: Query<(&mut Transform, &TiledMapTileLayer, Entity), Changed<TiledMapTileLayer>>,
) {
    for (mut transform, _layer, entity) in layer_query.iter_mut() {
        // Since TiledMapTileLayer does not have an 'id' field, use the entity's id as a fallback for ordering.
        let layer_index = entity.index();
        print!("Layer entity index: {}\n", layer_index);
        if layer_index == 35 {
            transform.translation.z = 0.0;  // First layer (layer0) at Z = 0
        } else {
            transform.translation.z = 3.0;  // Other layers at Z = 11.0, 12.0, etc.
        }
    }
}

// Send welcome message to chat on game start
pub fn send_welcome_message(mut game_state: ResMut<GameState>) {
    // Send a welcome message to the chat
    game_state
        .chat_messages
        .push("Welcome to Radio RPG!".to_string());
}
