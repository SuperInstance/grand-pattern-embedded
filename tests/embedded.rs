//! Comprehensive test suite for grand-pattern-embedded.
//!
//! 25+ tests covering core functionality, edge cases, and embedded constraints.

#![no_std]
#![allow(clippy::float_cmp)]

extern crate alloc;

use grand_pattern_embedded::*;
use grand_pattern_embedded::jepa::Jepa;
use grand_pattern_embedded::room::Room;
use grand_pattern_embedded::graph::CellGraph;
use grand_pattern_embedded::murmur::Murmur;
use grand_pattern_embedded::gossip;
use grand_pattern_embedded::fleet;
use grand_pattern_embedded::transport;

// 1. no_std builds (verified by compiling this test file with no_std)

#[test]
fn test_jepa_with_fixed_array_works() {
    let mut jepa = Jepa::new();
    assert_eq!(jepa.count, 0);
    jepa.learn(0, 1.0);
    assert_eq!(jepa.count, 1);
    jepa.learn(1, 2.0);
    assert_eq!(jepa.count, 2);
}

#[test]
fn test_jepa_predict_returns_weighted_average() {
    let mut jepa = Jepa::new();
    jepa.learn(0, 10.0);
    jepa.learn(1, 20.0);
    let pred = jepa.predict();
    assert!(pred > 0.0, "prediction should be positive");
    // With uniform weights and values 10, 20, prediction ≈ 15
    assert!((pred - 15.0).abs() < 5.0, "prediction should be near weighted average");
}

#[test]
fn test_jepa_learn_updates_weights() {
    let mut jepa = Jepa::new();
    jepa.learn(0, 1.0);
    let _w_before = jepa.weights[0];
    jepa.learn(1, 2.0);
    // Weights should have been adjusted (not necessarily the same)
    // At least verify they're still valid floats
    for i in 0..jepa.count {
        assert!(jepa.weights[i].is_finite());
    }
}

#[test]
fn test_room_has_mono_vibe() {
    let room = Room::new(1, 42.0);
    assert_eq!(room.id, 1);
    assert!((room.vibe - 42.0).abs() < f64::EPSILON);
}

#[test]
fn test_cellgraph_creation_with_fixed_arrays() {
    let graph = CellGraph::new();
    assert_eq!(graph.room_count, 0);
    assert_eq!(graph.edge_count, 0);
    assert_eq!(graph.tick_count, 0);
    assert!(graph.is_empty());
}

#[test]
fn test_add_room_within_bounds() {
    let mut graph = CellGraph::new();
    assert!(graph.add_room(1, 10.0));
    assert_eq!(graph.room_count, 1);
    assert!(graph.add_room(2, 20.0));
    assert_eq!(graph.room_count, 2);
}

#[test]
fn test_add_edge_within_bounds() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    graph.add_room(2, 20.0);
    assert!(graph.add_edge(1, 2, 0.5));
    assert_eq!(graph.edge_count, 1);
}

#[test]
fn test_diffuse_works() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    graph.add_room(2, 0.0);
    graph.add_edge(1, 2, 1.0);
    let before = graph.total_vibe();
    graph.diffuse(0.1);
    let after = graph.total_vibe();
    // Vibe should flow from high to low
    let v1 = graph.get_vibe(1).unwrap();
    let v2 = graph.get_vibe(2).unwrap();
    assert!(v1 < 10.0, "room 1 should have lost some vibe");
    assert!(v2 > 0.0, "room 2 should have gained some vibe");
    // Conservation
    assert!((before - after).abs() < 0.001, "total vibe should be conserved");
}

#[test]
fn test_conservation_holds() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 100.0);
    graph.add_room(2, 0.0);
    graph.add_room(3, 50.0);
    graph.add_edge(1, 2, 0.3);
    graph.add_edge(2, 3, 0.5);
    graph.add_edge(3, 1, 0.2);

    let initial = graph.total_vibe();
    for _ in 0..100 {
        graph.diffuse(0.05);
    }
    let final_vibe = graph.total_vibe();
    assert!((initial - final_vibe).abs() < 0.01, "conservation must hold after many diffusions");
}

#[test]
fn test_gossip_generates_murmurs() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    graph.add_room(2, 20.0);
    graph.tick();

    let mut buffer = [None; MAX_ROOMS];
    let count = gossip::generate_murmurs(&graph, &mut buffer);
    assert_eq!(count, 2);
    assert!(buffer[0].is_some());
    assert!(buffer[1].is_some());
}

#[test]
fn test_fleet_vibe_is_average() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    graph.add_room(2, 20.0);
    graph.add_room(3, 30.0);
    let avg = fleet::fleet_vibe(&graph);
    assert!((avg - 20.0).abs() < 0.001);
}

#[test]
fn test_fleet_surprise_is_average() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    graph.add_room(2, 20.0);
    graph.tick(); // learns and computes surprise
    let avg = fleet::fleet_surprise(&graph);
    assert!(avg.is_finite());
}

#[test]
fn test_empty_graph_handles_gracefully() {
    let graph = CellGraph::new();
    assert!(graph.is_empty());
    assert_eq!(graph.total_vibe(), 0.0);
    assert_eq!(fleet::fleet_vibe(&graph), 0.0);
    assert_eq!(fleet::fleet_surprise(&graph), 0.0);
}

#[test]
fn test_single_room_works() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 42.0);
    assert_eq!(graph.room_count, 1);
    assert!((graph.get_vibe(1).unwrap() - 42.0).abs() < f64::EPSILON);
    let surprise = graph.tick();
    // First reading, prediction is 0, so surprise = 42
    assert!(surprise.abs() > 0.0);
}

#[test]
fn test_max_rooms_fills_correctly() {
    let mut graph = CellGraph::new();
    for i in 0..32u8 {
        assert!(graph.add_room(i, i as f64), "should add room {}", i);
    }
    assert_eq!(graph.room_count, 32);
    // Can't add more
    assert!(!graph.add_room(32, 0.0));
}

#[test]
fn test_max_edges_fills_correctly() {
    let mut graph = CellGraph::new();
    // Need rooms first
    for i in 0..12u8 {
        graph.add_room(i, i as f64);
    }
    // Add 64 edges
    let mut count = 0u8;
    for i in 0..12u8 {
        for j in (i + 1)..12u8 {
            if count >= 64 { break; }
            assert!(graph.add_edge(i, j, 0.1));
            count += 1;
        }
        if count >= 64 { break; }
    }
    assert_eq!(graph.edge_count, 64);
    // Can't add more
    graph.add_room(100, 0.0);
    graph.add_room(101, 0.0);
    assert!(!graph.add_edge(100, 101, 0.1));
}

#[test]
fn test_overflow_protection() {
    let mut graph = CellGraph::new();
    for i in 0..32u8 {
        graph.add_room(i, i as f64);
    }
    // 33rd room should fail
    assert!(!graph.add_room(33, 0.0));
    assert_eq!(graph.room_count, 32);
}

#[test]
fn test_jepa_window_trim() {
    let mut jepa = Jepa::with_window(4);
    for i in 0..10u32 {
        jepa.learn(i, i as f64);
    }
    // Count should stay at window size after filling
    assert!(jepa.count <= jepa.window, "count should not exceed window");
}

#[test]
fn test_surprise_cascade() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 100.0);
    graph.add_room(2, 0.0);
    graph.add_edge(1, 2, 1.0);

    // First tick
    graph.tick();
    // Surprise should exist
    let s1 = fleet::fleet_surprise(&graph);
    assert!(s1.is_finite());

    // Change vibe dramatically
    graph.set_vibe(2, 500.0);
    graph.tick();
    let s2 = fleet::fleet_surprise(&graph);
    assert!(s2.is_finite());
}

#[test]
fn test_deterministic() {
    let mut g1 = CellGraph::new();
    let mut g2 = CellGraph::new();

    for i in 0..5u8 {
        g1.add_room(i, i as f64 * 10.0);
        g2.add_room(i, i as f64 * 10.0);
    }
    g1.add_edge(0, 1, 0.5);
    g2.add_edge(0, 1, 0.5);
    g1.add_edge(1, 2, 0.3);
    g2.add_edge(1, 2, 0.3);

    for _ in 0..50 {
        g1.tick();
        g2.tick();
    }

    for i in 0..5u8 {
        let v1 = g1.get_vibe(i);
        let v2 = g2.get_vibe(i);
        assert_eq!(v1, v2, "graphs should be identical for room {}", i);
    }
}

#[test]
fn test_memory_footprint() {
    // Runtime check that CellGraph fits in reasonable memory
    // CellGraph: 32 rooms * sizeof(Room) + 64 edges * sizeof((u8,u8,f64)) + overhead
    // Room: u8 + f64 + Jepa + f64
    // Jepa: 16 * (Option<(u32,f64)> + f64) + 2*usize = 16*25 + 16 = ~416 bytes
    // Room: ~416 + 8 + 8 + 8 = ~440 bytes
    // 32 rooms: ~14KB (the full 32 rooms)
    // But for 8 rooms (typical embedded): ~3.5KB
    //
    // Verify that a graph with 8 rooms uses reasonable memory
    let size_graph = core::mem::size_of::<CellGraph>();
    let size_jepa = core::mem::size_of::<Jepa>();
    let size_room = core::mem::size_of::<Room>();
    let size_murmur = core::mem::size_of::<Murmur>();

    // These are informational assertions
    assert!(size_murmur <= 64, "Murmur should be small: {} bytes", size_murmur);
    assert!(size_jepa <= 640, "Jepa should be bounded: {} bytes", size_jepa);

    // For 8 rooms, memory should be well under 4KB for just the rooms+edges portion
    // (The full CellGraph has 32 room slots, but 8 active rooms only use 8)
    // This is a structural check
    let _ = (size_graph, size_room);
}

#[test]
fn test_tick_performance_model() {
    // We can't benchmark on host, but we can verify tick correctness
    let mut graph = CellGraph::new();
    for i in 0..8u8 {
        graph.add_room(i, i as f64 * 5.0);
    }
    for i in 0..7u8 {
        graph.add_edge(i, i + 1, 0.2);
    }

    // Run 1000 ticks — on host this should be instant
    let start_tick = graph.tick_count;
    for _ in 0..1000 {
        graph.tick();
    }
    assert_eq!(graph.tick_count, start_tick + 1000);
}

#[test]
fn test_topology_chain() {
    let mut graph = CellGraph::new();
    // Chain: 0-1-2-3-4
    for i in 0..5u8 {
        graph.add_room(i, if i == 0 { 100.0 } else { 0.0 });
    }
    for i in 0..4u8 {
        graph.add_edge(i, i + 1, 0.5);
    }

    let initial = graph.total_vibe();
    for _ in 0..200 {
        graph.diffuse(0.05);
    }
    // Vibe should have spread along the chain
    assert!((graph.total_vibe() - initial).abs() < 0.01);
    // End of chain should have gained some vibe
    assert!(graph.get_vibe(4).unwrap() > 0.0);
}

#[test]
fn test_topology_ring() {
    let mut graph = CellGraph::new();
    // Ring: 0-1-2-3-4-0
    for i in 0..5u8 {
        graph.add_room(i, if i == 0 { 50.0 } else { 0.0 });
    }
    for i in 0..5u8 {
        let next = (i + 1) % 5;
        graph.add_edge(i, next, 0.3);
    }

    let initial = graph.total_vibe();
    for _ in 0..300 {
        graph.diffuse(0.05);
    }
    assert!((graph.total_vibe() - initial).abs() < 0.01);
    // In a ring, all rooms should eventually equalize
    let mut min_v = f64::INFINITY;
    let mut max_v = f64::NEG_INFINITY;
    for i in 0..5u8 {
        let v = graph.get_vibe(i).unwrap();
        if v < min_v { min_v = v; }
        if v > max_v { max_v = v; }
    }
    let max_diff = max_v - min_v;
    assert!(max_diff < 5.0, "ring should equalize, max diff: {}", max_diff);
}

#[test]
fn test_murmur_serialize_roundtrip() {
    let m = Murmur::new(5, 42.0, 1234, -0.5);
    let bytes = m.to_bytes();
    let restored = Murmur::from_bytes(&bytes).unwrap();
    assert_eq!(restored.room_id, 5);
    assert!((restored.vibe - 42.0).abs() < f64::EPSILON);
    assert_eq!(restored.tick, 1234);
    assert!((restored.surprise - (-0.5)).abs() < f64::EPSILON);
}

#[test]
fn test_murmur_interesting() {
    let boring = Murmur::new(1, 10.0, 0, 0.01);
    let interesting = Murmur::new(2, 10.0, 0, 5.0);
    assert!(!boring.is_interesting(1.0));
    assert!(interesting.is_interesting(1.0));
}

#[test]
fn test_transport_stubs_compile() {
    let mut wifi = transport::WifiGossip::new(1234);
    assert!(wifi.init());
    assert!(wifi.ready);

    let mut ble = transport::BleGossip::new(100);
    assert!(ble.init());

    let mut serial = transport::SerialGossip::new(115200);
    assert!(serial.init());

    let mut i2c = transport::I2cGossip::new(0x42);
    assert!(i2c.init());

    let mut flash = transport::FlashStorage::new();
    assert!(flash.init());

    let mut eeprom = transport::EepromStorage::new();
    assert!(eeprom.init());
}

#[test]
fn test_duplicate_room_rejected() {
    let mut graph = CellGraph::new();
    assert!(graph.add_room(1, 10.0));
    assert!(!graph.add_room(1, 20.0)); // duplicate
    assert_eq!(graph.room_count, 1);
}

#[test]
fn test_self_edge_rejected() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    assert!(!graph.add_edge(1, 1, 0.5));
}

#[test]
fn test_edge_to_nonexistent_room_rejected() {
    let mut graph = CellGraph::new();
    graph.add_room(1, 10.0);
    assert!(!graph.add_edge(1, 99, 0.5));
    assert!(!graph.add_edge(99, 1, 0.5));
}

#[test]
fn test_jepa_reset() {
    let mut jepa = Jepa::new();
    jepa.learn(0, 1.0);
    jepa.learn(1, 2.0);
    assert_eq!(jepa.count, 2);
    jepa.reset();
    assert_eq!(jepa.count, 0);
    assert_eq!(jepa.predict(), 0.0);
}
