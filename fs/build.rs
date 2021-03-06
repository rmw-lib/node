use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  tonic_build::configure()
    .build_client(false)
    .compile(&["proto/rmw.proto"], &["proto"])?;
  Ok(())
}
