# grand-pattern-embedded

Grand Pattern for microcontrollers — `no_std`, fixed memory, ESP32/Arduino/ARM Cortex-M.

A mono-vibe architecture that runs on a potato. One `f64` per room, simple JEPA with bounded memory, gossip with small packets.

## Features

- **`no_std` compatible** — zero heap allocation, fixed-size arrays everywhere
- **JEPA predictor** — sliding window prediction with learned weights (max 16 readings)
- **CellGraph** — up to 32 rooms, 64 edges, diffusion-based vibe propagation
- **Murmur gossip** — 32-byte fixed messages, serializable for any transport
- **Transport stubs** — Wi-Fi, BLE, UART, I2C (compile for any target, implement on hardware)

## Architecture

```
Room ─── Vibe (f64) ─── JEPA (predictor)
  │
  ├── CellGraph (32 rooms, 64 edges, diffusion)
  │
  └── Murmur (32-byte gossip message)
       │
       └── Transport (Wi-Fi / BLE / UART / I2C)
```

## Memory

| Component | Size |
|-----------|------|
| Murmur | 32 bytes |
| JEPA | ~400 bytes |
| Room | ~420 bytes |
| CellGraph (8 rooms) | ~4 KB |
| CellGraph (32 rooms) | ~14 KB |

## Usage

```rust
#![no_std]

use grand_pattern_embedded::{CellGraph, Murmur};

let mut graph = CellGraph::new();
graph.add_room(1, 42.0);
graph.add_room(2, 0.0);
graph.add_edge(1, 2, 0.5);

// Tick: JEPA learns + vibes diffuse
let surprise = graph.tick();

// Gossip: generate murmurs for transport
let mut murmurs = [None; 32];
let count = grand_pattern_embedded::gossip::generate_murmurs(&graph, &mut murmurs);

// Serialize
let bytes = murmurs[0].unwrap().to_bytes();
```

## ESP32 Integration

```rust
#[cfg(feature = "esp32")]
use grand_pattern_embedded::transport::WifiGossip;

let mut wifi = WifiGossip::new(4321);
wifi.init(); // real impl uses esp-wifi
```

## License

MIT
