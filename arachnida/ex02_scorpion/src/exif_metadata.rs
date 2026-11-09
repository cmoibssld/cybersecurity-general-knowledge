use exif::{Reader, Tag, Value};
use gif::DecodeOptions;
use std::{
    fmt,
    io::{BufReader, Read},
};

// None means unavailable, rather than a fabricated zero or an empty string.
#[derive(Debug, Default)]
pub struct ExifMetadata {
    pub file_name: String,
    pub extension: Option<String>,
    pub file_size: Option<u64>,
    // Filesystem creation time and the photo capture time are separate values.
    pub created_at: Option<String>,
    pub captured_at: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x_res: Option<f64>,
    pub y_res: Option<f64>,
    pub res_unit: Option<String>,
    pub user_comment: Option<String>,
}

fn write_row(f: &mut fmt::Formatter<'_>, label: &str, value: impl fmt::Display) -> fmt::Result {
    let value = value.to_string();
    let mut lines = value.lines();
    writeln!(f, "{label:<28} : {}", lines.next().unwrap_or(""))?;
    for line in lines {
        writeln!(f, "{:28}   {line}", "")?;
    }
    Ok(())
}

fn write_optional<T: fmt::Display>(
    f: &mut fmt::Formatter<'_>,
    label: &str,
    value: &Option<T>,
) -> fmt::Result {
    match value {
        Some(value) => write_row(f, label, value),
        None => write_row(f, label, "Unavailable"),
    }
}

// to the collectors. This allows the same display for JPEG, PNG, GIF and BMP.
impl fmt::Display for ExifMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_row(f, "File Name", &self.file_name)?;
        write_optional(f, "File Extension", &self.extension)?;
        write_optional(
            f,
            "File Size",
            &self.file_size.map(|size| format!("{size} bytes")),
        )?;
        write_optional(f, "File Creation Date", &self.created_at)?;
        write_optional(f, "Photo Capture Date", &self.captured_at)?;
        write_optional(f, "Image Width", &self.width)?;
        write_optional(f, "Image Height", &self.height)?;
        write_optional(f, "X Resolution", &self.x_res)?;
        write_optional(f, "Y Resolution", &self.y_res)?;
        write_optional(f, "Resolution Unit", &self.res_unit)?;
        write_optional(f, "User Comment", &self.user_comment)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum ExMeError {
    ForbiddenExtension(String),
    NoExtensionFound(String),
    WrongBMPHeader(),
}

impl fmt::Display for ExMeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExMeError::ForbiddenExtension(ext) => {
                write!(
                    f,
                    "Extension not allowed: {ext}. Must be .jpg/jpeg, .png, .gif, .bmp"
                )
            }
            ExMeError::NoExtensionFound(name) => write!(f, "No extension identified: {name}"),
            ExMeError::WrongBMPHeader() => write!(f, "Cannot identify bmp header"),
        }
    }
}

impl std::error::Error for ExMeError {}

// first handle EXIF compatible files: .png
impl ExifMetadata {
    pub fn new(file_name: String) -> Self {
        ExifMetadata {
            file_name,
            extension: None,
            file_size: None,
            created_at: None,
            captured_at: None,
            width: None,
            height: None,
            x_res: None,
            y_res: None,
            res_unit: None,
            user_comment: None,
        }
    }

    pub fn set_extension(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let valid_ext = vec!["jpg", "jpeg", "png", "gif", "bmp"];
        let Some((_, extension)) = self.file_name.rsplit_once(".") else {
            return Err(Box::new(ExMeError::NoExtensionFound(
                self.file_name.to_string(),
            )));
        };
        if valid_ext.contains(&extension) {
            self.extension = Some(extension.to_string());
            Ok(())
        } else {
            Err(Box::new(ExMeError::ForbiddenExtension(
                extension.to_string(),
            )))
        }
    }

    pub async fn parse_metadata(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.extension == Some("jpg".to_string()) {
            self.use_exif_crate().await
        } else if self.extension == Some("jpeg".to_string()) {
            self.use_exif_crate().await
        } else if self.extension == Some("png".to_string()) {
            self.use_exif_crate().await
        } else if self.extension == Some("gif".to_string()) {
            self.use_gif_crate().await
        } else if self.extension == Some("bmp".to_string()) {
            self.use_bmp_crate().await
        } else {
            Ok(())
        }
    }

    async fn use_exif_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = open_file(&self.file_name).await?;
        if let Ok(metadata) = file.metadata() {
            self.file_size = Some(metadata.len());
        };
        let mut bufreader = BufReader::new(file);
        let exifreader = Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader)?;
        for f in exif.fields() {
            let value = f.display_value().with_unit(&exif).to_string();
            match f.tag {
                Tag::DateTimeOriginal => self.captured_at = Some(value),
                Tag::DateTime => self.created_at = Some(value),
                Tag::ImageWidth => self.width = f.value.get_uint(0),
                Tag::ImageLength => self.height = f.value.get_uint(0),
                Tag::XResolution => self.x_res = rational_to_f64(&f.value),
                Tag::YResolution => self.y_res = rational_to_f64(&f.value),

                Tag::ResolutionUnit => self.res_unit = Some(value),
                Tag::UserComment => self.user_comment = Some(value),
                _ => {}
            }
        }
        Ok(())
    }

    async fn use_gif_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = open_file(&self.file_name).await?;
        if let Ok(metadata) = file.metadata() {
            self.file_size = Some(metadata.len());
        };
        let mut bufreader = BufReader::new(file);

        let mut options = DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let decoder = options.read_info(&mut bufreader)?;

        self.width = Some(decoder.width() as u32);
        self.height = Some(decoder.height() as u32);
        Ok(())
    }

    async fn use_bmp_crate(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = open_file(&self.file_name).await?;
        if let Ok(metadata) = file.metadata() {
            self.file_size = Some(metadata.len());
        };

        let mut header = [0u8; 54];
        file.read_exact(&mut header)?;

        if &header[0..2] != b"BM" {
            return Err(Box::new(ExMeError::WrongBMPHeader()));
        }

        let width = i32::from_le_bytes(header[18..22].try_into()?);
        let height = i32::from_le_bytes(header[22..26].try_into()?);
        let x_ppm = i32::from_le_bytes(header[38..42].try_into()?);
        let y_ppm = i32::from_le_bytes(header[42..46].try_into()?);

        self.width = u32::try_from(width).ok();
        self.height = height.checked_abs().and_then(|h| u32::try_from(h).ok());
        self.x_res = (x_ppm > 0).then_some(x_ppm as f64 / 39.3701);
        self.y_res = (y_ppm > 0).then_some(y_ppm as f64 / 39.3701);

        self.res_unit = Some("DPI".into());

        Ok(())
    }
}

// Function out of exaf create
async fn open_file(file_name: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    Ok(tokio::fs::File::open(file_name).await?.into_std().await)
}

fn rational_to_f64(value: &Value) -> Option<f64> {
    match value {
        Value::Rational(values) => values.first().map(|r| r.num as f64 / r.denom as f64),
        _ => None,
    }
}
