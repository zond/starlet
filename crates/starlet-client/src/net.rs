// Network client — future implementation.
//
// Will connect to starlet-server via WebSocket (upgrade to WebTransport when available).
//
// Responsibilities:
// - Send PlayerInput each physics tick
// - Receive WorldSnapshot at ~20Hz
// - Measure RTT via ping/pong
// - Client-side prediction: store input history in ring buffer, on server state
//   receipt roll back to server tick and replay inputs forward
// - Other ships: interpolate between two most recent server snapshots
// - Combat: send fire events with tick number for lag-compensated hit detection
