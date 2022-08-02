fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/generated")
        .compile(&["proto/echo.proto"], &["proto"])?;
    println!("Built proto files");
    Ok(())
}
