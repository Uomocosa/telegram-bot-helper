use atomic_delegate_macros::atomic_delegates;

use crate::Error;
use crate::deliver;

pub struct Report {
    pub captures: Vec<deliver::Capture>,
    pub clip: Option<deliver::Capture>,
}

impl std::fmt::Display for Report {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&str> = self
            .captures
            .iter()
            .map(|capture| capture.artifact.label.as_str())
            .collect();
        write!(
            formatter,
            "{} capture(s): {}",
            self.captures.len(),
            names.join(", ")
        )
    }
}

#[rustfmt::skip]
impl Report {
    #[must_use]
    pub const fn counts(&self) -> usize { self.captures.len() }
    #[must_use]
    pub fn first(&self) -> Option<&deliver::Capture> { self.captures.first() }
}

#[atomic_delegates]
impl Report {
    pub fn assert_centered(&self, tolerance: u32) -> Result<(), Error> {}
}

#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::report::Report;
    use crate::scene;
    use std::path::PathBuf;

    // needed helper: fixture shared by the tests in this file
    fn capture(label: &str, status: deliver::Status) -> deliver::Capture {
        deliver::Capture {
            path: PathBuf::from(format!("out/{label}.png")),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: label.to_string(),
                fingerprint: format!("ui|{label}"),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status,
        }
    }

    #[test]
    fn test_usage() {
        let report = Report {
            captures: vec![
                capture("rest", deliver::Status::Same),
                capture("hover", deliver::Status::Changed),
            ],
            clip: Some(capture("clip", deliver::Status::Changed)),
        };
        let text = report.to_string();
        assert!(text.contains("2 capture(s)"));
        assert!(text.contains("rest"));
        assert!(text.contains("hover"));
        assert_eq!(report.counts(), 2);
        assert!(report.first().is_some());
        assert!(report.clip.is_some());
    }
}
