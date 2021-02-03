fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(false)
        .format(true)
        .out_dir("controller")
        .compile(
            &["protos/blockchain.proto"],
            &["protos"],
        )?;
    Ok(())
}
