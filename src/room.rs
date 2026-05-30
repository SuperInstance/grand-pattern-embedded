//! Room: a single cell in the CellGraph.
//!
//! Each room has one mono-vibe (f64), a JEPA instance for prediction,
//! and tracks its last surprise value.

use crate::{Vibe, Jepa};

/// A room in the grand pattern — one vibe, one JEPA, bounded memory.
pub struct Room {
    /// Room identifier (0–255).
    pub id: u8,
    /// The current mono-vibe value.
    pub vibe: Vibe,
    /// JEPA predictor for this room.
    pub jepa: Jepa,
    /// Last prediction error (surprise).
    pub last_surprise: Vibe,
}

impl Room {
    /// Create a new room with the given ID and initial vibe.
    pub fn new(id: u8, vibe: Vibe) -> Self {
        Self {
            id,
            vibe,
            jepa: Jepa::new(),
            last_surprise: 0.0,
        }
    }

    /// Create a room with a custom JEPA window size.
    pub fn with_window(id: u8, vibe: Vibe, window: usize) -> Self {
        Self {
            id,
            vibe,
            jepa: Jepa::with_window(window),
            last_surprise: 0.0,
        }
    }

    /// Update the room's vibe and let JEPA learn from the new value.
    ///
    /// Returns the surprise (prediction error).
    pub fn tick(&mut self, global_tick: u32) -> Vibe {
        let surprise = self.jepa.learn(global_tick, self.vibe);
        self.last_surprise = surprise;
        surprise
    }

    /// Set the vibe directly.
    pub fn set_vibe(&mut self, vibe: Vibe) {
        self.vibe = vibe;
    }

    /// Get JEPA's prediction for the next vibe.
    pub fn predicted_vibe(&self) -> Vibe {
        self.jepa.predict()
    }
}
