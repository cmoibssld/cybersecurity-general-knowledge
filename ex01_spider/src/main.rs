use core::fmt;
use std::collections::HashSet;
use std::path::PathBuf;
use std::{collections::VecDeque, fs::create_dir_all};

use clap::Parser;

use reqwest::header::USER_AGENT;
use reqwest::redirect::Policy;
use reqwest::{Client, Response, Url};
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
    SearchedAlready(String),
    NoContentTypeHeader(),
    UselessContent(),
}

impl fmt::Display for SpiderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpiderError::HttpFailure(e) => write!(f, "HTTP failure, status: {}", e),
            SpiderError::UnsupportedImage(img) => write!(f, "Image type not supported: {img}"),
            SpiderError::SearchedAlready(url) => write!(f, "Ulr already searched: {}", url),
            SpiderError::NoContentTypeHeader() => write!(f, "Header content-type not found"),
            SpiderError::UselessContent() => {
                write!(f, "Content serves no purpose for this program")
            }
        }
    }
}

impl std::error::Error for SpiderError {}

async fn send_request(url: Url, client: &Client) -> Result<Response, Box<dyn std::error::Error>> {
    let response = client
        .get(url)
        .header(USER_AGENT, "arachnida/0.1")
        .send()
        .await?;
    if !response.status().is_success() && !response.status().is_redirection() {
        Err(Box::new(SpiderError::HttpFailure(response.status())))
    } else {
        Ok(response)
    }
}

fn is_an_image(media_type: &str) -> bool {
    let img_extension = ["jpg", "jpeg", "png", "gif", "bmp"];
    media_type
        .strip_prefix("image/")
        .is_some_and(|extension| img_extension.contains(&extension))
}

fn print_page_summary(url: &Url, downloaded_images: usize) {
    let suffix = if downloaded_images == 1 { "" } else { "s" };
    println!("{url}: {downloaded_images} image{suffix} downloaded");
}

fn unique_image_path(path: &PathBuf, file_name: &str, media_type: &str) -> PathBuf {
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

async fn save_image(
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
    let path = unique_image_path(path, file_name, content_type);
    tokio::fs::write(path, &body).await?;
    Ok(())
}

async fn download_image(
    url: &reqwest::Url,
    client: &Client,
    path: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = send_request(url.clone(), client).await?;
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
    if is_an_image(&media_type) == true {
        let body = response.bytes().await?;
        save_image(&url.to_string(), &body, &media_type, path).await?;
        Ok(())
    } else {
        Err(Box::new(SpiderError::UnsupportedImage(
            media_type.to_string(),
        )))
    }
}

struct PageLinks {
    images: Vec<Url>,
    pages: Vec<Url>,
}

fn extract_links(documents: &Html, base: &Url) -> PageLinks {
    let mut result = PageLinks {
        images: Vec::new(),
        pages: Vec::new(),
    };
    let selector = Selector::parse("img[src]").unwrap();
    for element in documents.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            let Ok(image_url) = base.join(src) else {
                continue;
            };
            result.images.push(image_url);
        }

        let selector = Selector::parse("a[href]").unwrap();
        for element in documents.select(&selector) {
            if let Some(src) = element.value().attr("ref") {
                let Ok(ref_url) = base.join(src) else {
                    continue;
                };
                result.pages.push(ref_url);
            }
        }
    }
    result
}

async fn scan_page(
    client: &Client,
    url: &Url,
    page_origin: &url::Origin,
    queue: &mut VecDeque<Url>,
    discovered: &mut HashSet<Url>,
    path: &PathBuf,
) -> Result<usize, Box<dyn std::error::Error>> {
    // first send the request for the whole page
    if discovered.contains(url) {
        return Err(Box::new(SpiderError::SearchedAlready(url.to_string())));
    }
    let response = send_request(url.clone(), client).await?;
    discovered.insert(url.clone());

    let Some(content_type) = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
    else {
        return Err(Box::new(SpiderError::NoContentTypeHeader()));
    };
    let content_type = &content_type.to_string();
    let media_type = content_type.split(';').next().unwrap_or("").trim();

    if is_an_image(media_type) == true {
        let body = response.bytes().await?;
        save_image(url.as_str(), &body, media_type, path).await?;
        return Ok(1);
    } else if media_type != "text/html" {
        return Err(Box::new(SpiderError::UselessContent()));
    }
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    let links = extract_links(&document, url);

    let mut downloaded_images = 0;
    // parse image
    for image in links.images {
        if discovered.contains(&image) {
            continue;
        }
        if let Err(error) = download_image(&image, client, path).await {
            eprintln!("Failed to download {image}: {error}");
            continue;
        }
        discovered.insert(image);
        downloaded_images += 1;
    }

    // parse ref aka links
    for ref_page in links.pages {
        if &ref_page.origin() != page_origin {
            continue;
        }
        if !discovered.contains(&ref_page) {
            queue.push_back(ref_page);
        }
    }
    Ok(downloaded_images)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let param = Param::parse();

    let client = Client::builder().redirect(Policy::limited(3)).build()?;
    let mut queue: VecDeque<Url> = VecDeque::new();
    let mut discovered: HashSet<Url> = HashSet::new();

    queue.push_front(Url::parse(&param.url)?);

    let page_origin = queue[0].origin();
    for _ in 0..param.level + 1 {
        let round_size = queue.len();
        for _ in 0..round_size {
            let url = queue.pop_front().unwrap();
            match scan_page(
                &client,
                &url,
                &page_origin,
                &mut queue,
                &mut discovered,
                &param.path,
            )
            .await
            {
                Ok(downloaded_images) => print_page_summary(&url, downloaded_images),
                Err(error) => eprintln!("Failed to process {url}: {error}"),
            }
        }
        if param.recursive == false {
            break;
        }
    }
    Ok(())
}
