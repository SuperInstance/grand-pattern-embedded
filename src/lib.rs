//! Grand Pattern for microcontrollers and embedded systems.
//!
//! A `no_std` compatible library implementing mono-vibe architecture with
//! fixed-size data structures, zero heap allocation, and minimal memory footprint.
//!
//! Designed for ESP32, Arduino, ARM Cortex-M, and anything with a few KB of RAM.

#![no_std]

pub mod jepa;
pub mod room;
pub mod graph;
pub mod gossip;
pub mod murmur;
pub mod fleet;
pub mod transport;

pub use jepa::Jepa;
pub use room::Room;
pub use graph::CellGraph;
pub use murmur::Murmur;

/// A single vibe value — one f64 per room.
pub type Vibe = f64;

/// Maximum number of rooms in a CellGraph.
pub const MAX_ROOMS: usize = 32;

/// Maximum number of edges in a CellGraph.
pub const MAX_EDGES: usize = 64;

/// Maximum number of readings stored in JEPA.
pub const JEPA_WINDOW: usize = 16;

/// Maximum murmur payload size in bytes.
pub const MURMUR_SIZE: usize = 32;
