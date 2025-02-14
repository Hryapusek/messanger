fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure().build_server(true)
              .out_dir("./src/protobufs")
              .compile_protos(&["proto/auth.proto"], &["proto"])?;
  Ok(())
}