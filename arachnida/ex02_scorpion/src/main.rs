// Trying to give the same output as EXIF tool
use clap::Parser;

use crate::exif_metadata::ExifMetadata;

mod exif_metadata;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Param {
    #[arg(required = true)]
    file: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();
    for name in param.file {
        let mut metadata = ExifMetadata::new(name);
        metadata.set_extension()?;
        if let Err(e) = metadata.parse_metadata().await {
            eprintln!("scorpion: {}: {e}", metadata.file_name);
            continue;
        }
        println!("{}\n{}", metadata.file_name, metadata);
    }
    Ok(())
}
