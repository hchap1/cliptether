use std::path::PathBuf;

use percent_encoding::{percent_decode_str, utf8_percent_encode, NON_ALPHANUMERIC};

const SAFE: &percent_encoding::AsciiSet = &NON_ALPHANUMERIC.remove(b'/');

pub fn encode(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| format!("file://{}", utf8_percent_encode(&path.to_string_lossy(), SAFE)))
        .collect::<Vec<_>>()
        .join("\r\n")
}

pub fn decode(raw: &str) -> Vec<PathBuf> {
    raw.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.strip_prefix("file://"))
        .map(|path| percent_decode_str(path).decode_utf8_lossy().into_owned())
        .map(PathBuf::from)
        .collect()
}
