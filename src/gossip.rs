//! Gossip: generate and process murmurs for the CellGraph.
//!
//! Murmurs are small messages that propagate room state through the network.

use crate::{CellGraph, Murmur, Vibe, MAX_ROOMS};

/// Generate murmurs for all rooms in the graph.
///
/// Returns the number of murmurs written into the buffer.
pub fn generate_murmurs(graph: &CellGraph, buffer: &mut [Option<Murmur>; MAX_ROOMS]) -> usize {
    let mut count = 0;
    for i in 0..graph.room_count as usize {
        if let Some(ref room) = graph.rooms[i] {
            buffer[count] = Some(Murmur::new(
                room.id,
                room.vibe,
                graph.tick_count,
                room.last_surprise,
            ));
            count += 1;
        }
    }
    // Clear remaining
    for i in count..MAX_ROOMS {
        buffer[i] = None;
    }
    count
}

/// Process an incoming murmur: update the corresponding room's vibe.
///
/// Returns true if a room was updated.
pub fn process_murmur(graph: &mut CellGraph, murmur: &Murmur) -> bool {
    graph.set_vibe(murmur.room_id, murmur.vibe)
}

/// Filter murmurs: only keep those with surprise above threshold.
pub fn filter_interesting<'a>(
    murmurs: &'a [Option<Murmur>],
    threshold: Vibe,
    output: &mut [Option<Murmur>; MAX_ROOMS],
) -> usize {
    let mut count = 0;
    for m in murmurs {
        if let Some(ref murmur) = m {
            if murmur.is_interesting(threshold) {
                output[count] = Some(*murmur);
                count += 1;
            }
        }
    }
    for i in count..MAX_ROOMS {
        output[i] = None;
    }
    count
}
