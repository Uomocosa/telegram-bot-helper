use crate::deliver;

#[must_use]
pub fn previous_hash_for(manifest: &deliver::Manifest, artifact: &deliver::Artifact) -> String {
    let found = manifest.artifacts.get(&artifact.fingerprint).or_else(|| {
        manifest
            .artifacts
            .values()
            .find(|old| old.scene == artifact.scene && old.label == artifact.label)
    });
    found.map_or(String::new(), |old| old.pixel_hash.clone())
}

#[cfg(test)]
mod tests {
    use super::previous_hash_for;

    #[test]
    fn test_usage() {
        let _ = previous_hash_for;
    }
}
