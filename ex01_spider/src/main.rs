use std::path::PathBuf;
use std::{collections::HashMap, fs::create_dir_all};

use clap::{
    Parser,
    builder::{ArgPredicate, OsStr},
};

use reqwest::{Client, Response};

fn get_default_dl_path() -> PathBuf {
    PathBuf::from("./data")
}

fn parse_directory(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err("path must not be empty".into());
    }
    let path = PathBuf::from(value);
    if !path.is_dir() {
        match create_dir_all(&path) {
            Ok(_) => {
                return Ok(path);
            }
            Err(_) => return Err("Cannot create directory".into()),
        }
    }
    Ok(path)
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Param {
    url: String,
    #[arg(short)]
    recursive: bool,
    #[arg(short, default_value("5"))]
    level: u32,
    #[arg(short, default_value=get_default_dl_path().into_os_string(), value_parser=parse_directory)]
    path: PathBuf,
}

// My algo for spider

// 1. Create a client.
// 1.1 Create one map: queue (URL as string, searched? as bool)
// 1.2 Add the param url to the queue
//
// 2. For each url inside the queue
// 2.1 Make a request on the URL
//     2.2 Set the value in the map as 'true'
//     2.3 Inspect the response:
//         if not an image: look for connexion in the text, add every possble link (inside the domain name) to the queue if not already inside (don't modify the value, solely look for the key)
//         if an image: download it, create directory according to path given.
//
// 3. If recursion level is reached
//     return
// Else
//     start step 2 again
//
// Because of multiple links at the same time, tokio and async seemed the best option

async fn send_request(
    param: &Param,
    client: &Client,
) -> Result<Response, Box<dyn std::error::Error>> {
    let response = client.get(&param.url).send().await?;
    Ok(response)
}

fn look_for_images(response: &Response) {
    let content_type = response.headers().get("content-type");
    if content_type.is_none() {
        return;
    }
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();
    println!("url: {}", param.url);
    println!("recursive: {}, level: {}", param.recursive, param.level);
    println!("path to save: {}", param.path.display());

    let client = Client::new(); // blocking !! For now at least
    let mut queue: HashMap<bool, &str> = HashMap::new();
    let first = get_first_response(&param, &client)?;
    look_for_images(&first);
    if param.recursive == false {
        return Ok(());
    }
    println!("coming here");
    for lv in 1..param.level {
        println!("recursing level {lv}");
    }
}
