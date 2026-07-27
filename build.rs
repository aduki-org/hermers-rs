fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only compile protos when the `grpc` feature is enabled.
    if std::env::var_os("CARGO_FEATURE_GRPC").is_none() {
        return Ok(());
    }

    let proto_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../proto");
    let protos = [
        "contact.proto",
        "mail.proto",
        "session.proto",
        "sync.proto",
        "tier.proto",
        "usage.proto",
        "spam.proto",
        "storage.proto",
        "feeds.proto",
        "security.proto",
    ];

    println!("cargo:rerun-if-changed={}", proto_dir.display());
    for name in &protos {
        println!("cargo:rerun-if-changed={}/{}", proto_dir.display(), name);
    }

    let paths: Vec<_> = protos.iter().map(|p| proto_dir.join(p)).collect();
    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(&paths, &[proto_dir])?;
    Ok(())
}
