use crate::Error;
use crate::MediaItem;
use crate::MediaKind;
use crate::TelegramBot;

const MIN_BYTES: usize = 1024;

pub fn read_album_item(item: &MediaItem) -> Result<Vec<u8>, Error> {
    let display = item.path.display().to_string();
    let bytes = std::fs::read(item.path).map_err(|source| Error::Read {
        path: display.clone(),
        source,
    })?;
    let kind = match item.kind {
        MediaKind::Photo => "preview",
        MediaKind::Video => "clip",
    };
    if bytes.len() < MIN_BYTES {
        return Err(Error::TooSmall {
            kind,
            path: display,
            bytes: bytes.len(),
        });
    }
    match item.kind {
        MediaKind::Photo if !TelegramBot::is_png(&bytes) => Err(Error::NotPng { path: display }),
        MediaKind::Video if !TelegramBot::has_ftyp(&bytes) => Err(Error::NotMp4 { path: display }),
        _ => Ok(bytes),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::read_album_item;
    use crate::Error;
    use crate::MediaItem;
    use crate::MediaKind;

    #[test]
    fn test_usage() {
        let missing = MediaItem {
            kind: MediaKind::Photo,
            path: Path::new("/nonexistent-album-item.png"),
        };
        let err = read_album_item(&missing).unwrap_err();
        assert!(err.to_string().contains("nonexistent-album-item.png"));
        let dir = std::env::temp_dir().join("telegram-bot-read-album-item-test");
        let _ = std::fs::create_dir_all(&dir);
        let fake_clip = dir.join("fake.mp4");
        let _ = std::fs::write(&fake_clip, vec![0_u8; 2048]);
        let video = MediaItem {
            kind: MediaKind::Video,
            path: &fake_clip,
        };
        assert!(matches!(read_album_item(&video), Err(Error::NotMp4 { .. })));
        let photo = MediaItem {
            kind: MediaKind::Photo,
            path: &fake_clip,
        };
        assert!(matches!(read_album_item(&photo), Err(Error::NotPng { .. })));
        let _ = std::fs::remove_file(&fake_clip);
    }
}
