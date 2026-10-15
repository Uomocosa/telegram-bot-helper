use std::path::Path;

use crate::MediaKind;

#[derive(Debug, Clone, Copy)]
pub struct MediaItem<'a> {
    pub kind: MediaKind,
    pub path: &'a Path,
}

pub struct Creds {
    pub token: String,
    pub chat_id: String,
}
