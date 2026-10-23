use crate::deliver;

#[must_use]
pub fn status_in(
    artifact: &deliver::Artifact,
    previous: Option<&deliver::Manifest>,
) -> deliver::Status {
    let before = previous.and_then(|previous| {
        previous.artifacts.get(&artifact.fingerprint).or_else(|| {
            previous
                .artifacts
                .values()
                .find(|old| old.scene == artifact.scene && old.label == artifact.label)
        })
    });
    match before {
        None => deliver::Status::New,
        Some(old) if old.pixel_hash != artifact.pixel_hash => deliver::Status::Changed,
        Some(_) => deliver::Status::Same,
    }
}

#[cfg(test)]
mod tests {
    use super::status_in;

    #[test]
    fn test_usage() {
        let _ = status_in;
    }
}
