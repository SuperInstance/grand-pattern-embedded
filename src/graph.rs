//! CellGraph: the core data structure for the Grand Pattern on embedded.
//!
//! Fixed-size arrays for rooms and edges. No heap allocation.

use crate::{Vibe, Room, MAX_ROOMS, MAX_EDGES};

/// A graph of rooms connected by weighted edges.
///
/// Vibes diffuse along edges, conservation is maintained.
/// All storage is fixed-size arrays — no Vec, no heap.
pub struct CellGraph {
    /// Rooms stored in fixed array. `None` means empty slot.
    pub rooms: [Option<Room>; MAX_ROOMS],
    /// Edges: (from_id, to_id, weight).
    pub edges: [(u8, u8, Vibe); MAX_EDGES],
    /// Number of occupied room slots.
    pub room_count: u8,
    /// Number of occupied edge slots.
    pub edge_count: u8,
    /// Global tick counter.
    pub tick_count: u32,
}

impl CellGraph {
    /// Create a new empty CellGraph.
    pub fn new() -> Self {
        Self {
            rooms: [const { None }; MAX_ROOMS],
            edges: [(0, 0, 0.0); MAX_EDGES],
            room_count: 0,
            edge_count: 0,
            tick_count: 0,
        }
    }

    /// Find the index of a room by ID.
    pub fn find_room(&self, id: u8) -> Option<usize> {
        for i in 0..self.room_count as usize {
            if let Some(ref room) = self.rooms[i] {
                if room.id == id {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Add a room. Returns false if at capacity or duplicate ID.
    pub fn add_room(&mut self, id: u8, initial_vibe: Vibe) -> bool {
        if self.room_count as usize >= MAX_ROOMS {
            return false;
        }
        if self.find_room(id).is_some() {
            return false; // duplicate
        }
        let idx = self.room_count as usize;
        self.rooms[idx] = Some(Room::new(id, initial_vibe));
        self.room_count += 1;
        true
    }

    /// Add an edge between two rooms with a weight.
    ///
    /// Returns false if at capacity, rooms don't exist, or self-edge.
    pub fn add_edge(&mut self, from_id: u8, to_id: u8, weight: Vibe) -> bool {
        if self.edge_count as usize >= MAX_EDGES {
            return false;
        }
        if from_id == to_id {
            return false;
        }
        if self.find_room(from_id).is_none() || self.find_room(to_id).is_none() {
            return false;
        }
        let idx = self.edge_count as usize;
        self.edges[idx] = (from_id, to_id, weight);
        self.edge_count += 1;
        true
    }

    /// Get a room's vibe by ID.
    pub fn get_vibe(&self, id: u8) -> Option<Vibe> {
        self.find_room(id).and_then(|i| self.rooms[i].as_ref().map(|r| r.vibe))
    }

    /// Set a room's vibe by ID.
    pub fn set_vibe(&mut self, id: u8, vibe: Vibe) -> bool {
        if let Some(i) = self.find_room(id) {
            if let Some(ref mut room) = self.rooms[i] {
                room.vibe = vibe;
                return true;
            }
        }
        false
    }

    /// Diffuse vibes along edges. One step of the diffusion process.
    ///
    /// For each edge (A → B, weight w), a fraction of the vibe difference
    /// flows from A to B. Total vibe is conserved.
    pub fn diffuse(&mut self, dt: Vibe) {
        // Compute flows first (don't modify in-place during iteration)
        let mut flows: [Vibe; MAX_ROOMS] = [0.0; MAX_ROOMS];

        for i in 0..self.edge_count as usize {
            let (from_id, to_id, weight) = self.edges[i];
            let from_vibe = self.get_vibe(from_id).unwrap_or(0.0);
            let to_vibe = self.get_vibe(to_id).unwrap_or(0.0);
            let flow = weight * dt * (from_vibe - to_vibe);
            let from_idx = self.find_room(from_id);
            let to_idx = self.find_room(to_id);
            if let Some(fi) = from_idx {
                flows[fi] -= flow;
            }
            if let Some(ti) = to_idx {
                flows[ti] += flow;
            }
        }

        // Apply flows
        for i in 0..self.room_count as usize {
            if let Some(ref mut room) = self.rooms[i] {
                room.vibe += flows[i];
            }
        }
    }

    /// Run one tick: let each room's JEPA learn, then diffuse.
    ///
    /// Returns the average surprise across all rooms.
    pub fn tick(&mut self) -> Vibe {
        let mut total_surprise = 0.0_f64;
        let mut count = 0u8;

        for i in 0..self.room_count as usize {
            if let Some(ref mut room) = self.rooms[i] {
                let surprise = room.tick(self.tick_count);
                total_surprise += surprise;
                count += 1;
            }
        }

        // Diffuse vibes
        self.diffuse(0.1);

        self.tick_count += 1;

        if count > 0 {
            total_surprise / count as f64
        } else {
            0.0
        }
    }

    /// Compute total vibe across all rooms (should be conserved).
    pub fn total_vibe(&self) -> Vibe {
        let mut total = 0.0;
        for i in 0..self.room_count as usize {
            if let Some(ref room) = self.rooms[i] {
                total += room.vibe;
            }
        }
        total
    }

    /// Check if the graph is empty.
    pub fn is_empty(&self) -> bool {
        self.room_count == 0
    }
}

impl Default for CellGraph {
    fn default() -> Self {
        Self::new()
    }
}
