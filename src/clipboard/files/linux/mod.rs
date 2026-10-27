mod uri;
mod wayland;
mod x11;

use std::path::PathBuf;

use crate::error::Res;

fn is_wayland() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
}

pub fn read_paths() -> Option<Vec<PathBuf>> {
    if is_wayland() {
        wayland::read_paths()
    } else {
        x11::read_paths()
    }
}

pub fn write_paths(paths: &[PathBuf]) -> Res<()> {
    if is_wayland() {
        wayland::write_paths(paths)
    } else {
        x11::write_paths(paths)
    }
}
