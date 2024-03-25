fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path = "../grpctest-proto";

    // hello_world.proto
    let hello_world_proto_path = [proto_path, "helloworld.proto"].join("/");
    std::env::set_var("PROTOC", [proto_path, "binary-dependency/protoc/bin/protoc.exe"].join("/"));
    tonic_build::configure()
        .build_server(false)
        .compile(&[&hello_world_proto_path], &[proto_path])
        .expect("Failed to compile proto file");
    std::env::remove_var("PROTOC");
    println!("cargo:rerun-if-changed={}", &hello_world_proto_path);

    Ok(())
}