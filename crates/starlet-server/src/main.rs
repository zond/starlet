// Starlet multiplayer server — future implementation.
//
// Architecture:
// - Runs authoritative physics via starlet_shared::physics::step_ship()
// - Receives PlayerInput from clients via UDP (QUIC/WebTransport)
// - Validates inputs (anti-cheat: clamp thrust, check orientation rate)
// - Broadcasts WorldSnapshot at ~20Hz to all clients
// - Lag-compensated hit detection: rewinds positions to firing tick
// - Uses Cap'n Proto for zero-copy message reading from network buffers

fn main() {
    println!("starlet-server: not yet implemented");
}
