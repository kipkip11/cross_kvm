use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Packet {
    Handshake {
        client_name: String,
        os: String,
        nonce: [u8; 12],
    },
    HandshakeAck {
        server_name: String,
        success: bool,
    },
    Heartbeat,
    InputEvent(InputPayload),
    ClipboardData(ClipboardPayload),
    FileInit {
        file_name: String,
        file_size: u64,
        sha256_hash: String,
    },
    FileChunk {
        chunk_index: u64,
        data: Vec<u8>,
        is_last: bool,
    },
    FileAck {
        file_name: String,
        success: bool,
    },
    SwitchScreen {
        to_remote: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InputPayload {
    MouseMove { x: f64, y: f64 },
    MouseDown { button: u8 },
    MouseUp { button: u8 },
    MouseWheel { delta_x: i32, delta_y: i32 },
    KeyDown { key_code: u32 },
    KeyUp { key_code: u32 },
    ResetModifiers, // Chống kẹt phím (Failsafe Modifier Release)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClipboardPayload {
    Text(String),
    Image {
        width: usize,
        height: usize,
        bytes: Vec<u8>,
    },
}
