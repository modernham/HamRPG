# HamRPG

Multiplayer P2P RPG demo for amateur radio using KISS protocol. Built with Rust and the Bevy game engine.

![Alt text](/screenshots/scr2.png?raw=true "Game screenshot")

## Features

- P2P multiplayer over amateur radio (HF/VHF)
- KISS protocol via software network TNC (direwolf, sound modem, and VARA KISS port)
- Compressed packet format optimized for low data rates
- Player position updates with configurable intervals
- Position interpolation for smooth movement despite high latency
- In-game chat system

## Running the Game

### Windows

1. Download the windows-hamrpg.zip file from the releases page
2. Extract the zip file and open the extracted folder
3. Execute "hamrpg.exe"

### Linux

1. Download the linux-hamrpg.zip file from the releases page
2. Extract the zip file and open the extracted folder
3. Open a terminal in the extracted folder
4. Install Bevy Game Engine Requirements:
```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
# If using Wayland
sudo apt-get install libwayland-dev libxkbcommon-dev
```
5. Mark the game as executable:
```bash
sudo chmod +x hamrpg
```
6. Run the game:
```bash
./hamrpg
```

### Configuration

Configure TNC connection settings in the title screen. Set your position update interval based on propagation conditions and desired data rate.

## Building from Source

1. Install the Rust toolchain
2. Install Bevy Game Engine Requirements (Linux only):
```bash
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
# If using Wayland
sudo apt-get install libwayland-dev libxkbcommon-dev
```
3. Build the game:
```bash
cargo build --release
```
4. Move the assets folder next to the compiled executable for deployment.
5. Run the game:
```bash
cargo run --release
```

## Protocol

Uses AX.25 KISS protocol with compact text encoding to minimize packet size for HF operation where data rates are typically 300 baud.

![Alt text](/screenshots/scr1.png?raw=true "Game screenshot")

## Credits

The following asset pack was used for the sprites:
https://game-endeavor.itch.io/mystic-woods