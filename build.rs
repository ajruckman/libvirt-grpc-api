fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/protoc")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(
            &["proto/libvirt_api.proto"],
            &["proto/"],
        )?;
    Ok(())
}