@0xb726a8b6530a5a6e;

struct Vec3 {
    x @0 :Float32;
    y @1 :Float32;
    z @2 :Float32;
}

struct Quat {
    x @0 :Float32;
    y @1 :Float32;
    z @2 :Float32;
    w @3 :Float32;
}

struct ShipState {
    id @0 :UInt64;
    position @1 :Vec3;
    velocity @2 :Vec3;
    orientation @3 :Quat;
    angularVelocity @6 :Vec3;
    thrust @4 :Float32;
    desiredSpeed @7 :Float32;
    tick @5 :UInt64;
}

struct PlayerInput {
    # Joystick deflection: desired angular velocity (pitch, yaw, roll).
    # Server validates magnitude <= MAX_TURN_RATE.
    turnInput @0 :Vec3;
    # Desired speed. Server validates rate of change.
    desiredSpeed @1 :Float32;
    fire @2 :Bool;
    tick @3 :UInt64;
}

struct ClientMessage {
    union {
        input @0 :PlayerInput;
        ping :group {
            clientTime @1 :Float64;
        }
    }
}

struct ServerMessage {
    union {
        worldSnapshot @0 :List(ShipState);
        pong :group {
            clientTime @1 :Float64;
            serverTime @2 :Float64;
        }
        hitConfirm :group {
            target @3 :UInt64;
            tick @4 :UInt64;
        }
    }
}
