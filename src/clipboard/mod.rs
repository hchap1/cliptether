mod files;

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::error::{Error, Res};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Content {
    Text(String),
    Image { width: u32, height: u32, rgba: Vec<u8> },
    Files(Vec<FileEntry>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub data: Vec<u8>,
}

pub fn read() -> Option<Content> {
    if let Some(entries) = files::read() {
        if !entries.is_empty() {
            return Some(Content::Files(entries));
        }
    }

    let mut clipboard = arboard::Clipboard::new().ok()?;

    if let Ok(image) = clipboard.get_image() {
        return Some(Content::Image {
            width: image.width as u32,
            height: image.height as u32,
            rgba: image.bytes.into_owned(),
        });
    }

    if let Ok(text) = clipboard.get_text() {
        return Some(Content::Text(text));
    }

    None
}

pub fn write(content: Content) -> Res<()> {
    match content {
        Content::Text(text) => {
            let mut clipboard = arboard::Clipboard::new().map_err(|_| Error::Clipboard)?;
            clipboard.set_text(text).map_err(|_| Error::Clipboard)
        }
        Content::Image { width, height, rgba } => {
            let mut clipboard = arboard::Clipboard::new().map_err(|_| Error::Clipboard)?;
            let image = arboard::ImageData { width: width as usize, height: height as usize, bytes: rgba.into() };
            clipboard.set_image(image).map_err(|_| Error::Clipboard)
        }
        Content::Files(entries) => files::write(&entries),
    }
}

// Writes received file entries to a fresh directory and returns the top-level paths
// (so a nested "dir/file.txt" entry collapses into a single "dir" path on disk).
fn materialize(entries: &[FileEntry]) -> Res<Vec<PathBuf>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|_| Error::Clipboard)?.as_nanos();

    let base = match directories::ProjectDirs::from("", "", crate::DAEMON_NAME) {
        Some(dirs) => dirs.cache_dir().join("received").join(stamp.to_string()),
        None => return Err(Error::NoSuitableConfigLocation),
    };

    let mut top_level = Vec::new();

    for entry in entries {
        let path = base.join(&entry.name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &entry.data)?;

        let top = base.join(entry.name.split('/').next().unwrap_or(&entry.name));
        if !top_level.contains(&top) {
            top_level.push(top);
        }
    }

    Ok(top_level)
}

// Reads a (possibly nested) path from disk into flat file entries, relative to its own name.
fn collect(path: &std::path::Path, name: &str, out: &mut Vec<FileEntry>) {
    if path.is_dir() {
        let Ok(dir) = std::fs::read_dir(path) else { return };
        for child in dir.flatten() {
            let child_name = format!("{}/{}", name, child.file_name().to_string_lossy());
            collect(&child.path(), &child_name, out);
        }
    } else if let Ok(data) = std::fs::read(path) {
        out.push(FileEntry { name: name.to_string(), data });
    }
}
