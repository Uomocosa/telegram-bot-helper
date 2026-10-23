use crate::deliver;
use crate::preview;
use crate::scene;
use std::collections::BTreeMap;
use std::path::PathBuf;

// needed helper: fixture shared by the tests in this file
fn artifact(scene: &str, label: &str, fingerprint: &str, pixels: &str) -> deliver::Artifact {
    deliver::Artifact {
        scene: scene.to_string(),
        label: label.to_string(),
        fingerprint: fingerprint.to_string(),
        pixel_hash: pixels.to_string(),
        media: deliver::Media::Png,
        kind: scene::Kind::System,
    }
}

// needed helper: fixture shared by the tests in this file
fn capture(label: &str, status: deliver::Status) -> deliver::Capture {
    deliver::Capture {
        path: PathBuf::from(format!("out/{label}.png")),
        artifact: artifact("ui", label, &format!("ui|{label}"), "hash"),
        status,
    }
}

// needed helper: fixture shared by the tests in this file
fn manifest(entries: Vec<deliver::Artifact>) -> deliver::Manifest {
    deliver::Manifest {
        crate_name: String::from("demo"),
        artifacts: entries
            .into_iter()
            .map(|entry| (entry.fingerprint.clone(), entry))
            .collect::<BTreeMap<_, _>>(),
    }
}

#[test]
fn test_usage() {
    assert_eq!(
        deliver::Artifact::slug("hover then press"),
        "hover_then_press"
    );
    assert_eq!(deliver::Artifact::slug("A-B.c"), "a_b_c");
    assert_eq!(deliver::Artifact::fingerprint_of("ui", "rest"), "ui|rest");
    assert_eq!(
        deliver::Artifact::hash_bytes(b"abc"),
        deliver::Artifact::hash_bytes(b"abc")
    );
    assert_ne!(
        deliver::Artifact::hash_bytes(b"abc"),
        deliver::Artifact::hash_bytes(b"abd")
    );
    assert_eq!(
        deliver::Artifact::hash_bytes(b"abc").len(),
        deliver::basic::constants::HASH_CHARS
    );
    assert!(capture("rest", deliver::Status::Changed).is_sendable());
    assert!(capture("rest", deliver::Status::New).is_sendable());
    assert!(!capture("rest", deliver::Status::Same).is_sendable());
}

#[test]
fn test_usage_status_distinguishes_new_changed_and_same() {
    let before = manifest(vec![artifact("ui", "rest", "a", "1")]);
    let same = artifact("ui", "rest", "a", "1");
    let changed = artifact("ui", "rest", "a", "2");
    let fresh = artifact("ui", "other", "b", "1");
    assert_eq!(same.status_in(None), deliver::Status::New);
    assert_eq!(same.status_in(Some(&before)), deliver::Status::Same);
    assert_eq!(changed.status_in(Some(&before)), deliver::Status::Changed);
    assert_eq!(fresh.status_in(Some(&before)), deliver::Status::New);
}

#[test]
fn test_usage_status_matches_by_scene_and_label_when_fingerprint_moved() {
    let before = manifest(vec![artifact("ui", "rest", "a", "1")]);
    let moved = artifact("ui", "rest", "a-renamed", "1");
    assert_eq!(moved.status_in(Some(&before)), deliver::Status::Same);
}

#[test]
fn test_usage_collect_labels_every_artifact_with_a_status() {
    let rendered = vec![preview::RenderedState {
        path: PathBuf::from("out/rest.png"),
        label: String::from("rest"),
        pixel_hash: String::from("h1"),
    }];
    let captures = deliver::Capture::collect("ui", scene::Kind::System, &rendered, None).unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures[0].status, deliver::Status::New);
    assert_eq!(captures[0].artifact.fingerprint, "ui|rest");
    let previous = manifest(vec![captures[0].artifact.clone()]);
    let again =
        deliver::Capture::collect("ui", scene::Kind::System, &rendered, Some(&previous)).unwrap();
    assert_eq!(again[0].status, deliver::Status::Same);
}

#[test]
fn test_usage_hash_file_reads_bytes_and_reports_missing_files() {
    let dir = tempfile::tempdir().unwrap();
    let png = dir.path().join("bytes.png");
    std::fs::write(&png, b"pixel-bytes").unwrap();
    assert_eq!(
        deliver::Artifact::hash_file(&png).unwrap(),
        deliver::Artifact::hash_bytes(b"pixel-bytes")
    );
    assert!(deliver::Artifact::hash_file(&dir.path().join("absent.png")).is_err());
}

#[test]
fn test_usage_manifest_round_trips_through_disk() {
    let dir = tempfile::tempdir().unwrap();
    assert!(deliver::Manifest::load(dir.path()).is_none());
    deliver::Manifest::write_all(dir.path(), "demo", &[capture("rest", deliver::Status::New)])
        .unwrap();
    let loaded = deliver::Manifest::load(dir.path()).unwrap();
    assert_eq!(loaded.crate_name, "demo");
    assert_eq!(loaded.artifacts.len(), 1);
    assert!(
        dir.path()
            .join(deliver::basic::constants::MANIFEST_FILE)
            .exists()
    );
    assert!(
        dir.path()
            .join(deliver::basic::constants::CHANGES_FILE)
            .exists()
    );
}

#[test]
fn test_usage_update_manifest_records_sent_and_drops_pending() {
    let dir = tempfile::tempdir().unwrap();
    deliver::Manifest::write_all(dir.path(), "demo", &[capture("rest", deliver::Status::New)])
        .unwrap();
    deliver::Manifest::update(
        dir.path(),
        "demo",
        &[&capture("hover", deliver::Status::New)],
        &[&capture("rest", deliver::Status::New)],
    )
    .unwrap();
    let loaded = deliver::Manifest::load(dir.path()).unwrap();
    assert_eq!(
        loaded.artifacts.len(),
        1,
        "pending captures leave the manifest"
    );
    assert!(loaded.artifacts.contains_key("ui|hover"));
}

#[test]
fn test_usage_changes_markdown_lists_only_what_moved() {
    let all = vec![
        capture("rest", deliver::Status::Same),
        capture("hover", deliver::Status::Changed),
        capture("press", deliver::Status::New),
    ];
    let text = deliver::Capture::changes_markdown(&all);
    assert!(text.contains("1 changed, 1 new, 1 unchanged"));
    assert!(text.contains("hover"));
    let quiet = deliver::Capture::changes_markdown(&[capture("rest", deliver::Status::Same)]);
    assert!(quiet.contains("No UI state changed"));
    assert!(deliver::Capture::changes_markdown(&[]).contains("No UI state changed"));
    let fresh = deliver::Capture::changes_markdown(&[capture("press", deliver::Status::New)]);
    assert!(fresh.contains("0 changed, 1 new"));
}
// no test_usage necessary
