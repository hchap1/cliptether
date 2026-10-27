use objc2_app_kit::{NSPasteboard, NSPasteboardTypePNG};

use crate::clipboard::Content;

// Screenshots land on the pasteboard as PNG. Asking for TIFF (as arboard does) makes AppKit
// convert on demand, which crashes inside ImageIO on macOS 15.7, so decode the PNG ourselves.
// Returns None when there is no PNG at all, so callers can still fall back to other formats.
pub fn read_png() -> Option<Option<Content>> {
    let pasteboard = NSPasteboard::generalPasteboard();
    let data = unsafe { pasteboard.dataForType(NSPasteboardTypePNG) }?;
    let image = image::load_from_memory_with_format(&data.to_vec(), image::ImageFormat::Png).ok().map(|i| i.into_rgba8());

    Some(image.map(|rgba| Content::Image { width: rgba.width(), height: rgba.height(), rgba: rgba.into_raw() }))
}
