#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{read_paths, write_paths};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{read_paths, write_paths};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::{read_paths, write_paths};

use crate::clipboard::{collect, materialize, FileEntry};
use crate::error::Res;

pub fn read() -> Option<Vec<FileEntry>> {
    let paths = read_paths()?;
    let mut out = Vec::new();
    for path in paths {
        let name = path.file_name()?.to_string_lossy().into_owned();
        collect(&path, &name, &mut out);
    }
    Some(out)
}

pub fn write(entries: &[FileEntry]) -> Res<()> {
    let paths = materialize(entries)?;
    write_paths(&paths)
}
