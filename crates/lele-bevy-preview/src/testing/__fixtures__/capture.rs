use std::path::PathBuf;

use crate::deliver;
use crate::testing;

pub fn capture(
    path: impl Into<PathBuf>,
    label: &str,
    media: deliver::Media,
    status: deliver::Status,
) -> deliver::Capture {
    let mut artifact = testing::artifact("ui", label, "hash");
    artifact.media = media;
    deliver::Capture {
        path: path.into(),
        artifact,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::capture;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let item = capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::New,
        );
        assert_eq!(item.artifact.label, "rest");
    }
}
