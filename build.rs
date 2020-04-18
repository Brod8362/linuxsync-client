extern crate protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["protos/packet.proto", "protos/auth.proto"])
        .include("protos")
        .run()
        .expect("Running protoc failed.");
}