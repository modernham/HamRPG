use bevy::audio::*;
use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, Res, ResMut};

pub fn play_background_audio(
  asset_server: Res<AssetServer>,
  mut commands: Commands,
  mut volume: ResMut<GlobalVolume>,
) {
  let audio = asset_server.load("background_audio.ogg");

  // Create an entity dedicated to playing our background music
  commands.spawn((
    AudioPlayer::new(audio),
    PlaybackSettings::LOOP,
  ));
  volume.volume = Volume::Linear(0.1);
}
