use crate::systems::remote_player::PlayerPositionData;

// Custom packet protocol for amateur radio transmission
// Format: {TYPE|FIELD1|FIELD2|...
//
// Uses APRS "User-Defined" data type indicator '{' for TNC compatibility
//
// Position: {P|CALLSIGN|X|Y|DIR
// Example:  {P|N0CALL-1|128|256|S
//
// Chat:     {C|CALLSIGN|MESSAGE
// Example:  {C|N0CALL-1|Hello world
//
// This is a plain-text protocol (not encryption) that is:
// - Human readable and easily decoded
// - APRS-compliant

pub fn encode_position(callsign: &str, x: f32, y: f32, direction: &str) -> String {
    let dir_code = match direction {
        "north" => "N",
        "south" => "S",
        "east" => "E",
        "west" => "W",
        _ => "S", // default
    };
    format!("{{P|{}|{}|{}|{}", callsign, x.round() as i32, y.round() as i32, dir_code)
}

pub fn encode_chat(callsign: &str, message: &str) -> String {
    format!("{{C|{}|{}", callsign, message)
}

pub fn decode_packet(data: &str) -> Result<DecodedPacket, String> {
    // Remove APRS user-defined prefix if present
    let data = data.strip_prefix('{').unwrap_or(data);

    let parts: Vec<&str> = data.split('|').collect();

    if parts.is_empty() {
        return Err("Empty packet".to_string());
    }

    match parts[0] {
        "P" => {
            // Position packet: {P|CALLSIGN|X|Y|DIR
            if parts.len() < 5 {
                return Err("Invalid position packet".to_string());
            }
            let callsign = parts[1].to_string();
            let x = parts[2].parse::<f32>().map_err(|_| "Invalid X coordinate")?;
            let y = parts[3].parse::<f32>().map_err(|_| "Invalid Y coordinate")?;
            let direction = match parts[4] {
                "N" => "north",
                "S" => "south",
                "E" => "east",
                "W" => "west",
                _ => "south",
            }.to_string();

            Ok(DecodedPacket::Position(PlayerPositionData {
                callsign,
                x,
                y,
                direction,
            }))
        }
        "C" => {
            // Chat packet: {C|CALLSIGN|MESSAGE
            if parts.len() < 3 {
                return Err("Invalid chat packet".to_string());
            }
            let callsign = parts[1].to_string();
            // Rejoin remaining parts in case message contains '|'
            let message = parts[2..].join("|");
            Ok(DecodedPacket::Chat(format!("{}: {}", callsign, message)))
        }
        _ => Err(format!("Unknown packet type: {}", parts[0])),
    }
}

pub enum DecodedPacket {
    Position(PlayerPositionData),
    Chat(String),
}
