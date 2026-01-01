use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// Game states to differentiate between menu and gameplay
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

/// Resource to store menu input values
#[derive(Resource)]
pub struct MenuConfig {
    pub callsign: String,
    pub tnc_host: String,
    pub tnc_port: String,
    pub position_update_time: String,
    pub connect_clicked: bool,
    pub connection_error: Option<String>,
    pub is_connecting: bool,
    pub validation_time: Option<std::time::Instant>,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuConfig {
    pub fn new() -> Self {
        Self {
            callsign: "N0CALL-1".to_string(),
            tnc_host: "127.0.0.1".to_string(),
            tnc_port: "8100".to_string(),
            position_update_time: "30".to_string(),
            connect_clicked: false,
            connection_error: None,
            is_connecting: false,
            validation_time: None,
        }
    }

    pub fn get_tnc_address(&self) -> String {
        format!("tnc:tcpkiss:{}:{}", self.tnc_host, self.tnc_port)
    }

    pub fn get_position_update_time(&self) -> u64 {
        self.position_update_time.parse().unwrap_or(30)
    }
}

/// System to display the startup menu
pub fn display_menu(
    mut contexts: EguiContexts,
    mut menu_config: ResMut<MenuConfig>,
) {
    let ctx = contexts.ctx_mut();

    // Set dark retro theme
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = egui::Color32::from_rgb(10, 20, 10);
    style.visuals.panel_fill = egui::Color32::from_rgb(5, 15, 5);
    ctx.set_style(style);

    egui::CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(5, 10, 5),
            ..Default::default()
        })
        .show(ctx, |ui| {
            // Get screen dimensions
            let screen_rect = ui.available_rect_before_wrap();
            let center_x = screen_rect.center().x;

            ui.vertical(|ui| {
                ui.add_space(screen_rect.height() * 0.15);

                // Title - centered
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        egui::pos2(center_x, ui.cursor().top()),
                        egui::vec2(600.0, 60.0),
                    ),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("HAM RADIO RPG")
                                    .size(48.0)
                                    .color(egui::Color32::from_rgb(100, 200, 100))
                                    .strong()
                                    .family(egui::FontFamily::Monospace),
                            );
                        });
                    },
                );

                ui.add_space(10.0);

                // Subtitle
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        egui::pos2(center_x, ui.cursor().top()),
                        egui::vec2(600.0, 30.0),
                    ),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("MULTIPLAYER RPG OVER AMATEUR RADIO")
                                    .size(14.0)
                                    .color(egui::Color32::from_rgb(120, 180, 120))
                                    .family(egui::FontFamily::Monospace),
                            );
                        });
                    },
                );

                ui.add_space(40.0);

                // Configuration panel - centered
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        egui::pos2(center_x, ui.cursor().top() + 150.0),
                        egui::vec2(500.0, 300.0),
                    ),
                    |ui| {
                        egui::Frame::group(ui.style())
                            .fill(egui::Color32::from_rgb(20, 30, 20))
                            .stroke(egui::Stroke::new(
                                2.0,
                                egui::Color32::from_rgb(100, 200, 100),
                            ))
                            .rounding(0.0)
                            .inner_margin(25.0)
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("[ CONNECTION SETUP ]")
                                            .size(18.0)
                                            .color(egui::Color32::from_rgb(100, 200, 100))
                                            .strong()
                                            .family(egui::FontFamily::Monospace),
                                    );
                                    ui.add_space(20.0);

                                    // Callsign input
                                    ui.horizontal(|ui| {
                                        ui.add_sized(
                                            [180.0, 20.0],
                                            egui::Label::new(
                                                egui::RichText::new("Callsign:")
                                                    .size(14.0)
                                                    .color(egui::Color32::from_rgb(120, 200, 120))
                                                    .family(egui::FontFamily::Monospace),
                                            ),
                                        );
                                        ui.add_sized(
                                            [250.0, 25.0],
                                            egui::TextEdit::singleline(&mut menu_config.callsign)
                                                .hint_text("N0CALL-1")
                                                .font(egui::TextStyle::Monospace),
                                        );
                                    });
                                    ui.add_space(12.0);

                                    // TNC Host input
                                    ui.horizontal(|ui| {
                                        ui.add_sized(
                                            [180.0, 20.0],
                                            egui::Label::new(
                                                egui::RichText::new("TNC Host:")
                                                    .size(14.0)
                                                    .color(egui::Color32::from_rgb(120, 200, 120))
                                                    .family(egui::FontFamily::Monospace),
                                            ),
                                        );
                                        ui.add_sized(
                                            [250.0, 25.0],
                                            egui::TextEdit::singleline(&mut menu_config.tnc_host)
                                                .hint_text("127.0.0.1")
                                                .font(egui::TextStyle::Monospace),
                                        );
                                    });
                                    ui.add_space(12.0);

                                    // TNC Port input
                                    ui.horizontal(|ui| {
                                        ui.add_sized(
                                            [180.0, 20.0],
                                            egui::Label::new(
                                                egui::RichText::new("TNC Port:")
                                                    .size(14.0)
                                                    .color(egui::Color32::from_rgb(120, 200, 120))
                                                    .family(egui::FontFamily::Monospace),
                                            ),
                                        );
                                        ui.add_sized(
                                            [250.0, 25.0],
                                            egui::TextEdit::singleline(&mut menu_config.tnc_port)
                                                .hint_text("8100")
                                                .font(egui::TextStyle::Monospace),
                                        );
                                    });
                                    ui.add_space(12.0);

                                    // Update interval input
                                    ui.horizontal(|ui| {
                                        ui.add_sized(
                                            [180.0, 20.0],
                                            egui::Label::new(
                                                egui::RichText::new("Update Interval (sec):")
                                                    .size(14.0)
                                                    .color(egui::Color32::from_rgb(120, 200, 120))
                                                    .family(egui::FontFamily::Monospace),
                                            ),
                                        );
                                        ui.add_sized(
                                            [250.0, 25.0],
                                            egui::TextEdit::singleline(&mut menu_config.position_update_time)
                                                .hint_text("30")
                                                .font(egui::TextStyle::Monospace),
                                        );
                                    });
                                    ui.add_space(12.0);

                                    ui.add_space(20.0);

                                    // Error message display
                                    if let Some(error) = &menu_config.connection_error {
                                        ui.label(
                                            egui::RichText::new(format!("ERROR: {}", error))
                                                .size(14.0)
                                                .color(egui::Color32::from_rgb(255, 100, 100))
                                                .family(egui::FontFamily::Monospace),
                                        );
                                        ui.add_space(10.0);
                                    }

                                    // Connect button
                                    let button_text = if menu_config.validation_time.is_some() {
                                        "[ VALIDATED - STARTING... ]"
                                    } else if menu_config.is_connecting {
                                        "[ CONNECTING... ]"
                                    } else {
                                        "[ CONNECT ]"
                                    };

                                    let button_enabled = !menu_config.is_connecting && menu_config.validation_time.is_none();

                                    if ui
                                        .add_enabled(
                                            button_enabled,
                                            egui::Button::new(
                                                egui::RichText::new(button_text)
                                                    .size(18.0)
                                                    .strong()
                                                    .family(egui::FontFamily::Monospace),
                                            )
                                            .fill(egui::Color32::from_rgb(40, 80, 40))
                                            .stroke(egui::Stroke::new(
                                                2.0,
                                                egui::Color32::from_rgb(100, 200, 100),
                                            ))
                                            .rounding(0.0)
                                            .min_size(egui::vec2(250.0, 45.0)),
                                        )
                                        .clicked()
                                    {
                                        // Validate inputs
                                        let valid = !menu_config.callsign.is_empty()
                                            && !menu_config.tnc_host.is_empty()
                                            && !menu_config.tnc_port.is_empty()
                                            && !menu_config.position_update_time.is_empty();

                                        if valid {
                                            println!("[i] Connecting with callsign: {}", menu_config.callsign);
                                            println!(
                                                "[i] TNC: TCP KISS {}:{}",
                                                menu_config.tnc_host, menu_config.tnc_port
                                            );
                                            println!(
                                                "[i] Position update interval: {} seconds",
                                                menu_config.position_update_time
                                            );
                                            menu_config.connection_error = None;
                                            menu_config.is_connecting = true;
                                            menu_config.connect_clicked = true;
                                        } else {
                                            menu_config.connection_error = Some("Please fill in all fields".to_string());
                                        }
                                    }
                                });
                            });
                    },
                );

                // Footer text
                ui.allocate_ui_at_rect(
                    egui::Rect::from_center_size(
                        egui::pos2(center_x, screen_rect.bottom() - 60.0),
                        egui::vec2(600.0, 40.0),
                    ),
                    |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                egui::RichText::new("! REQUIRES TNC SOFTWARE RUNNING !")
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(200, 200, 100))
                                    .family(egui::FontFamily::Monospace),
                            );
                            ui.label(
                                egui::RichText::new("Direwolf / SoundModem / VARA")
                                    .size(10.0)
                                    .color(egui::Color32::from_rgb(120, 150, 120))
                                    .family(egui::FontFamily::Monospace),
                            );
                        });
                    },
                );
            });
        });
}

/// System to validate TNC connection before entering game
pub fn validate_connection(
    mut menu_config: ResMut<MenuConfig>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    use ax25_tnc::tnc::{Tnc, TncAddress};
    use std::time::{Duration, Instant};

    // Check if we're waiting after successful validation
    if let Some(validation_time) = menu_config.validation_time {
        // Wait 3 seconds before transitioning to give modem time to disconnect
        if validation_time.elapsed() >= Duration::from_secs(3) {
            println!("[i] Connecting to game...");
            menu_config.validation_time = None;
            menu_config.is_connecting = false;
            menu_config.connect_clicked = false;
            next_state.set(AppState::InGame);
        }
        return;
    }

    // If not validating, return early
    if !menu_config.connect_clicked || !menu_config.is_connecting {
        return;
    }

    // Try to connect to the TNC
    let tnc_address_str = menu_config.get_tnc_address();

    match tnc_address_str.parse::<TncAddress>() {
        Ok(addr) => {
            match Tnc::open(&addr) {
                Ok(_tnc) => {
                    // Connection successful! Set validation time and wait
                    println!("[i] TNC connection validated successfully");
                    println!("[i] Waiting for modem to reset...");
                    menu_config.validation_time = Some(Instant::now());
                    // Don't transition yet - wait for the timer
                }
                Err(e) => {
                    // Connection failed
                    let error_msg = format!("Connection failed: {}", e);
                    println!("[!] {}", error_msg);
                    menu_config.connection_error = Some(error_msg);
                    menu_config.is_connecting = false;
                    menu_config.connect_clicked = false;
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Invalid TNC address: {}", e);
            println!("[!] {}", error_msg);
            menu_config.connection_error = Some(error_msg);
            menu_config.is_connecting = false;
            menu_config.connect_clicked = false;
        }
    }
}
