//! Transport abstractions for gossip on embedded platforms.
//!
//! Provides traits for different transport mechanisms:
//! - Wi-Fi (UDP multicast) — ESP32
//! - BLE — ESP32
//! - UART/Serial — Arduino
//! - I2C — Arduino
//!
//! Actual implementations require platform-specific code (esp-idf, arduino-hal).
//! This module defines the interface only.

use crate::murmur::Murmur;

/// A transport that can send and receive murmurs.
pub trait GossipTransport {
    /// Send a murmur. Returns true on success.
    fn send(&mut self, murmur: &Murmur) -> bool;

    /// Try to receive a murmur (non-blocking).
    /// Returns None if no murmur is available.
    fn receive(&mut self) -> Option<Murmur>;

    /// Check if the transport is ready.
    fn is_ready(&self) -> bool;
}

/// Placeholder for Wi-Fi UDP multicast transport (ESP32).
///
/// Actual implementation requires `esp-idf` or `esp-wifi` crate.
/// This struct compiles on any platform for testing.
pub struct WifiGossip {
    /// Local port for multicast.
    pub port: u16,
    /// Whether transport is initialized.
    pub ready: bool,
}

impl WifiGossip {
    pub fn new(port: u16) -> Self {
        Self { port, ready: false }
    }

    /// Initialize Wi-Fi transport (stub — real impl needs esp-idf).
    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}

/// Placeholder for BLE gossip transport (ESP32).
pub struct BleGossip {
    /// BLE advertising interval in ms.
    pub interval_ms: u16,
    /// Whether transport is initialized.
    pub ready: bool,
}

impl BleGossip {
    pub fn new(interval_ms: u16) -> Self {
        Self { interval_ms, ready: false }
    }

    /// Initialize BLE transport (stub — real impl needs esp-idf).
    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}

/// Placeholder for UART/Serial gossip transport (Arduino).
pub struct SerialGossip {
    /// Baud rate.
    pub baud: u32,
    /// Whether transport is initialized.
    pub ready: bool,
}

impl SerialGossip {
    pub fn new(baud: u32) -> Self {
        Self { baud, ready: false }
    }

    /// Initialize serial transport (stub — real impl needs arduino-hal).
    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}

/// Placeholder for I2C mesh gossip transport (Arduino).
pub struct I2cGossip {
    /// I2C address of this device.
    pub address: u8,
    /// Whether transport is initialized.
    pub ready: bool,
}

impl I2cGossip {
    pub fn new(address: u8) -> Self {
        Self { address, ready: false }
    }

    /// Initialize I2C transport (stub — real impl needs arduino-hal).
    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}

/// Storage trait for persisting JEPA weights across reboots.
pub trait WeightStorage {
    /// Save weights for a room. Returns true on success.
    fn save(&mut self, room_id: u8, weights: &[Option<f64>]) -> bool;

    /// Load weights for a room. Returns number of weights loaded.
    fn load(&mut self, room_id: u8, weights: &mut [Option<f64>]) -> usize;
}

/// Placeholder for flash storage (ESP32).
pub struct FlashStorage {
    pub ready: bool,
}

impl FlashStorage {
    pub fn new() -> Self {
        Self { ready: false }
    }

    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}

/// Placeholder for EEPROM storage (Arduino).
pub struct EepromStorage {
    pub ready: bool,
}

impl EepromStorage {
    pub fn new() -> Self {
        Self { ready: false }
    }

    pub fn init(&mut self) -> bool {
        self.ready = true;
        true
    }
}
