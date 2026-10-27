use std::path::PathBuf;

use clipboard_win::{formats, get_clipboard, set_clipboard};

use crate::error::{Error, Res};

pub fn read_paths() -> Option<Vec<PathBuf>> {
    let list: Vec<String> = get_clipboard(formats::FileList).ok()?;
    Some(list.into_iter().map(PathBuf::from).collect())
}

pub fn write_paths(paths: &[PathBuf]) -> Res<()> {
    let list: Vec<String> = paths.iter().map(|path| path.to_string_lossy().into_owned()).collect();
    set_clipboard(formats::FileList, list).map_err(|_| Error::Clipboard)
}
