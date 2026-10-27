use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::deliver::basic::enums::Status;
use crate::preview;
use crate::scene;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capture {
    pub path: PathBuf,
    pub artifact: deliver::Artifact,
    pub status: Status,
}

#[atomic_delegates]
impl Capture {
    #[must_use]
    pub const fn is_sendable(&self) -> bool {}
    #[must_use]
    pub const fn status_word(&self) -> &'static str {}
    #[must_use]
    pub fn is_resend_all_requested() -> bool {}
    pub fn collect(
        scene: &str,
        kind: scene::Kind,
        rendered: &[preview::RenderedState],
        previous: Option<&deliver::Manifest>,
    ) -> Result<Vec<Capture>, Error> {
    }
    #[must_use]
    pub fn changes_markdown(captures: &[Capture]) -> String {}
    pub fn assert_centered(&self, tolerance: u32) -> Result<(), Error> {}
    pub fn send_one(&self, caption: &str) -> Result<bool, Error> {}
    pub fn send_group(items: &[&Capture], caption: &str) -> Result<usize, Error> {}
    pub(crate) fn send_album(items: &[&Capture], caption: &str) -> Result<usize, Error> {}
    pub(crate) fn write_markdown_section(out: &mut String, title: &str, captures: &[&Capture]) {}
}

#[rustfmt::skip]
impl Capture {
    pub(crate) const fn telegram_media_kind(&self) -> telegram_bot_helper::MediaKind { match self.artifact.media { Media::Png => telegram_bot_helper::MediaKind::Photo, Media::Mp4 => telegram_bot_helper::MediaKind::Video } }
    pub(crate) fn is_mp4(&self) -> bool { self.path.extension().and_then(std::ffi::OsStr::to_str) == Some("mp4") }
}

#[cfg(test)]
mod tests {
    use super::Capture;
    use crate::deliver;
    use crate::scene;

    fn capture(status: deliver::Status) -> Capture {
        Capture {
            path: std::path::PathBuf::from("out/ui__one.png"),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: String::from("one"),
                fingerprint: String::from("ui|one"),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status,
        }
    }

    #[test]
    fn test_usage() {
        assert!(capture(deliver::Status::New).is_sendable());
        assert!(!capture(deliver::Status::Same).is_sendable());
        assert_eq!(capture(deliver::Status::Changed).status_word(), "changed");
        assert_ne!(Capture::changes_markdown(&[]), "");
        let png = capture(deliver::Status::New);
        assert!(!png.is_mp4());
        assert_eq!(
            png.telegram_media_kind(),
            telegram_bot_helper::MediaKind::Photo
        );
        let mut clip = capture(deliver::Status::New);
        clip.path = std::path::PathBuf::from("out/ui__clip.mp4");
        clip.artifact.media = deliver::Media::Mp4;
        assert!(clip.is_mp4());
        assert_eq!(
            clip.telegram_media_kind(),
            telegram_bot_helper::MediaKind::Video
        );
    }
}
