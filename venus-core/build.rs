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
        // warn!("{}", entry?.path().display());
        tonic_build::compile_protos(entry.path().to_str().unwrap())?;
    }
    Ok(())
}
