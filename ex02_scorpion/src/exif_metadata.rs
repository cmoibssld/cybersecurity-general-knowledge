use io::Error;
use std::{fmt, io, path::PathBuf};
use tokio::sync::OwnedMappedMutexGuard;

// AI generated: shared metadata model. Collectors fill the fields they can read;
// None means unavailable, rather than a fabricated zero or an empty string.
#[derive(Debug, Default)]
pub struct ExifMetadata {
    pub file_name: String,
    pub extension: Option<String>,
    pub file_size: Option<u64>,
    // AI generated: dates are display-ready strings supplied by the collector.
    // Filesystem creation time and the photo capture time are separate values.
    pub created_at: Option<String>,
    pub captured_at: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    // AI generated: resolution may be fractional; its unit is stored separately.
    pub x_res: Option<f64>,
    pub y_res: Option<f64>,
    pub res_unit: Option<String>,
    pub user_comment: Option<String>,
}

// AI generated: render aligned ExifTool-style rows and propagate write errors.
// Multiline comments receive indented continuation lines.
fn write_row(f: &mut fmt::Formatter<'_>, label: &str, value: impl fmt::Display) -> fmt::Result {
    let value = value.to_string();
    let mut lines = value.lines();
    writeln!(f, "{label:<28} : {}", lines.next().unwrap_or(""))?;
    for line in lines {
        writeln!(f, "{:28}   {line}", "")?;
    }
    Ok(())
}

// AI generated: consistent handling of missing metadata across image formats.
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

// AI generated: formatting only; file access and metadata extraction belong
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

// NOT AI Generated, my own work

#[derive(Debug, Clone)]
enum ExMeError {
    FileNotFound(String),
    ForbiddenExtension(String),
    NoExtensionFound(String),
    OpenError(),
}

impl fmt::Display for ExMeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExMeError::FileNotFound(name) => write!(f, "File not found: {name}"),
            ExMeError::ForbiddenExtension(ext) => {
                write!(
                    f,
                    "Extension not allowed: {ext}. Must be .jpg/jpeg, .png, .gif, .bmp"
                )
            }
            ExMeError::OpenError(err) => write!(f, "Cannot open {err}"),
            ExMeError::NoExtensionFound(name) => write!(f, "No extension identified: {name}"),
        }
    }
}

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

    pub fn set_extension(&mut self) -> Result<(), ExMeError> {
        let valid_ext = vec!["jpg", "jpeg", "png", "gif", "bmp"];
        let Some((_, extension)) = self.file_name.rsplit_once(".") else {
            return Err(ExMeError::NoExtensionFound(self.file_name.to_string()));
        };
        if valid_ext.contains(&extension) {
            self.extension = Some(extension.to_string());
            Ok(())
        } else {
            Err(ExMeError::ForbiddenExtension(extension.to_string()))
        }
    }

    pub fn parse_metadata(&mut self, file_path: &PathBuf) -> Result<(), ExMeError> {
        Ok(())
    }
}
