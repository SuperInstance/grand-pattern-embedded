//! Murmur: a small fixed-size gossip message.
//!
//! Contains a room ID, vibe value, tick, and optional surprise.
//! Fits in 32 bytes for efficient transmission over constrained transports.

use crate::{Vibe, MURMUR_SIZE};

/// A gossip message — small enough for UDP, BLE, UART, or I2C.
#[derive(Clone, Copy, Debug)]
pub struct Murmur {
    /// Room ID that originated this murmur.
    pub room_id: u8,
    /// Vibe value at time of murmur.
    pub vibe: Vibe,
    /// Tick when this murmur was created.
    pub tick: u32,
    /// Surprise value (prediction error).
    pub surprise: Vibe,
}

impl Murmur {
    /// Create a new murmur.
    pub fn new(room_id: u8, vibe: Vibe, tick: u32, surprise: Vibe) -> Self {
        Self { room_id, vibe, tick, surprise }
    }

    /// Serialize to a fixed-size byte array.
    ///
    /// Layout: [room_id:1][vibe:8][tick:4][surprise:8][padding:11]
    pub fn to_bytes(&self) -> [u8; MURMUR_SIZE] {
        let mut buf = [0u8; MURMUR_SIZE];
        buf[0] = self.room_id;
        buf[1..9].copy_from_slice(&self.vibe.to_le_bytes());
        buf[9..13].copy_from_slice(&self.tick.to_le_bytes());
        buf[13..21].copy_from_slice(&self.surprise.to_le_bytes());
        // bytes 21..32 are padding (zeros)
        buf
    }

    /// Deserialize from a byte array.
    pub fn from_bytes(data: &[u8; MURMUR_SIZE]) -> Option<Self> {
        if data.len() < MURMUR_SIZE {
            return None;
        }
        let room_id = data[0];
        let vibe = f64::from_le_bytes([
            data[1], data[2], data[3], data[4],
            data[5], data[6], data[7], data[8],
        ]);
        let tick = u32::from_le_bytes([data[9], data[10], data[11], data[12]]);
        let surprise = f64::from_le_bytes([
            data[13], data[14], data[15], data[16],
            data[17], data[18], data[19], data[20],
        ]);
        Some(Self { room_id, vibe, tick, surprise })
    }

    /// Check if this murmur is "interesting" (high surprise).
    pub fn is_interesting(&self, threshold: Vibe) -> bool {
        self.surprise.abs() > threshold
    }
}
