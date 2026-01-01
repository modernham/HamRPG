use crate::components::RemotePlayer;
use crate::connection::compression::encode_chat;
use crate::connection::message::MessageType;
use crate::connection::tnc_plugin::TncOutgoingEvent;
use crate::connection::tnc_integration::GameState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

// Resource to track chat input state
//This helps to see if the player is chatting or not.
#[derive(Resource, Default)]
pub struct ChatInputState {
    pub active: bool,
    pub input: String,
}

// Chat window system
pub fn chat_window(
    mut contexts: EguiContexts,
    mut chat_state: ResMut<ChatInputState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<TncOutgoingEvent>,
    mut game_state: ResMut<GameState>,
) {
    // Toggle chat input with T key
    if keyboard.just_pressed(KeyCode::KeyT) {
        chat_state.active = true;
    }

    // Get the egui context
    let ctx = contexts.ctx_mut();

    // Display the chat history window
    egui::Window::new("Chat")
        .frame(egui::Frame {
            fill: egui::Color32::from_rgba_unmultiplied(20, 20, 20, 220), // Semi-transparent dark background
            stroke: egui::Stroke::new(
                1.0,
                egui::Color32::from_rgba_unmultiplied(100, 100, 100, 200),
            ),
            ..Default::default()
        })
        .resizable(true)
        .collapsible(true)
        .default_width(400.0)
        .default_height(400.0)
        .min_width(400.0)
        .default_open(true)
        .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, -25.0))
        .show(ctx, |ui| {
            // Chat history area
            ui.add_space(100.0);
            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 25.0) // Adjust dynamically to window size
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    // Display the recent messages
                    for message in game_state.chat_messages.iter().rev().take(20).rev() {
                        ui.label(
                            egui::RichText::new(message)
                                .size(18.0) // Larger font size for messages
                                .color(egui::Color32::from_rgba_unmultiplied(230, 230, 230, 240)),
                        );
                        ui.separator();
                    }
                });

            // Chat input area - only shown when active
            if chat_state.active {
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    // Text input field with focus
                    let text_edit = egui::TextEdit::singleline(&mut chat_state.input)
                        .hint_text("Type message and press Enter...")
                        .desired_width(ui.available_width() - 80.0)
                        .font(egui::TextStyle::Heading) // Use heading style for larger text
                        .text_color(egui::Color32::WHITE);

                    let response = ui.add(text_edit);

                    // Focus the text input
                    if response.gained_focus() {
                        // Already focused
                    } else if !response.has_focus() {
                        response.request_focus();
                    }

                    // Check for Enter key or button press
                    let send_pressed = ui
                        .button(egui::RichText::new("Send").size(16.0).strong())
                        .clicked();
                    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));

                    if (enter_pressed || send_pressed) && !chat_state.input.trim().is_empty() {
                        // Send the message using custom compact protocol
                        let encoded_chat = encode_chat(
                            &game_state.player_callsign,
                            &chat_state.input
                        );

                        // Add to local chat history
                        let player_message =
                            format!("{}: {}", game_state.player_callsign, chat_state.input);
                        game_state.chat_messages.push(player_message);

                        // Send over TNC
                        event_writer.write(TncOutgoingEvent {
                            message: encoded_chat,
                            message_type: MessageType::Chat,
                        });

                        // Clear the input and disable chat mode
                        chat_state.input.clear();
                        chat_state.active = false;
                    }

                    // Close if Escape is pressed
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        chat_state.active = false;
                    }
                });
            } else {
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("Press T to chat")
                        .size(16.0)
                        .italics()
                        .color(egui::Color32::from_rgba_unmultiplied(200, 200, 200, 200)),
                );
            }
        });
}

//This is updated every frame to display each player callsign above their head.
pub fn display_player_callsigns(
    mut contexts: EguiContexts,
    cameras: Query<(&Camera, &GlobalTransform)>,
    remote_players: Query<(&Transform, &RemotePlayer)>,
    local_player: Query<&Transform, (With<crate::components::Entity>, Without<RemotePlayer>)>,
    game_state: Res<GameState>,
) {
    // Get the camera for screen position calculations
    let (camera, camera_transform) = match cameras.single() {
        Ok(result) => result,
        Err(_) => return, // No camera available
    };

    let ctx = contexts.ctx_mut();

    // Process egui rendering on top of the game
    egui::Area::new(egui::Id::new("player_labels"))
        .movable(false)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            // Display callsigns for remote players
            for (transform, remote_player) in remote_players.iter() {
                let world_pos = transform.translation;

                // Add offset to position text above player's head
                let label_world_pos = Vec3::new(world_pos.x, world_pos.y + 20.0, world_pos.z);

                // Convert world position to screen position
                if let Some(screen_pos) = world_to_screen(camera, camera_transform, label_world_pos)
                {
                    ui.painter().text(
                        egui::pos2(screen_pos.x, screen_pos.y),
                        egui::Align2::CENTER_CENTER,
                        &remote_player.callsign,
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );
                }
            }

            // Display callsign for local player
            if let Ok(player_transform) = local_player.single() {
                let world_pos = player_transform.translation;
                let label_world_pos = Vec3::new(world_pos.x, world_pos.y + 20.0, world_pos.z);

                if let Some(screen_pos) = world_to_screen(camera, camera_transform, label_world_pos)
                {
                    ui.painter().text(
                        egui::pos2(screen_pos.x, screen_pos.y),
                        egui::Align2::CENTER_CENTER,
                        &game_state.player_callsign,
                        egui::FontId::proportional(14.0),
                        // Highlight local player with different color
                        egui::Color32::WHITE,
                    );
                }
            }
        });
}

// Helper function to convert world position to screen position
fn world_to_screen(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    world_pos: Vec3,
) -> Option<Vec2> {
    camera
        .world_to_viewport(camera_transform, world_pos)
        .ok()
        .map(|v| Vec2::new(v.x, v.y))
}
