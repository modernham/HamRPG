// Game-level TNC event handlers
// Processes incoming radio packets and sends position updates

use super::compression::encode_position;
use super::resources::PositionUpdateTime;
use crate::components::{Entity, RemotePlayer};
use crate::connection::message::MessageType;
use crate::connection::tnc_plugin::{TncIncomingEvent, TncOutgoingEvent};
use crate::systems::remote_player::{PlayerPositionData, spawn_player_remote};
use bevy::prelude::*;
use std::collections::HashMap;
use rand::{rng, Rng};
use std::time::{Duration, Instant};

// Process incoming TNC messages
pub fn handle_tnc_events(
    mut commands: Commands,
    mut incoming_events: EventReader<TncIncomingEvent>,
    mut game_state: ResMut<GameState>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut transforms: Query<(&Transform, &mut RemotePlayer)>,
) {
    for event in incoming_events.read() {
        match event.message_type {
            MessageType::Position => {
                if let Ok(pos_data) = serde_json::from_str::<PlayerPositionData>(&event.message) {
                    // Don't process our own position updates
                    if pos_data.callsign == game_state.player_callsign {
                        continue;
                    }

                    if !game_state.player_entities.contains_key(&pos_data.callsign) {
                        // Spawn new remote player
                        let entity = spawn_player_remote(
                            &mut commands,
                            &asset_server,
                            &mut texture_atlas_layouts,
                            &pos_data,
                        );

                        game_state.known_players.push(pos_data.callsign.clone());
                        game_state
                            .player_entities
                            .insert(pos_data.callsign.clone(), entity);

                        println!("[i] New player joined: {}", pos_data.callsign);
                    } else {
                        // Update existing player's position with smooth movement
                        if let Some(&entity) = game_state.player_entities.get(&pos_data.callsign) {
                            if let Ok((transform, mut remote_player)) = transforms.get_mut(entity) {
                                let new_position =
                                    Vec3::new(pos_data.x, pos_data.y, transform.translation.z);

                                // Only start movement if the position actually changed
                                if transform.translation.distance(new_position) > 1.0 {
                                    remote_player.start_position = transform.translation;
                                    remote_player.target_position = new_position;
                                    remote_player.move_start_time = Instant::now();
                                    remote_player.move_duration = 2.0; // 2 seconds to move
                                    remote_player.is_moving = true;
                                    remote_player.last_update = Instant::now();

                                    println!(
                                        "[i] Updating position for {}: {:?} -> {:?}",
                                        pos_data.callsign,
                                        remote_player.start_position,
                                        new_position
                                    );
                                }
                            } else {
                                println!(
                                    "[!] Entity exists in map but not in the world - might be despawned"
                                );
                                game_state.player_entities.remove(&pos_data.callsign);
                                game_state.known_players.retain(|p| p != &pos_data.callsign);
                            }
                        }
                    }
                }
            }
            MessageType::Chat => {
                println!("[i] Chat message received: {}", event.message);

                // Add the message to our chat history
                game_state.chat_messages.push(event.message.clone());

                // Limit chat history to prevent excessive memory usage
                if game_state.chat_messages.len() > 100 {
                    game_state.chat_messages.remove(0);
                }
            }
        }
    }
}

// Send player position updates via TNC
pub fn send_position_updates(
    query: Query<(&Transform, &Entity)>,
    mut event_writer: EventWriter<TncOutgoingEvent>,
    mut last_update: Local<Option<Instant>>,
    game_state: Res<GameState>,
    pos_update_time: Res<PositionUpdateTime>,
) {
    // Only send position updates at intervals with random timing
    let now = Instant::now();
    let should_update = if let Some(last) = *last_update {
        let mut rng = rng();
        let random_offset = rng.random_range(-4..=4);
        let randomized_interval = (pos_update_time.0 as i64 + random_offset).max(1);
        now.duration_since(last) >= Duration::from_secs(randomized_interval.try_into().unwrap())
    } else {
        true // First update should happen immediately
    };

    // Skip if not time to update yet
    if !should_update {
        return;
    }

    // Update the last update time
    *last_update = Some(now);

    // Get player position and send update
    for (transform, _) in query.iter() {
        // Encode position using custom compact protocol
        let encoded = encode_position(
            &game_state.player_callsign,
            transform.translation.x,
            transform.translation.y,
            &get_player_direction(),
        );

        event_writer.write(TncOutgoingEvent {
            message: encoded,
            message_type: MessageType::Position,
        });
        println!(
            "[i] Position update sent for {}",
            game_state.player_callsign
        );
    }
}

// Helper function to determine player direction
fn get_player_direction() -> String {
    "south".to_string() // Replace with actual direction logic
}

// Game state resource
#[derive(Resource)]
pub struct GameState {
    pub chat_messages: Vec<String>,
    pub known_players: Vec<String>,
    pub player_entities: HashMap<String, bevy::prelude::Entity>,
    pub player_callsign: String,
}
