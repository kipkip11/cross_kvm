use crate::protocol::ClipboardPayload;
use anyhow::Result;
use arboard::{Clipboard, ImageData};
use std::borrow::Cow;

pub struct ClipboardManager {
    clipboard: Clipboard,
    last_text: Option<String>,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        let clipboard = Clipboard::new()?;
        Ok(Self {
            clipboard,
            last_text: None,
        })
    }

    pub fn poll_change(&mut self) -> Option<ClipboardPayload> {
        if let Ok(text) = self.clipboard.get_text() {
            if self.last_text.as_ref() != Some(&text) && !text.is_empty() {
                self.last_text = Some(text.clone());
                return Some(ClipboardPayload::Text(text));
            }
        }
        None
    }

    pub fn apply_payload(&mut self, payload: ClipboardPayload) -> Result<()> {
        match payload {
            ClipboardPayload::Text(text) => {
                self.last_text = Some(text.clone());
                self.clipboard.set_text(text)?;
            }
            ClipboardPayload::Image {
                width,
                height,
                bytes,
            } => {
                let img_data = ImageData {
                    width,
                    height,
                    bytes: Cow::Owned(bytes),
                };
                self.clipboard.set_image(img_data)?;
            }
        }
        Ok(())
    }
}
