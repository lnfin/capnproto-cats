fn main {
    ::capnpc::CompilerCommand::new()
        .file("schema/cats.capnp")
        .run()
        .expect("error");
}