// HamRPG - Multiplayer 2D RPG over amateur radio
// Uses KISS protocol to communicate with TNC software for AX.25 packet transmission
mod components;
mod connection;
mod constants;
mod menu;
mod systems;

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_egui::EguiPlugin;
use connection::resources::PositionUpdateTime;
use connection::tnc_integration::{GameState, handle_tnc_events, send_position_updates};
use connection::tnc_plugin::TncPlugin;
use ini::Ini;
use iyes_perf_ui::prelude::*;
use menu::{AppState, MenuConfig, display_menu, validate_connection};
use systems::audio::play_background_audio;
use systems::animation::{animate_sprite, update_animation};
use systems::camera::update_camera;
use systems::gui::{ChatInputState, chat_window, display_player_callsigns};
use systems::player::{add_player, move_player};
use systems::remote_player::{cleanup_inactive_players, update_remote_player_movement};
use systems::setup::{send_welcome_message, setup, adjust_layer_z_ordering};

fn main() {
    // Load configuration from game_config.ini if available
    let menu_config = if let Ok(conf) = Ini::load_from_file("game_config.ini") {
        let game_info = conf.section(Some("Game")).unwrap();
        let callsign = game_info.get("callsign").unwrap_or("N0CALL-1").to_string();
        let pos_update_time = game_info.get("position_update_time").unwrap_or("30").to_string();

        let mut config = MenuConfig::new();
        config.callsign = callsign;
        config.position_update_time = pos_update_time;
        config
    } else {
        MenuConfig::new()
    };

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
            PerfUiPlugin,
            TiledMapPlugin::default(),
            TncPlugin,
            EguiPlugin {
                enable_multipass_for_primary_context: false,
            },
        ))
        .init_state::<AppState>()
        .insert_resource(menu_config)
        .insert_resource(ChatInputState::default())
        // Menu state systems
        .add_systems(Update, display_menu.run_if(in_state(AppState::Menu)))
        .add_systems(Update, validate_connection.run_if(in_state(AppState::Menu)))
        // One-time setup when entering game
        .add_systems(OnEnter(AppState::InGame), setup_game_state)
        .add_systems(OnEnter(AppState::InGame), (
            setup,
            add_player,
            send_welcome_message,
            play_background_audio,
        ).after(setup_game_state))
        // Per-frame game loop systems
        .add_systems(
            Update,
            (
                move_player,
                animate_sprite,
                update_animation,
                update_camera,
                handle_tnc_events,
                send_position_updates,
                update_remote_player_movement,
                cleanup_inactive_players,
                chat_window,
                display_player_callsigns,
                adjust_layer_z_ordering,
            ).run_if(in_state(AppState::InGame)),
        )
        .run();
}

// Initialize game state resource from menu configuration
fn setup_game_state(mut commands: Commands, menu_config: Res<MenuConfig>) {
    commands.insert_resource(GameState {
        chat_messages: Vec::new(),
        known_players: Vec::new(),
        player_entities: std::collections::HashMap::new(),
        player_callsign: menu_config.callsign.clone(),
    });

    // Update position update time from menu config
    commands.insert_resource(PositionUpdateTime(menu_config.get_position_update_time()));
}
