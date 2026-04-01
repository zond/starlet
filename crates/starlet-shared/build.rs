fn main() {
    capnpc::CompilerCommand::new()
        .file("protocol.capnp")
        .run()
        .expect("capnp compile failed");
}
