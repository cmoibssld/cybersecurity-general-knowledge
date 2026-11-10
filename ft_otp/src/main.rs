use clap::{ArgGroup, Parser};
use std::fs::File;
use std::path::PathBuf;
mod generate_key;

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("mode").required(true).args(["generate", "key"])))]
struct Param {
    /// Encrypt and store key
    #[arg(short)]
    generate: bool,
    /// Generate an OTP
    #[arg(short)]
    key: bool,
    /// Hex key to encrypt OR a key to create an OTP
    file: PathBuf,
}

fn check_file(file: &PathBuf) -> Result<File, Box<dyn std::error::Error>> {
    Ok(File::open(file)?) // this open is readonly !
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();
    let file = check_file(&param.file)?;
    if param.generate == true {
        let file_path = generate_key::generate_key(file)?;
        println!("A key was successfully savec in {}", file_path.display());
    } else {
        println!("Still need to create a new password");
    }
    Ok(())
}
