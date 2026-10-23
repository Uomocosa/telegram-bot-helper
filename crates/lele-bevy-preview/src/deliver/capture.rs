use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;
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
    }
}
