fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &["protos/helloworld.proto", "protos/user.proto"],
        &["protos/"],
    )?;
    Ok(())
}
