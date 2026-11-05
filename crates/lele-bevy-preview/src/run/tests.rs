#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use bevy::ecs::world::World;
    use bevy::prelude::App;

    use crate::Error;
    use crate::Report;
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use crate::scene::Scene;
    use crate::scene::basic::structs::State;

    fn noop(_world: &mut World) {}
    fn noop_build(_app: &mut App) {}

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
        let report = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(report.captures.len(), 1);
        assert_eq!(report.captures[0].status, deliver::Status::New);
        assert!(report.clip.is_none());
        assert!(report.captures[0].path.exists());
        assert!(report.to_string().contains("two rooms"));
    }

    #[test]
    fn test_usage_second_identical_run_hashes_identically() {
        let dir = tempfile::tempdir().unwrap();
        let report = scene().build_report(&config(dir.path())).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
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
        let previous =
            deliver::Capture::collect("sync_room_list", scene::Kind::System, &stale, None).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&previous[0]], &[]).unwrap();
        std::fs::write(dir.path().join("sync_room_list__two_rooms.png"), b"stale").unwrap();
        let third = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(third.captures[0].status, deliver::Status::Changed);
        assert!(third.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_manifest_accumulates_across_scenes() {
        let dir = tempfile::tempdir().unwrap();
        let two_rooms = vec![preview::RenderedState {
            path: PathBuf::from("sync_room_list__two_rooms.png"),
            label: String::from("two rooms"),
            pixel_hash: String::from("hash"),
        }];
        let first =
            deliver::Capture::collect("sync_room_list", scene::Kind::System, &two_rooms, None)
                .unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&first[0]], &[]).unwrap();
        let other =
            deliver::Capture::collect("spawn_root", scene::Kind::System, &two_rooms, None).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&other[0]], &[]).unwrap();
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
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
        let result = broken.run(&config(dir.path()), "demo");
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
        let report = scene().build_report(&config(dir.path())).unwrap();
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
        let first = scene().build_report(&config(dir.path())).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(second.captures[0].status, deliver::Status::Same);
        assert!(!second.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_a_deleted_preview_is_pending_again() {
        let dir = tempfile::tempdir().unwrap();
        let first = scene().build_report(&config(dir.path())).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
        std::fs::remove_file(&first.captures[0].path).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(second.captures[0].status, deliver::Status::New);
        assert!(second.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_an_unrecorded_preview_stays_pending() {
        let dir = tempfile::tempdir().unwrap();
        scene().build_report(&config(dir.path())).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(
            second.captures[0].status,
            deliver::Status::New,
            "a preview never confirmed as sent must keep being offered"
        );
    }

    #[test]
    fn test_usage_an_unsent_preview_is_dropped_from_the_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let first = scene().build_report(&config(dir.path())).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&first.captures[0]], &[]).unwrap();
        std::fs::remove_file(&first.captures[0].path).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[], &[&second.captures[0]]).unwrap();
        let third = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(
            third.captures[0].status,
            deliver::Status::New,
            "a re-rendered but unsent preview must stay pending on the next run"
        );
    }

    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    type RecordedAlbum = Vec<RecordedSend>;

    #[derive(Clone)]
    struct RecordedSend {
        caption: String,
        fingerprints: Vec<String>,
    }
    type RecordedMap = HashMap<PathBuf, RecordedAlbum>;

    fn recorded_calls() -> &'static Mutex<RecordedMap> {
        static MAP: OnceLock<Mutex<RecordedMap>> = OnceLock::new();
        MAP.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn recording_send(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
        let dir = items
            .first()
            .and_then(|capture| capture.path.parent())
            .map(std::path::Path::to_path_buf)
            .unwrap_or_default();
        let fingerprints: Vec<String> = items
            .iter()
            .map(|capture| capture.artifact.fingerprint.clone())
            .collect();
        recorded_calls()
            .lock()
            .map_err(|_| Error::Scene(String::from("lock poisoned")))?
            .entry(dir)
            .or_default()
            .push(RecordedSend {
                caption: caption.to_string(),
                fingerprints,
            });
        Ok(items.len())
    }

    fn fail_on_single_chunk(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
        let _ = caption;
        if items.len() < 10 {
            return Err(Error::Scene(String::from("injected failure")));
        }
        Ok(items.len())
    }

    fn file_recording_send(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
        use std::io::Write;
        let _ = caption;
        let Some(dir) = items
            .first()
            .and_then(|capture| capture.path.parent())
            .map(std::path::Path::to_path_buf)
        else {
            return Ok(0);
        };
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("test-sent.log"))
            .map_err(|_| Error::Scene(String::from("sent log unavailable")))?;
        for capture in items {
            writeln!(file, "{}", capture.artifact.fingerprint)
                .map_err(|_| Error::Scene(String::from("sent log unavailable")))?;
        }
        Ok(items.len())
    }

    fn scene_for(name: &str) -> Scene {
        Scene {
            name: name.to_string(),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        }
    }

    fn hand_capture(
        dir: &std::path::Path,
        scene_name: &str,
        label: &str,
        hash: &str,
        status: deliver::Status,
    ) -> deliver::Capture {
        deliver::Capture {
            path: dir.join(format!(
                "{}__{}.png",
                scene_name,
                deliver::Artifact::slug(label)
            )),
            artifact: deliver::Artifact {
                scene: scene_name.to_string(),
                label: label.to_string(),
                fingerprint: deliver::Artifact::fingerprint_of(scene_name, label),
                pixel_hash: hash.to_string(),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status,
        }
    }

    fn config_with(
        dir: &std::path::Path,
        send: fn(&[&deliver::Capture], &str) -> Result<usize, Error>,
    ) -> preview::Config {
        preview::Config {
            out_dir: dir.to_path_buf(),
            send_preview_group: send,
            ..preview::Config::default()
        }
    }

    #[test]
    fn test_usage_same_hash_sends_nothing_and_manifest_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let capture = hand_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            "hash",
            deliver::Status::New,
        );
        deliver::Manifest::update(dir.path(), "demo", &[&capture], &[]).unwrap();
        std::fs::write(&capture.path, b"bytes").unwrap();
        let before =
            std::fs::read(dir.path().join(deliver::basic::constants::MANIFEST_FILE)).unwrap();
        let same = hand_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            "hash",
            deliver::Status::Same,
        );
        let report = Report {
            captures: vec![same],
            clip: None,
        };
        let scene_obj = Scene {
            name: String::from("sync_room_list"),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        };
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &cfg, "demo").unwrap();
        assert_eq!(sent, 0);
        let empty = recorded_calls()
            .lock()
            .unwrap()
            .get(dir.path())
            .map_or(0, Vec::len);
        assert_eq!(empty, 0);
        let after =
            std::fs::read(dir.path().join(deliver::basic::constants::MANIFEST_FILE)).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_renaming_scene_or_label_gives_new() {
        let dir = tempfile::tempdir().unwrap();
        let old = hand_capture(dir.path(), "ui", "one", "h", deliver::Status::New);
        deliver::Manifest::update(dir.path(), "demo", &[&old], &[]).unwrap();
        let previous = deliver::Manifest::load(dir.path()).unwrap();
        let renamed_scene = deliver::Artifact {
            scene: String::from("other"),
            label: String::from("one"),
            fingerprint: deliver::Artifact::fingerprint_of("other", "one"),
            pixel_hash: String::from("h"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        assert_eq!(
            renamed_scene.status_in(previous.as_ref()),
            deliver::Status::New
        );
        let renamed_label = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("two"),
            fingerprint: deliver::Artifact::fingerprint_of("ui", "two"),
            pixel_hash: String::from("h"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        assert_eq!(
            renamed_label.status_in(previous.as_ref()),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_one_changed_item_sends_whole_album_with_status_words() {
        let dir = tempfile::tempdir().unwrap();
        let old_rest = hand_capture(dir.path(), "ui", "rest", "h", deliver::Status::New);
        let old_hover = hand_capture(dir.path(), "ui", "hover", "old", deliver::Status::New);
        std::fs::write(&old_rest.path, b"bytes").unwrap();
        std::fs::write(&old_hover.path, b"bytes").unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&old_rest, &old_hover], &[]).unwrap();
        let captures = vec![
            hand_capture(dir.path(), "ui", "rest", "h", deliver::Status::Same),
            hand_capture(dir.path(), "ui", "hover", "h2", deliver::Status::Changed),
            hand_capture(dir.path(), "ui", "press", "h3", deliver::Status::New),
        ];
        for capture in &captures {
            std::fs::write(&capture.path, b"bytes").unwrap();
        }
        let report = Report {
            captures,
            clip: None,
        };
        let scene_obj = Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        };
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &cfg, "demo").unwrap();
        assert_eq!(sent, 3);
        let entries = recorded_calls()
            .lock()
            .unwrap()
            .get(dir.path())
            .cloned()
            .unwrap_or_default();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].fingerprints.len(), 3);
        assert!(entries[0].caption.contains("changed"));
        assert!(entries[0].caption.contains("new"));
        assert!(entries[0].caption.contains("same"));
    }

    #[test]
    fn test_usage_failed_send_then_retry_does_not_resend_succeeded_chunks() {
        let dir = tempfile::tempdir().unwrap();
        let mut captures = Vec::new();
        for index in 0..11 {
            captures.push(hand_capture(
                dir.path(),
                "ui",
                &format!("state {index}"),
                "h",
                deliver::Status::New,
            ));
        }
        for capture in &captures {
            std::fs::write(&capture.path, b"bytes").unwrap();
        }
        let report = Report {
            captures,
            clip: None,
        };
        let scene_obj = Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        };
        let failing = config_with(dir.path(), fail_on_single_chunk);
        assert!(scene_obj.deliver_report(&report, &failing, "demo").is_err());
        let retry = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &retry, "demo").unwrap();
        assert_eq!(sent, 1);
        let entries = recorded_calls()
            .lock()
            .unwrap()
            .get(dir.path())
            .cloned()
            .unwrap_or_default();
        let total: usize = entries.iter().map(|entry| entry.fingerprints.len()).sum();
        assert_eq!(total, 1);
    }

    #[test]
    fn test_usage_corrupt_manifest_gives_error_sends_nothing_and_keeps_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let manifest_path = dir.path().join(deliver::basic::constants::MANIFEST_FILE);
        std::fs::write(&manifest_path, b"{ not json").unwrap();
        let before = std::fs::read(&manifest_path).unwrap();
        let capture = hand_capture(dir.path(), "ui", "one", "h", deliver::Status::New);
        let report = Report {
            captures: vec![capture],
            clip: None,
        };
        let scene_obj = Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        };
        let cfg = config_with(dir.path(), recording_send);
        assert!(scene_obj.deliver_report(&report, &cfg, "demo").is_err());
        assert!(recorded_calls().lock().unwrap().get(dir.path()).is_none());
        let after = std::fs::read(&manifest_path).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_two_reports_for_same_scene_delivered_one_after_other_send_once() {
        let dir = tempfile::tempdir().unwrap();
        let first = hand_capture(dir.path(), "ui", "one", "h", deliver::Status::New);
        let second = hand_capture(dir.path(), "ui", "one", "h", deliver::Status::New);
        std::fs::write(&first.path, b"bytes").unwrap();
        let first_report = Report {
            captures: vec![first],
            clip: None,
        };
        let second_report = Report {
            captures: vec![second],
            clip: None,
        };
        let scene_obj = Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: noop_build,
            states: Vec::new(),
            timeline: None,
        };
        let cfg = config_with(dir.path(), recording_send);
        let first_sent = scene_obj
            .deliver_report(&first_report, &cfg, "demo")
            .unwrap();
        assert_eq!(first_sent, 1);
        let second_sent = scene_obj
            .deliver_report(&second_report, &cfg, "demo")
            .unwrap();
        assert_eq!(second_sent, 0);
    }

    #[test]
    fn test_usage_sixteen_threads_updating_different_scenes_lose_no_entries() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_path_buf();
        std::thread::scope(|scope| {
            for thread in 0..16 {
                let path = path.clone();
                scope.spawn(move || {
                    for _ in 0..50 {
                        let capture = deliver::Capture {
                            path: path.join(format!("scene_{thread}__a.png")),
                            artifact: deliver::Artifact {
                                scene: format!("scene_{thread}"),
                                label: String::from("a"),
                                fingerprint: format!("scene_{thread}|a"),
                                pixel_hash: String::from("h"),
                                media: deliver::Media::Png,
                                kind: scene::Kind::System,
                            },
                            status: deliver::Status::New,
                        };
                        deliver::Manifest::update(&path, "demo", &[&capture], &[]).unwrap();
                    }
                });
            }
        });
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(merged.artifacts.len(), 16);
    }

    #[test]
    #[ignore = "worker for process tests"]
    fn preview_manifest_worker_updates_one_scene() {
        let dir = std::env::var("PREVIEW_WORKER_OUT_DIR").unwrap();
        let scene_name = std::env::var("PREVIEW_WORKER_SCENE").unwrap();
        let iterations: usize = std::env::var("PREVIEW_WORKER_ITERATIONS")
            .unwrap()
            .parse()
            .unwrap();
        let dir = PathBuf::from(dir);
        for _ in 0..iterations {
            let capture = deliver::Capture {
                path: dir.join(format!("{scene_name}__a.png")),
                artifact: deliver::Artifact {
                    scene: scene_name.clone(),
                    label: String::from("a"),
                    fingerprint: format!("{scene_name}|a"),
                    pixel_hash: String::from("h"),
                    media: deliver::Media::Png,
                    kind: scene::Kind::System,
                },
                status: deliver::Status::New,
            };
            deliver::Manifest::update(&dir, "demo", &[&capture], &[]).unwrap();
        }
    }

    #[test]
    fn test_usage_real_processes_updating_different_scenes_lose_no_entries() {
        let dir = tempfile::tempdir().unwrap();
        let exe = std::env::current_exe().unwrap();
        let mut children = Vec::new();
        for thread in 0..8 {
            let child = std::process::Command::new(&exe)
                .arg("run_tests::tests::preview_manifest_worker_updates_one_scene")
                .arg("--exact")
                .arg("--ignored")
                .arg("--nocapture")
                .env("PREVIEW_WORKER_OUT_DIR", dir.path())
                .env("PREVIEW_WORKER_SCENE", format!("proc_{thread}"))
                .env("PREVIEW_WORKER_ITERATIONS", "30")
                .env("TELEGRAM_BOT_TOKEN", "")
                .env("TELEGRAM_CHAT_ID", "")
                .spawn()
                .unwrap();
            children.push(child);
        }
        let manifest_path = dir.path().join(deliver::basic::constants::MANIFEST_FILE);
        for _ in 0..200 {
            if let Ok(text) = std::fs::read_to_string(&manifest_path) {
                assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        for mut child in children {
            assert!(child.wait().unwrap().success());
        }
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(merged.artifacts.len(), 8);
    }

    #[test]
    fn test_usage_sixteen_threads_delivering_different_scenes_send_each_once() {
        let dir = tempfile::tempdir().unwrap();
        for thread in 0..16 {
            let capture = hand_capture(
                dir.path(),
                &format!("parallel_{thread}"),
                "a",
                "h",
                deliver::Status::New,
            );
            std::fs::write(&capture.path, b"bytes").unwrap();
        }
        let path = dir.path().to_path_buf();
        std::thread::scope(|scope| {
            for thread in 0..16 {
                let path = path.clone();
                scope.spawn(move || {
                    let name = format!("parallel_{thread}");
                    for _ in 0..20 {
                        let capture = hand_capture(&path, &name, "a", "h", deliver::Status::New);
                        let report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let cfg = config_with(&path, file_recording_send);
                        scene_for(&name)
                            .deliver_report(&report, &cfg, "demo")
                            .unwrap();
                    }
                });
            }
        });
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(merged.artifacts.len(), 16);
        let text = std::fs::read_to_string(dir.path().join("test-sent.log")).unwrap();
        let mut counts: HashMap<String, usize> = HashMap::new();
        for line in text.lines().filter(|line| !line.is_empty()) {
            *counts.entry(line.to_string()).or_default() += 1;
        }
        assert_eq!(counts.len(), 16);
        for count in counts.values() {
            assert_eq!(*count, 1);
        }
    }

    #[test]
    fn test_usage_sixteen_threads_delivering_same_scene_send_exactly_once() {
        let dir = tempfile::tempdir().unwrap();
        let first = hand_capture(dir.path(), "shared", "a", "h", deliver::Status::New);
        std::fs::write(&first.path, b"bytes").unwrap();
        let path = dir.path().to_path_buf();
        let barrier = std::sync::Barrier::new(16);
        std::thread::scope(|scope| {
            for _ in 0..16 {
                let path = path.clone();
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    for _ in 0..20 {
                        let capture = hand_capture(&path, "shared", "a", "h", deliver::Status::New);
                        let report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let cfg = config_with(&path, file_recording_send);
                        scene_for("shared")
                            .deliver_report(&report, &cfg, "demo")
                            .unwrap();
                    }
                });
            }
        });
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(merged.artifacts.len(), 1);
        let text = std::fs::read_to_string(dir.path().join("test-sent.log")).unwrap();
        assert_eq!(text.lines().filter(|line| !line.is_empty()).count(), 1);
    }

    #[test]
    fn test_usage_changed_item_in_failed_chunk_stays_changed_on_retry() {
        let dir = tempfile::tempdir().unwrap();
        let old = hand_capture(dir.path(), "ui", "state 10", "old", deliver::Status::New);
        std::fs::write(&old.path, b"bytes").unwrap();
        deliver::Manifest::update(dir.path(), "demo", &[&old], &[]).unwrap();
        let mut captures = Vec::new();
        for index in 0..10 {
            captures.push(hand_capture(
                dir.path(),
                "ui",
                &format!("state {index}"),
                "h",
                deliver::Status::New,
            ));
        }
        captures.push(hand_capture(
            dir.path(),
            "ui",
            "state 10",
            "h2",
            deliver::Status::Changed,
        ));
        for capture in &captures {
            std::fs::write(&capture.path, b"bytes").unwrap();
        }
        let report = Report {
            captures,
            clip: None,
        };
        let scene_obj = scene_for("ui");
        let failing = config_with(dir.path(), fail_on_single_chunk);
        assert!(scene_obj.deliver_report(&report, &failing, "demo").is_err());
        let retry = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &retry, "demo").unwrap();
        assert_eq!(sent, 1);
        let entries = recorded_calls()
            .lock()
            .unwrap()
            .get(dir.path())
            .cloned()
            .unwrap_or_default();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].caption.contains("changed"));
        assert!(!entries[0].caption.contains("new"));
    }

    #[test]
    #[ignore = "worker for process delivery tests"]
    fn preview_deliver_worker_delivers_one_scene() {
        let dir = std::env::var("PREVIEW_WORKER_OUT_DIR").unwrap();
        let scene_name = std::env::var("PREVIEW_WORKER_SCENE").unwrap();
        let iterations: usize = std::env::var("PREVIEW_WORKER_ITERATIONS")
            .unwrap()
            .parse()
            .unwrap();
        let dir = PathBuf::from(dir);
        let probe = hand_capture(&dir, &scene_name, "a", "h", deliver::Status::New);
        std::fs::write(&probe.path, b"bytes").unwrap();
        for _ in 0..iterations {
            let capture = hand_capture(&dir, &scene_name, "a", "h", deliver::Status::New);
            let report = Report {
                captures: vec![capture],
                clip: None,
            };
            let config = preview::Config {
                out_dir: dir.clone(),
                send_preview_group: file_recording_send,
                ..preview::Config::default()
            };
            scene_for(&scene_name)
                .deliver_report(&report, &config, "demo")
                .unwrap();
        }
    }

    #[test]
    fn test_usage_real_processes_delivering_different_scenes_send_each_once() {
        let dir = tempfile::tempdir().unwrap();
        for thread in 0..8 {
            let probe = hand_capture(
                dir.path(),
                &format!("dproc_{thread}"),
                "a",
                "h",
                deliver::Status::New,
            );
            std::fs::write(&probe.path, b"bytes").unwrap();
        }
        let exe = std::env::current_exe().unwrap();
        let mut children = Vec::new();
        for thread in 0..8 {
            let child = std::process::Command::new(&exe)
                .arg("run_tests::tests::preview_deliver_worker_delivers_one_scene")
                .arg("--exact")
                .arg("--ignored")
                .arg("--nocapture")
                .env("PREVIEW_WORKER_OUT_DIR", dir.path())
                .env("PREVIEW_WORKER_SCENE", format!("dproc_{thread}"))
                .env("PREVIEW_WORKER_ITERATIONS", "20")
                .env("TELEGRAM_BOT_TOKEN", "")
                .env("TELEGRAM_CHAT_ID", "")
                .spawn()
                .unwrap();
            children.push(child);
        }
        let manifest_path = dir.path().join(deliver::basic::constants::MANIFEST_FILE);
        for _ in 0..200 {
            if let Ok(text) = std::fs::read_to_string(&manifest_path) {
                assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        for mut child in children {
            assert!(child.wait().unwrap().success());
        }
        let merged = deliver::Manifest::load(dir.path()).unwrap().unwrap();
        assert_eq!(merged.artifacts.len(), 8);
        let text = std::fs::read_to_string(dir.path().join("test-sent.log")).unwrap();
        let mut counts: HashMap<String, usize> = HashMap::new();
        for line in text.lines().filter(|line| !line.is_empty()) {
            *counts.entry(line.to_string()).or_default() += 1;
        }
        assert_eq!(counts.len(), 8);
        for count in counts.values() {
            assert_eq!(*count, 1);
        }
    }
    // no test_usage necessary
}
