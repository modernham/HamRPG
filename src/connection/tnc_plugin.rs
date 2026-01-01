// TNC (Terminal Node Controller) integration plugin
// Manages connection to KISS-compatible TNC software for AX.25 packet transmission

use super::compression::decode_packet;
use super::message::{GameMessage, MessageType};
use crate::menu::{AppState, MenuConfig};
use ax25::frame::{
    Address, Ax25Frame, CommandResponse, FrameContent, ProtocolIdentifier, UnnumberedInformation,
};
use ax25_tnc::tnc::{Tnc, TncAddress};
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use std::sync::{Arc, Mutex};
use std::thread;

// TNC communication events
#[derive(Event)]
pub struct TncIncomingEvent {
    pub message: String,
    pub message_type: MessageType,
}

#[derive(Event)]
pub struct TncOutgoingEvent {
    pub message: String,
    pub message_type: MessageType,
}

// Channel resources for TNC communication
#[derive(Resource)]
pub struct TncChannels {
    pub sender: Sender<GameMessage>,
    pub receiver: Receiver<GameMessage>,
}

pub struct TncPlugin;

impl Plugin for TncPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TncIncomingEvent>()
            .add_event::<TncOutgoingEvent>()
            .add_systems(OnEnter(AppState::InGame), setup_tnc_connection)
            .add_systems(
                Update,
                (handle_incoming_tnc_messages, handle_outgoing_tnc_messages)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn setup_tnc_connection(mut commands: Commands, menu_config: Res<MenuConfig>) {
    println!("[i] Setting up TNC connection...");

    // Create channels for communication between game and TNC thread
    let (tx_to_game, rx_from_tnc) = unbounded::<GameMessage>();
    let (tx_to_tnc, rx_from_game) = unbounded::<GameMessage>();

    // Store channels as a resource
    commands.insert_resource(TncChannels {
        sender: tx_to_tnc.clone(),
        receiver: rx_from_tnc,
    });

    let tnc_address_str = menu_config.get_tnc_address();
    let callsign = menu_config.callsign.clone();

    // Connect to TNC in a separate thread to avoid blocking the game
    thread::spawn(move || {
        // Parse TNC address
        let addr = match tnc_address_str.parse::<TncAddress>() {
            Ok(addr) => addr,
            Err(e) => {
                println!("[!] Failed to parse TNC address: {}", e);
                return;
            }
        };

        // Parse callsign as AX.25 address
        let source_addr = match callsign.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                println!("[!] Failed to parse callsign: {}", e);
                return;
            }
        };

        // Use a broadcast-like destination for game messages
        let dest_addr = match "HAMRPG-0".parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => {
                println!("[!] Failed to parse destination address: {}", e);
                return;
            }
        };

        // Connect to TNC
        println!("[i] Connecting to TNC: {}", tnc_address_str);
        let tnc = match Tnc::open(&addr) {
            Ok(tnc) => {
                println!("[i] Connected to TNC successfully!");
                tnc
            }
            Err(e) => {
                println!("[!] Failed to connect to TNC: {}", e);
                return;
            }
        };

        let tnc = Arc::new(Mutex::new(tnc));
        let tnc_clone = Arc::clone(&tnc);

        // Spawn a thread to listen for incoming frames from TNC
        let tx_to_game_clone = tx_to_game.clone();
        thread::spawn(move || {
            println!("[i] Starting TNC receiver thread...");
            let receiver = tnc_clone.lock().unwrap().incoming();

            while let Ok(frame) = receiver.recv().unwrap() {
                if let Some(frame_data) = frame.info_string_lossy() {
                    println!("[i] Received from TNC: {} bytes - {}", frame_data.len(), frame_data);

                    // Decode the packet using our custom protocol
                    match decode_packet(&frame_data) {
                        Ok(decoded) => {
                            use crate::connection::compression::DecodedPacket;

                            let game_message = match decoded {
                                DecodedPacket::Position(pos_data) => {
                                    // Convert position data to JSON string for compatibility
                                    match serde_json::to_string(&pos_data) {
                                        Ok(json) => GameMessage {
                                            content: json,
                                            message_type: MessageType::Position,
                                        },
                                        Err(e) => {
                                            println!("[!] Failed to serialize position: {}", e);
                                            continue;
                                        }
                                    }
                                }
                                DecodedPacket::Chat(message) => GameMessage {
                                    content: message,
                                    message_type: MessageType::Chat,
                                },
                            };

                            if let Err(e) = tx_to_game_clone.send(game_message) {
                                println!("[!] Failed to send message to game: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            println!("[!] Failed to decode packet: {}", e);
                        }
                    }
                }
            }
            println!("[!] TNC receiver thread ended");
        });

        // Process outgoing messages in this thread
        println!("[i] Starting TNC sender thread...");
        for message in rx_from_game {
            // The message content is already in the internal format (JSON for position)
            // We need to re-encode it using our custom protocol
            let encoded_data = message.content;

            println!("[i] Sending to TNC: {} bytes - {}", encoded_data.len(), encoded_data);

            // Construct AX.25 frame
            let frame = Ax25Frame {
                source: source_addr.clone(),
                destination: dest_addr.clone(),
                route: Vec::new(),
                command_or_response: Some(CommandResponse::Command),
                content: FrameContent::UnnumberedInformation(UnnumberedInformation {
                    pid: ProtocolIdentifier::None,
                    info: encoded_data.as_bytes().to_vec(),
                    poll_or_final: false,
                }),
            };

            // Send frame to TNC
            match tnc.lock().unwrap().send_frame(&frame) {
                Ok(_) => {
                    println!("[i] Frame sent successfully");
                }
                Err(e) => {
                    println!("[!] Failed to send frame to TNC: {}", e);
                }
            }
        }
        println!("[!] TNC sender thread ended");
    });
}

fn handle_incoming_tnc_messages(
    tnc_channels: Option<Res<TncChannels>>,
    mut event_writer: EventWriter<TncIncomingEvent>,
) {
    if let Some(tnc_channels) = tnc_channels {
        // Try to receive all available messages without blocking
        while let Ok(message) = tnc_channels.receiver.try_recv() {
            event_writer.write(TncIncomingEvent {
                message: message.content,
                message_type: message.message_type,
            });
        }
    }
}

fn handle_outgoing_tnc_messages(
    mut events: EventReader<TncOutgoingEvent>,
    tnc_channels: Option<Res<TncChannels>>,
) {
    if let Some(tnc_channels) = tnc_channels {
        for event in events.read() {
            let game_message = GameMessage {
                content: event.message.clone(),
                message_type: event.message_type.clone(),
            };

            if let Err(e) = tnc_channels.sender.send(game_message) {
                println!("[!] Failed to send message to TNC thread: {}", e);
            }
        }
    }
}
