fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &["../../proto/content_storage/v1/content_storage_service.proto"],
        &["../../proto"],
    )?;
    Ok(())
}
