pub mod constants;
pub mod physics;
pub mod types;

pub mod protocol_capnp {
    include!(concat!(env!("OUT_DIR"), "/protocol_capnp.rs"));
}
