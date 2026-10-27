use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_app_kit::{NSPasteboard, NSPasteboardWriting};
use objc2_foundation::{NSArray, NSString, NSURL};

use crate::error::{Error, Res};

pub fn read_paths() -> Option<Vec<PathBuf>> {
    let pasteboard = NSPasteboard::generalPasteboard();
    let classes = NSArray::from_slice(&[NSURL::class()]);
    let objects = unsafe { pasteboard.readObjectsForClasses_options(&classes, None) }?;

    let mut paths = Vec::new();
    for object in objects.iter() {
        if let Some(url) = object.downcast_ref::<NSURL>() {
            if let Some(path) = url.path() {
                paths.push(PathBuf::from(path.to_string()));
            }
        }
    }

    (!paths.is_empty()).then_some(paths)
}

pub fn write_paths(paths: &[PathBuf]) -> Res<()> {
    let pasteboard = NSPasteboard::generalPasteboard();
    pasteboard.clearContents();

    let urls: Vec<Retained<NSURL>> =
        paths.iter().map(|path| NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy()))).collect();
    let objects: Vec<&ProtocolObject<dyn NSPasteboardWriting>> =
        urls.iter().map(|url| ProtocolObject::from_ref(&**url)).collect();
    let array = NSArray::from_slice(&objects);

    if pasteboard.writeObjects(&array) { Ok(()) } else { Err(Error::Clipboard) }
}
