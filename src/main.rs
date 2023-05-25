extern crate chrono;
extern crate clipboard;

use chrono::prelude::*;
use clipboard::ClipboardProvider;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[cfg(target_os = "macos")]
type PlatformSpecificClipboardContext = clipboard::osx_clipboard::OSXClipboardContext;

#[cfg(target_os = "linux")]
type PlatformSpecificClipboardContext = clipboard::x11_clipboard::X11ClipboardContext;

#[cfg(target_os = "windows")]
type PlatformSpecificClipboardContext = clipboard::windows_clipboard::WindowsClipboardContext;

fn main() {
    let output_path = std::env::args().nth(1).unwrap();
    let mut file = OpenOptions::new().append(true).open(output_path).unwrap();
    let mut clipboard: PlatformSpecificClipboardContext = ClipboardProvider::new().unwrap();
    let mut old_clipboard_content = clipboard.get_contents().unwrap_or_else(|_| "".to_string());

    loop {
        let new_clipboard_content = clipboard.get_contents().unwrap_or_else(|_| "".to_string());
        if new_clipboard_content != old_clipboard_content {
            let timestamp = Utc::now();
            writeln!(
                file,
                "{}:\n{}",
                timestamp.to_rfc3339(),
                new_clipboard_content
            )
            .unwrap();
            old_clipboard_content = new_clipboard_content;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
