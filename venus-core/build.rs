use walkdir::WalkDir;

#[allow(unused)]
macro_rules! warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new("proto") {
        let entry = entry?;
        if entry.clone().file_type().is_dir() {
            continue;
        }
        tonic_build::configure()
            .build_server(false)
            .compile_protos(&[entry.path().to_str().unwrap()], &["proto"])?;
    }
    Ok(())
}
