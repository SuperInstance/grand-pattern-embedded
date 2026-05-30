//! Fleet-level operations: aggregate stats across all rooms.

use crate::{CellGraph, Vibe};

/// Compute the average vibe across all rooms.
pub fn fleet_vibe(graph: &CellGraph) -> Vibe {
    if graph.room_count == 0 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 0..graph.room_count as usize {
        if let Some(ref room) = graph.rooms[i] {
            total += room.vibe;
        }
    }
    total / graph.room_count as f64
}

/// Compute the average surprise across all rooms.
pub fn fleet_surprise(graph: &CellGraph) -> Vibe {
    if graph.room_count == 0 {
        return 0.0;
    }
    let mut total = 0.0;
    for i in 0..graph.room_count as usize {
        if let Some(ref room) = graph.rooms[i] {
            total += room.last_surprise;
        }
    }
    total / graph.room_count as f64
}

/// Compute the max surprise across all rooms.
pub fn max_surprise(graph: &CellGraph) -> Vibe {
    let mut max_val = 0.0;
    for i in 0..graph.room_count as usize {
        if let Some(ref room) = graph.rooms[i] {
            let abs_s = room.last_surprise.abs();
            if abs_s > max_val {
                max_val = abs_s;
            }
        }
    }
    max_val
}
