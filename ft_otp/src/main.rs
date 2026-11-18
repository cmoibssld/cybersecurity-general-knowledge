use clap::{ArgGroup, Parser};
use std::fs::File;
use std::path::PathBuf;
use std::process::ExitCode;

mod generate_key;
mod password;

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

fn main() -> ExitCode {
    // Param::parse() calls std::process::exit() on error/--help, which skips
    // destructors and leaves clap's allocations "still reachable" in valgrind.
    // try_parse() lets us print the error and return normally, so it is dropped.
    let param = match Param::try_parse() {
        Ok(p) => p,
        Err(e) => {
            let _ = e.print(); // stderr for errors, stdout for --help/--version
            return ExitCode::from(e.exit_code() as u8);
        }
    };
    match run(param) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ft_otp: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(param: Param) -> Result<(), Box<dyn std::error::Error>> {
    let file = check_file(&param.file)?;
    if param.generate == true {
        let file_path = generate_key::generate_key(file)?;
        println!("A key was successfully saved in {}", file_path.display());
    } else {
        println!("Still need to create a new password");
    }
    Ok(())
}
