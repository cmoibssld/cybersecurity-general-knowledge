use std::env;
use std::path::PathBuf;

use clap::{
    Parser,
    builder::{ArgPredicate, OsStr},
};
use reqwest::Client;

fn get_default_dl_path() -> PathBuf {
    PathBuf::from("./data")
}

fn parse_directory(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err("path must not be empty".into());
    }
    let path = PathBuf::from(value);
    if !path.is_dir() {
        return Err("path must point to an existing directory".into());
    }
    Ok(path)
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    url: String,
    #[arg(short)]
    recursive: bool,
    #[arg(
        short,
        default_value_if("recursive", ArgPredicate::Equals(OsStr::from("true")), "5")
    )]
    level: Option<u32>,
    #[arg(short, default_value=get_default_dl_path().into_os_string(), value_parser=parse_directory)]
    path: PathBuf,
}

// create a client.
// Make a request on the URL

fn main() {
    let cli = Cli::parse();
    println!("url: {}", cli.url);
    println!(
        "recursive: {}, level: {}",
        cli.recursive,
        cli.level.unwrap_or_default()
    );
    println!("path to save: {}", cli.path.display());
    let client = Client::new();
}
