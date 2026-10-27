use std::io::Read;
use std::path::PathBuf;

use wl_clipboard_rs::copy::{MimeType as CopyMime, Options, Source};
use wl_clipboard_rs::paste::{get_contents, ClipboardType, MimeType as PasteMime, Seat};

use crate::error::{Error, Res};

use super::uri;

const URI_LIST: &str = "text/uri-list";

pub fn read_paths() -> Option<Vec<PathBuf>> {
    let (mut pipe, _) = get_contents(ClipboardType::Regular, Seat::Unspecified, PasteMime::Specific(URI_LIST)).ok()?;
    let mut raw = String::new();
    pipe.read_to_string(&mut raw).ok()?;
    Some(uri::decode(&raw))
}

pub fn write_paths(paths: &[PathBuf]) -> Res<()> {
    let data = uri::encode(paths);
    Options::new()
        .copy(Source::Bytes(data.into_bytes().into()), CopyMime::Specific(URI_LIST.to_string()))
        .map_err(|_| Error::Clipboard)
}
