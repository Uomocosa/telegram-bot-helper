use crate::run;
use std::path::PathBuf;

use bevy::ecs::world::World;
use bevy::prelude::App;

use crate::Error;
use crate::deliver;
use crate::preview;
use crate::scene;
use crate::scene::Scene;
use crate::scene::basic::structs::State;

// needed helper: fixture shared by the tests in this file
fn noop(_world: &mut World) {}
// needed helper: fixture shared by the tests in this file
fn noop_build(_app: &mut App) {}

// needed helper: fixture shared by the tests in this file
fn scene() -> Scene {
    Scene {
        name: String::from("sync_room_list"),
        kind: scene::Kind::System,
        build: noop_build,
        states: vec![State {
            label: String::from("two rooms"),
            apply: noop,
        }],
        timeline: None,
    }
}

// needed helper: fixture shared by the tests in this file
fn config(dir: &std::path::Path) -> preview::Config {
    preview::Config {
        out_dir: dir.to_path_buf(),
        warmup_frames: 5,
        max_capture_frames: 240,
        min_distinct_colors: 1,
        ..preview::Config::default()
    }
}

#[test]
fn test_usage() {
    let dir = tempfile::tempdir().unwrap();
    let report = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(report.captures.len(), 1);
    assert_eq!(report.captures[0].status, deliver::Status::New);
    assert!(report.clip.is_none());
    assert!(report.captures[0].path.exists());
    assert!(report.to_string().contains("two rooms"));
}

#[test]
fn test_usage_second_identical_run_hashes_identically() {
    let dir = tempfile::tempdir().unwrap();
    let report = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    let second = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    let first_hash = &report.captures[0].artifact.pixel_hash;
    assert_eq!(
        &second.captures[0].artifact.pixel_hash, first_hash,
        "two identical runs must hash identically, otherwise the change gate is noise"
    );
}

#[test]
fn test_usage_differing_pixels_report_changed() {
    let dir = tempfile::tempdir().unwrap();
    let stale = vec![preview::RenderedState {
        path: PathBuf::from("sync_room_list__two_rooms.png"),
        label: String::from("two rooms"),
        pixel_hash: String::from("stale"),
    }];
    let previous = deliver::collect("sync_room_list", scene::Kind::System, &stale, None).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&previous[0]], &[]).unwrap();
    std::fs::write(dir.path().join("sync_room_list__two_rooms.png"), b"stale").unwrap();
    let third = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(third.captures[0].status, deliver::Status::Changed);
    assert!(deliver::sendable::sendable(third.captures[0].status));
}

#[test]
fn test_usage_manifest_accumulates_across_scenes() {
    let dir = tempfile::tempdir().unwrap();
    let two_rooms = vec![preview::RenderedState {
        path: PathBuf::from("sync_room_list__two_rooms.png"),
        label: String::from("two rooms"),
        pixel_hash: String::from("hash"),
    }];
    let first = deliver::collect("sync_room_list", scene::Kind::System, &two_rooms, None).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&first[0]], &[]).unwrap();
    let other = deliver::collect("spawn_root", scene::Kind::System, &two_rooms, None).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&other[0]], &[]).unwrap();
    let merged = deliver::load_previous::load_previous(dir.path()).unwrap();
    assert_eq!(merged.artifacts.len(), 2);
    assert!(merged.artifacts.contains_key("sync_room_list|two rooms"));
    assert!(merged.artifacts.contains_key("spawn_root|two rooms"));
}

#[test]
fn test_usage_an_invalid_scene_is_reported_before_any_capture() {
    let dir = tempfile::tempdir().unwrap();
    let broken = Scene {
        states: Vec::new(),
        ..scene()
    };
    let result = run(&broken, &config(dir.path()), "demo");
    assert!(matches!(result, Err(Error::Scene(_))));
    assert!(
        !dir.path()
            .join(deliver::basic::constants::MANIFEST_FILE)
            .exists()
    );
}

#[test]
fn test_usage_artifacts_are_named_after_the_scene_and_state() {
    let dir = tempfile::tempdir().unwrap();
    let report = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    let path: PathBuf = report.captures[0].path.clone();
    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    assert_eq!(name, "sync_room_list__two_rooms.png");
}

#[test]
fn test_usage_a_recorded_preview_stays_same() {
    let dir = tempfile::tempdir().unwrap();
    let first = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
    let second = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(second.captures[0].status, deliver::Status::Same);
    assert!(!deliver::sendable::sendable(second.captures[0].status));
}

#[test]
fn test_usage_a_deleted_preview_is_pending_again() {
    let dir = tempfile::tempdir().unwrap();
    let first = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
    std::fs::remove_file(&first.captures[0].path).unwrap();
    let second = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(second.captures[0].status, deliver::Status::New);
    assert!(deliver::sendable::sendable(second.captures[0].status));
}

#[test]
fn test_usage_an_unrecorded_preview_stays_pending() {
    let dir = tempfile::tempdir().unwrap();
    run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    let second = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(
        second.captures[0].status,
        deliver::Status::New,
        "a preview never confirmed as sent must keep being offered"
    );
}

#[test]
fn test_usage_an_unsent_preview_is_dropped_from_the_manifest() {
    let dir = tempfile::tempdir().unwrap();
    let first = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
    std::fs::remove_file(&first.captures[0].path).unwrap();
    let second = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    deliver::update_manifest(dir.path(), "demo", &[], &[&second.captures[0]]).unwrap();
    let third = run::build_report::build_report(&scene(), &config(dir.path())).unwrap();
    assert_eq!(
        third.captures[0].status,
        deliver::Status::New,
        "a re-rendered but unsent preview must stay pending on the next run"
    );
}
// no test_usage necessary
