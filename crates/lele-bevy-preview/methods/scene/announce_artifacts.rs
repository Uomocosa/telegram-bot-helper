use crate::deliver;

pub fn announce_artifacts(captures: &[deliver::Capture], clip: Option<&deliver::Capture>) {
    for capture in captures {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            capture.path.display(),
            capture.status_word(),
            capture.artifact.label
        );
    }
    if let Some(clip) = clip {
        println!(
            "PREVIEW_ARTIFACT={} status={} label={}",
            clip.path.display(),
            clip.status_word(),
            clip.artifact.label
        );
    }
    println!(
        "PREVIEW_MANIFEST={}",
        deliver::basic::constants::MANIFEST_FILE
    );
}

#[cfg(test)]
mod tests {
    use super::announce_artifacts;

    #[test]
    fn test_usage() {
        announce_artifacts(&[], None);
    }
}
