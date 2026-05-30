//! Joint-Embedding Predictive Architecture (JEPA) for embedded systems.
//!
//! Fixed-size, no-heap implementation with configurable window.
//! Maintains a sliding window of readings and learns prediction weights.

use crate::{JEPA_WINDOW, Vibe};

/// A minimal JEPA implementation with fixed-size arrays.
///
/// Stores up to `JEPA_WINDOW` readings, each tagged with a tick timestamp.
/// Predictions are weighted averages; learning adjusts weights by prediction error.
pub struct Jepa {
    /// Stored readings: (tick, value) pairs. `None` means empty slot.
    pub readings: [Option<(u32, Vibe)>; JEPA_WINDOW],
    /// Prediction weights, one per reading slot.
    pub weights: [Vibe; JEPA_WINDOW],
    /// Current window size (how many slots are actually used).
    pub window: usize,
    /// Number of readings currently stored.
    pub count: usize,
}

impl Jepa {
    /// Create a new JEPA with default weights (uniform).
    pub fn new() -> Self {
        Self {
            readings: [None; JEPA_WINDOW],
            weights: [1.0; JEPA_WINDOW],
            window: JEPA_WINDOW,
            count: 0,
        }
    }

    /// Create a JEPA with a custom window size (≤ JEPA_WINDOW).
    pub fn with_window(window: usize) -> Self {
        let w = if window == 0 { 1 } else if window > JEPA_WINDOW { JEPA_WINDOW } else { window };
        Self {
            readings: [None; JEPA_WINDOW],
            weights: [1.0; JEPA_WINDOW],
            window: w,
            count: 0,
        }
    }

    /// Predict the next value as a weighted average of stored readings.
    ///
    /// Returns 0.0 if no readings are stored.
    pub fn predict(&self) -> Vibe {
        let mut sum = 0.0_f64;
        let mut weight_sum = 0.0_f64;
        for i in 0..self.count {
            if let Some((_, val)) = self.readings[i] {
                let w = self.weights[i];
                sum += val * w;
                weight_sum += w;
            }
        }
        if weight_sum == 0.0 { 0.0 } else { sum / weight_sum }
    }

    /// Learn from a new reading: store it, trim old readings, and adjust weights.
    ///
    /// Returns the prediction error (surprise) for this reading.
    pub fn learn(&mut self, tick: u32, value: Vibe) -> Vibe {
        let prediction = self.predict();
        let error = value - prediction;

        // Shift readings if window is full
        if self.count >= self.window {
            // Shift left by 1
            for i in 0..self.window.saturating_sub(1) {
                self.readings[i] = self.readings[i + 1];
                self.weights[i] = self.weights[i + 1];
            }
            self.count = self.window.saturating_sub(1);
        }

        // Insert new reading
        if self.count < JEPA_WINDOW {
            // Adjust weights: reduce weight of readings that were wrong
            let abs_error = if error < 0.0 { -error } else { error };
            for i in 0..self.count {
                if let Some((_, prev_val)) = self.readings[i] {
                    let diff = (prev_val - prediction).abs();
                    // If this reading was close to the prediction, it contributed to the error
                    if diff < abs_error * 0.5 {
                        self.weights[i] *= 0.95; // slight decay
                    } else {
                        self.weights[i] *= 1.01; // slight boost, capped
                        if self.weights[i] > 10.0 {
                            self.weights[i] = 10.0;
                        }
                    }
                }
            }

            self.readings[self.count] = Some((tick, value));
            self.weights[self.count] = 1.0;
            self.count += 1;
        }

        error
    }

    /// Trim readings to fit within the current window.
    pub fn trim(&mut self) {
        if self.count > self.window {
            let excess = self.count - self.window;
            for i in 0..self.window {
                self.readings[i] = self.readings[i + excess];
                self.weights[i] = self.weights[i + excess];
            }
            for i in self.window..self.count {
                self.readings[i] = None;
                self.weights[i] = 1.0;
            }
            self.count = self.window;
        }
    }

    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.readings = [None; JEPA_WINDOW];
        self.weights = [1.0; JEPA_WINDOW];
        self.count = 0;
    }

    /// Get the last stored reading value, if any.
    pub fn last_value(&self) -> Option<Vibe> {
        if self.count > 0 {
            self.readings[self.count - 1].map(|(_, v)| v)
        } else {
            None
        }
    }
}

impl Default for Jepa {
    fn default() -> Self {
        Self::new()
    }
}
