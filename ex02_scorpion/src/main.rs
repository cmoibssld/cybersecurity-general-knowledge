// Trying to give the same output as EXIF tool
use clap::Parser;
use core::fmt;
use exif::{Exif, Reader};
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Param {
    #[arg(required = true)]
    file: Vec<String>,
}

#[derive(Debug, Clone)]
enum ScorpionError {
    WrongExtension(String),
}

impl fmt::Display for ScorpionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScorpionError::WrongExtension(file) => write!(f, "Unrecognized extension: {file}"),
        }
    }
}

fn correct_extension(name: &str) -> Result<String, ScorpionError> {
    let valid_ext = vec!["jpg", "jpeg", "png", "gif", "bmp"];
    let Some((_, extension)) = name.rsplit_once(".") else {
        return Err(ScorpionError::WrongExtension(name.to_string()));
    };
    if valid_ext.contains(&extension) {
        Ok(extension.to_string())
    } else {
        Err(ScorpionError::WrongExtension(name.to_string()))
    }
}

async fn open_file(file_name: &str) -> Result<File, Box<dyn Error>> {
    Ok(tokio::fs::File::open(file_name).await?.into_std().await)
    // let mut contents = vec![];
    // file.read_to_end(&mut contents).await?;
    // Ok(contents)
}

fn read_exif_with_crate(file: &mut File) -> Result<Exif, Box<dyn std::error::Error>> {
    let mut bufreader = BufReader::new(file);
    let exifreader = Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    Ok(exif)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();
    let exif_crate_ext = vec!["jpg", "jpeg", "png"];
    for name in param.file {
        let Ok(mut file) = open_file(&name).await else {
            eprintln!("Error: File not found - {}", name);
            continue;
        };
        let Ok(ext) = correct_extension(&name) else {
            continue;
        };
        if exif_crate_ext.contains(&ext.as_str()) {
            let Ok(exif) = read_exif_with_crate(&mut file) else {
                eprintln!("Error: failed retreiving EXIF from {ext}: {name}");
                continue;
            };
            for f in exif.fields() {
                println!(
                    "{} {} {}",
                    f.tag,
                    f.ifd_num,
                    f.display_value().with_unit(&exif)
                );
            }
        }

        println!("extension: {ext}");
    }

    Ok(())
}
