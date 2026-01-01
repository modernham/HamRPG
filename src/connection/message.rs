use serde::{Deserialize, Serialize};

// Message type identifier for game events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Chat,
    Position,
}

// Game message structure for internal event passing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMessage {
    pub content: String,
    pub message_type: MessageType,
}
