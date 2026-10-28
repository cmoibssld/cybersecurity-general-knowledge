use core::fmt;
use std::collections::HashSet;
use std::fmt::write;
use std::path::PathBuf;
use std::{collections::VecDeque, fs::create_dir_all};

use clap::Parser;

use reqwest::header::USER_AGENT;
use reqwest::redirect::Policy;
use reqwest::{Client, Response};
use scraper::{Html, Selector};

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
    url: String, // maybe use reqwest::Url...
    #[arg(short)]
    recursive: bool,
    #[arg(short, default_value("5"))]
    level: u32,
    #[arg(short, default_value=get_default_dl_path().into_os_string(), value_parser=parse_directory)]
    path: PathBuf,
}

#[derive(Debug, Clone)]
enum SpiderError {
    HttpFailure(reqwest::StatusCode),
    UnsupportedImage(String),
}

impl fmt::Display for SpiderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpiderError::HttpFailure(e) => write!(f, "HTTP failure, status: {}", e),
            SpiderError::UnsupportedImage(img) => write!(f, "Image type not supported: {img}"),
        }
    }
}

impl std::error::Error for SpiderError {}

// My algo for spider

// 1. Create a client.
// 1.1 Create a VecDeque: queue (URL as string)
// 1.2 Create a HashSet: history of visited/scanned url
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

async fn send_request(url: &str, client: &Client) -> Result<Response, Box<dyn std::error::Error>> {
    let response = client
        .get(url)
        .header(USER_AGENT, "arachnida/0.1")
        .send()
        .await?;
    Ok(response)
}

fn look_for_images(media_type: &str) -> bool {
    let img_extension = ["jpg", "jpeg", "png", "gif", "bmp"];
    if media_type
        .strip_prefix("image/")
        .is_some_and(|extension| img_extension.contains(&extension))
    {
        println!("I found an image");
        true
    } else {
        println!("This is not an available type: {media_type}");
        false
    }
}

fn get_proper_name(path: &PathBuf, file_name: &str, media_type: &str) -> PathBuf {
    let name = std::path::Path::new(&file_name);
    let stem = name.file_stem().unwrap().to_string_lossy();
    let extension = match name.extension() {
        Some(ext) => ext.to_string_lossy(),
        None => {
            let ext = media_type.rsplit_once("/").unwrap_or(("", media_type)).1;
            std::ffi::OsStr::new(ext).to_string_lossy()
        }
    };
    let mut destination = path.join(format!("{stem}.{extension}"));
    let mut counter = 1;

    while destination.exists() {
        destination = path.join(format!("{stem}_{counter}.{extension}"));
        counter += 1;
    }
    destination
}

async fn save_images(
    url: &str,
    body: &[u8],
    content_type: &str,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = url
        .rsplit_once("?")
        .unwrap_or((url, ""))
        .0
        .rsplit_once("/")
        .unwrap_or(("", url))
        .1;
    let path = get_proper_name(path, file_name, content_type);
    tokio::fs::write(path, &body).await?;
    println!("image {file_name} correctly saved");
    Ok(())
}

async fn scrap_image(
    url: &reqwest::Url,
    client: &Client,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = send_request(&url.to_string(), client).await?;
    if response.status().is_success() == false {
        return Err(Box::new(SpiderError::HttpFailure(response.status())));
    }
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .ok_or("no content-type")?
        .to_string();
    let media_type = content_type.split(';').next().unwrap_or("").trim();
    if look_for_images(&media_type) == true {
        let body = response.bytes().await?;
        save_images(&url.to_string(), &body, &media_type, path).await?;
        Ok(())
    } else {
        Err(Box::new(SpiderError::UnsupportedImage(
            media_type.to_string(),
        )))
    }
}

async fn scan_page(
    client: &Client,
    url: &str,
    queue: &mut VecDeque<String>,
    discovered: &mut HashSet<String>,
    path: &PathBuf,
) {
    // first send the request for the whole page
    if discovered.contains(url) {
        return;
    }
    eprintln!("Scanning page {url}");
    let response = match send_request(url, client).await {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Failed request for {url}: {error}");
            return;
        }
    };
    eprintln!("Status is {}", response.status());
    discovered.insert(url.to_string());
    if response.status().is_success() == false {
        return;
    }

    let Some(content_type) = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
    else {
        return;
    };
    let content_type = &content_type.to_string();
    let media_type = content_type.split(';').next().unwrap_or("").trim();

    if look_for_images(media_type) == true {
        let body = match response.bytes().await {
            Ok(body) => body,
            Err(error) => {
                eprintln!("Failed to read response for {url}: {error}");
                return;
            }
        };
        match save_images(url, &body, media_type, path).await {
            Ok(_) => {
                println!("Image saved: {url}");
            }
            Err(_) => {
                eprintln!("Could not save image at {url}");
            }
        };
        return;
    } else if media_type != "text/html" {
        return;
    }
    let body = match response.text().await {
        Ok(body) => body,
        Err(error) => {
            eprintln!("Failed to read response for {url}: {error}");
            return;
        }
    };

    let document = Html::parse_document(&body);
    eprintln!("Identifying html");
    // parse image
    let selector = Selector::parse("img[src]").unwrap();
    for element in document.select(&selector) {
        eprintln!("Looking for img[src]");
        if let Some(src) = element.value().attr("src") {
            let page_url = reqwest::Url::parse(url);
            if page_url.is_err() {
                continue;
            }
            let image_url = page_url.unwrap().join(src);
            if image_url.is_err() {
                continue;
            }
            if discovered.contains(&image_url.as_ref().unwrap().to_string()) {
                continue;
            }
            match scrap_image(image_url.as_ref().unwrap(), client, path).await {
                Ok(_) => {
                    discovered.insert(image_url.as_ref().unwrap().to_string());
                    println!("Image saved: {url}");
                }
                Err(_) => {
                    eprintln!("Could not save image at {url}");
                }
            }
        }
    }

    // parse ref aka links
    let selector = Selector::parse("a[href]").unwrap();
    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let page_url = reqwest::Url::parse(url);
            if page_url.is_err() {
                continue;
            }
            let ref_url = page_url.unwrap().join(link);
            if ref_url.is_err() {
                continue;
            }
            queue.push_back(ref_url.unwrap().to_string());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();
    println!("url: {}", param.url);
    println!("recursive: {}, level: {}", param.recursive, param.level);
    println!("path to save: {}", param.path.display());

    let client = Client::builder().redirect(Policy::none()).build()?;
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut discovered: HashSet<String> = HashSet::new();

    queue.push_front(param.url);

    for i in 0..param.level + 1 {
        println!("Going at level {i} of depth in the website");
        let round_size = queue.len();
        for _ in 0..round_size {
            let url = queue.pop_front().unwrap();
            scan_page(&client, &url, &mut queue, &mut discovered, &param.path).await;
        }
        if param.recursive == false {
            break;
        }
    }
    Ok(())
}
