use std::collections::HashMap;

use crate::deliver;
use crate::report::Report;

#[must_use]
pub fn from_report_and_manifest(
    report: &Report,
    filtered: Option<&deliver::Manifest>,
) -> deliver::FreshDelivery {
    let manifest_state = filtered.as_ref().map_or_else(
        || String::from("missing"),
        |manifest| format!("loaded:{}", manifest.artifacts.len()),
    );
    let mut captures = Vec::new();
    let mut olds: HashMap<String, String> = HashMap::new();
    for capture in &report.captures {
        let status = capture.artifact.status_in(filtered);
        let old = filtered.map_or(String::new(), |manifest| {
            manifest.previous_hash_for(&capture.artifact)
        });
        olds.insert(capture.artifact.fingerprint.clone(), old);
        captures.push(deliver::Capture {
            path: capture.path.clone(),
            artifact: capture.artifact.clone(),
            status,
        });
    }
    let clip = report.clip.as_ref().map(|previous| {
        let status = previous.artifact.status_in(filtered);
        let old = filtered.map_or(String::new(), |manifest| {
            manifest.previous_hash_for(&previous.artifact)
        });
        olds.insert(previous.artifact.fingerprint.clone(), old);
        deliver::Capture {
            path: previous.path.clone(),
            artifact: previous.artifact.clone(),
            status,
        }
    });
    deliver::FreshDelivery {
        captures,
        clip,
        old_hashes: olds,
        manifest_state,
    }
}

#[cfg(test)]
mod tests {
    use super::from_report_and_manifest;

    #[test]
    fn test_usage() {
        let _ = from_report_and_manifest;
    }
}
