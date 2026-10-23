use std::path::Path;

use crate::MediaKind;

#[derive(Debug, Clone, Copy)]
pub struct MediaItem<'a> {
    pub kind: MediaKind,
    pub path: &'a Path,
}
