use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use x11_clipboard::Clipboard;

use crate::error::{Error, Res};

use super::uri;

static CLIPBOARD: OnceLock<Clipboard> = OnceLock::new();

fn clipboard() -> &'static Clipboard {
    CLIPBOARD.get_or_init(|| Clipboard::new().expect("failed to connect to X11 for clipboard access"))
}

pub fn read_paths() -> Option<Vec<PathBuf>> {
    let clip = clipboard();
    let target = clip.getter.get_atom("text/uri-list").ok()?;
    let bytes = clip.load(clip.getter.atoms.clipboard, target, clip.getter.atoms.property, Duration::from_secs(2)).ok()?;
    let raw = String::from_utf8(bytes).ok()?;
    Some(uri::decode(&raw))
}

pub fn write_paths(paths: &[PathBuf]) -> Res<()> {
    let clip = clipboard();
    let target = clip.setter.get_atom("text/uri-list").map_err(|_| Error::Clipboard)?;
    let data = uri::encode(paths);
    clip.store(clip.setter.atoms.clipboard, target, data.into_bytes()).map_err(|_| Error::Clipboard)
}
