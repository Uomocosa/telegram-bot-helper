#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::sync::OnceLock;

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

    fn record_artifact(dir: &std::path::Path, artifact: &deliver::Artifact) {
        let path = dir.join(artifact.file_name());
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&path, b"bytes").unwrap();
        deliver::SentPreviewHash::record(dir, artifact).unwrap();
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
            path: dir.path().join("sync_room_list__two_rooms.png"),
            label: String::from("two rooms"),
            pixel_hash: String::from("0000000000000000"),
        }];
        std::fs::write(&stale[0].path, b"stale").unwrap();
        let previous =
            deliver::Capture::collect("sync_room_list", scene::Kind::System, &stale, dir.path())
                .unwrap();
        record_artifact(dir.path(), &previous[0].artifact);
        std::fs::write(dir.path().join("sync_room_list__two_rooms.png"), b"stale").unwrap();
        let third = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(third.captures[0].status, deliver::Status::Changed);
        assert!(third.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_cache_accumulates_across_scenes() {
        let dir = tempfile::tempdir().unwrap();
        let two_rooms = vec![preview::RenderedState {
            path: dir.path().join("sync_room_list__two_rooms.png"),
            label: String::from("two rooms"),
            pixel_hash: String::from("0123456789abcdef"),
        }];
        std::fs::write(&two_rooms[0].path, b"bytes").unwrap();
        let first = deliver::Capture::collect(
            "sync_room_list",
            scene::Kind::System,
            &two_rooms,
            dir.path(),
        )
        .unwrap();
        record_artifact(dir.path(), &first[0].artifact);
        let other_rooms = vec![preview::RenderedState {
            path: dir.path().join("spawn_root__two_rooms.png"),
            label: String::from("two rooms"),
            pixel_hash: String::from("0123456789abcdef"),
        }];
        std::fs::write(&other_rooms[0].path, b"bytes").unwrap();
        let other =
            deliver::Capture::collect("spawn_root", scene::Kind::System, &other_rooms, dir.path())
                .unwrap();
        record_artifact(dir.path(), &other[0].artifact);
        let first_sent = deliver::SentPreviewHash::sent_path(dir.path(), &first[0].artifact);
        let other_sent = deliver::SentPreviewHash::sent_path(dir.path(), &other[0].artifact);
        assert!(first_sent.exists());
        assert!(other_sent.exists());
        assert_ne!(first_sent, other_sent);
    }

    #[test]
    fn test_usage_an_invalid_scene_is_reported_before_any_capture() {
        let dir = tempfile::tempdir().unwrap();
        let broken = Scene {
            states: Vec::new(),
            ..scene()
        };
        let result = broken.run(&config(dir.path()));
        assert!(matches!(result, Err(Error::Scene(_))));
        assert!(
            !dir.path()
                .join(deliver::basic::constants::CACHE_DIR)
                .exists()
                || !dir
                    .path()
                    .join(deliver::basic::constants::CACHE_DIR)
                    .join("sync_room_list.lock")
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
        record_artifact(dir.path(), &first.captures[0].artifact);
        let second = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(second.captures[0].status, deliver::Status::Same);
        assert!(!second.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_a_deleted_preview_is_pending_again() {
        let dir = tempfile::tempdir().unwrap();
        let first = scene().build_report(&config(dir.path())).unwrap();
        record_artifact(dir.path(), &first.captures[0].artifact);
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
    fn test_usage_an_unsent_preview_stays_pending_after_failed_send() {
        let dir = tempfile::tempdir().unwrap();
        let first = scene().build_report(&config(dir.path())).unwrap();
        record_artifact(dir.path(), &first.captures[0].artifact);
        std::fs::remove_file(&first.captures[0].path).unwrap();
        let second = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(second.captures[0].status, deliver::Status::New);
        let failing = config_with(dir.path(), fail_always);
        let report = Report {
            captures: vec![second.captures[0].clone()],
            clip: None,
        };
        let scene_obj = scene();
        assert!(scene_obj.deliver_report(&report, &failing).is_err());
        let third = scene().build_report(&config(dir.path())).unwrap();
        assert_eq!(
            third.captures[0].status,
            deliver::Status::New,
            "a re-rendered but unsent preview must stay pending on the next run"
        );
    }

    type RecordedAlbum = Vec<RecordedSend>;

    #[derive(Clone)]
    struct RecordedSend {
        caption: String,
        fingerprints: Vec<String>,
    }
    type RecordedMap = HashMap<PathBuf, RecordedAlbum>;

    static SENT_LOG_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

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

    fn fail_always(_items: &[&deliver::Capture], _caption: &str) -> Result<usize, Error> {
        Err(Error::Scene(String::from("injected failure")))
    }

    fn file_recording_send(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
        let _ = caption;
        let guard = SENT_LOG_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .map_err(|_| Error::Scene(String::from("lock poisoned")))?;
        let Some(dir) = items
            .first()
            .and_then(|capture| capture.path.parent())
            .map(std::path::Path::to_path_buf)
        else {
            return Ok(0);
        };
        let mut buffer = String::new();
        for capture in items {
            buffer.push_str(&capture.artifact.fingerprint);
            buffer.push('\n');
        }
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("test-sent.log"))
            .and_then(|mut file| {
                use std::io::Write;
                file.write_all(buffer.as_bytes())
            })
            .map_err(|_| Error::Scene(String::from("sent log unavailable")))?;
        drop(guard);
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
    fn test_usage_same_hash_sends_nothing_and_cache_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let capture = hand_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&capture.path, b"bytes").unwrap();
        deliver::SentPreviewHash::record(dir.path(), &capture.artifact).unwrap();
        let sent_path = deliver::SentPreviewHash::sent_path(dir.path(), &capture.artifact);
        let before = std::fs::read(&sent_path).unwrap();
        let same = hand_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            "0123456789abcdef",
            deliver::Status::Same,
        );
        std::fs::write(&same.path, b"bytes").unwrap();
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
        let sent = scene_obj.deliver_report(&report, &cfg).unwrap();
        assert_eq!(sent, 0);
        let empty = recorded_calls()
            .lock()
            .unwrap()
            .get(dir.path())
            .map_or(0, Vec::len);
        assert_eq!(empty, 0);
        let after = std::fs::read(&sent_path).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_renaming_scene_or_label_gives_new() {
        let dir = tempfile::tempdir().unwrap();
        let old = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&old.path, b"bytes").unwrap();
        deliver::SentPreviewHash::record(dir.path(), &old.artifact).unwrap();
        let renamed_scene = deliver::Artifact {
            scene: String::from("other"),
            label: String::from("one"),
            fingerprint: deliver::Artifact::fingerprint_of("other", "one"),
            pixel_hash: String::from("0123456789abcdef"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        std::fs::write(dir.path().join(renamed_scene.file_name()), b"bytes").unwrap();
        assert_eq!(
            renamed_scene.status_in_cache(dir.path()).unwrap(),
            deliver::Status::New
        );
        let renamed_label = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("two"),
            fingerprint: deliver::Artifact::fingerprint_of("ui", "two"),
            pixel_hash: String::from("0123456789abcdef"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        std::fs::write(dir.path().join(renamed_label.file_name()), b"bytes").unwrap();
        assert_eq!(
            renamed_label.status_in_cache(dir.path()).unwrap(),
            deliver::Status::New
        );
    }

    #[test]
    fn test_usage_one_changed_item_sends_whole_album_with_status_words() {
        let dir = tempfile::tempdir().unwrap();
        let old_rest = hand_capture(
            dir.path(),
            "ui",
            "rest",
            "0000000000000000",
            deliver::Status::New,
        );
        let old_hover = hand_capture(
            dir.path(),
            "ui",
            "hover",
            "0000000000000001",
            deliver::Status::New,
        );
        std::fs::write(&old_rest.path, b"bytes").unwrap();
        std::fs::write(&old_hover.path, b"bytes").unwrap();
        deliver::SentPreviewHash::record(dir.path(), &old_rest.artifact).unwrap();
        deliver::SentPreviewHash::record(dir.path(), &old_hover.artifact).unwrap();
        let captures = vec![
            hand_capture(
                dir.path(),
                "ui",
                "rest",
                "0000000000000000",
                deliver::Status::Same,
            ),
            hand_capture(
                dir.path(),
                "ui",
                "hover",
                "0000000000000002",
                deliver::Status::Changed,
            ),
            hand_capture(
                dir.path(),
                "ui",
                "press",
                "0000000000000003",
                deliver::Status::New,
            ),
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
        let sent = scene_obj.deliver_report(&report, &cfg).unwrap();
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
                "0123456789abcdef",
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
        assert!(scene_obj.deliver_report(&report, &failing).is_err());
        let retry = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &retry).unwrap();
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
    fn test_usage_invalid_sent_gives_error_sends_nothing_and_keeps_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let capture = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&capture.path, b"bytes").unwrap();
        let sent_path = deliver::SentPreviewHash::sent_path(dir.path(), &capture.artifact);
        std::fs::create_dir_all(sent_path.parent().unwrap()).unwrap();
        std::fs::write(&sent_path, b"bad-content").unwrap();
        let before = std::fs::read(&sent_path).unwrap();
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
        let result = scene_obj.deliver_report(&report, &cfg);
        assert!(matches!(result, Err(Error::SentCacheInvalid { .. })));
        assert!(recorded_calls().lock().unwrap().get(dir.path()).is_none());
        let after = std::fs::read(&sent_path).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn test_usage_two_reports_for_same_scene_delivered_one_after_other_send_once() {
        let dir = tempfile::tempdir().unwrap();
        let first = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
        let second = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
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
        let first_sent = scene_obj.deliver_report(&first_report, &cfg).unwrap();
        assert_eq!(first_sent, 1);
        let second_sent = scene_obj.deliver_report(&second_report, &cfg).unwrap();
        assert_eq!(second_sent, 0);
    }

    #[test]
    fn test_usage_sixteen_threads_delivering_different_scenes_send_each_once() {
        let dir = tempfile::tempdir().unwrap();
        for thread in 0..16 {
            let capture = hand_capture(
                dir.path(),
                &format!("parallel_{thread}"),
                "a",
                "0123456789abcdef",
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
                        let capture = hand_capture(
                            &path,
                            &name,
                            "a",
                            "0123456789abcdef",
                            deliver::Status::New,
                        );
                        let report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let cfg = config_with(&path, file_recording_send);
                        scene_for(&name).deliver_report(&report, &cfg).unwrap();
                    }
                });
            }
        });
        for thread in 0..16 {
            let name = format!("parallel_{thread}");
            let probe = hand_capture(&path, &name, "a", "0123456789abcdef", deliver::Status::New);
            let sent = deliver::SentPreviewHash::sent_path(&path, &probe.artifact);
            assert!(sent.exists(), "missing sent for {name}");
        }
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
        let first = hand_capture(
            dir.path(),
            "shared",
            "a",
            "0123456789abcdef",
            deliver::Status::New,
        );
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
                        let capture = hand_capture(
                            &path,
                            "shared",
                            "a",
                            "0123456789abcdef",
                            deliver::Status::New,
                        );
                        let report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let cfg = config_with(&path, file_recording_send);
                        scene_for("shared").deliver_report(&report, &cfg).unwrap();
                    }
                });
            }
        });
        let probe = hand_capture(
            &path,
            "shared",
            "a",
            "0123456789abcdef",
            deliver::Status::New,
        );
        assert!(deliver::SentPreviewHash::sent_path(&path, &probe.artifact).exists());
        let text = std::fs::read_to_string(dir.path().join("test-sent.log")).unwrap();
        assert_eq!(text.lines().filter(|line| !line.is_empty()).count(), 1);
    }

    #[test]
    fn test_usage_changed_item_in_failed_chunk_stays_changed_on_retry() {
        let dir = tempfile::tempdir().unwrap();
        let old = hand_capture(
            dir.path(),
            "ui",
            "state 10",
            "0000000000000000",
            deliver::Status::New,
        );
        std::fs::write(&old.path, b"bytes").unwrap();
        deliver::SentPreviewHash::record(dir.path(), &old.artifact).unwrap();
        let mut captures = Vec::new();
        for index in 0..10 {
            captures.push(hand_capture(
                dir.path(),
                "ui",
                &format!("state {index}"),
                "0123456789abcdef",
                deliver::Status::New,
            ));
        }
        captures.push(hand_capture(
            dir.path(),
            "ui",
            "state 10",
            "1111111111111111",
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
        assert!(scene_obj.deliver_report(&report, &failing).is_err());
        let retry = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&report, &retry).unwrap();
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
    #[ignore = "worker for process tests"]
    fn preview_cache_worker_delivers_one_scene() {
        let dir = std::env::var("PREVIEW_WORKER_OUT_DIR").unwrap();
        let scene_name = std::env::var("PREVIEW_WORKER_SCENE").unwrap();
        let iterations: usize = std::env::var("PREVIEW_WORKER_ITERATIONS")
            .unwrap()
            .parse()
            .unwrap();
        let dir = PathBuf::from(dir);
        let probe = hand_capture(
            &dir,
            &scene_name,
            "a",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&probe.path, b"bytes").unwrap();
        for _ in 0..iterations {
            let capture = hand_capture(
                &dir,
                &scene_name,
                "a",
                "0123456789abcdef",
                deliver::Status::New,
            );
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
                .deliver_report(&report, &config)
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
                "0123456789abcdef",
                deliver::Status::New,
            );
            std::fs::write(&probe.path, b"bytes").unwrap();
        }
        let exe = std::env::current_exe().unwrap();
        let mut children = Vec::new();
        for thread in 0..8 {
            let child = std::process::Command::new(&exe)
                .arg("run_tests::tests::preview_cache_worker_delivers_one_scene")
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
        for _ in 0..200 {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        for mut child in children {
            assert!(child.wait().unwrap().success());
        }
        for thread in 0..8 {
            let probe = hand_capture(
                dir.path(),
                &format!("dproc_{thread}"),
                "a",
                "0123456789abcdef",
                deliver::Status::New,
            );
            assert!(
                deliver::SentPreviewHash::sent_path(dir.path(), &probe.artifact).exists(),
                "missing sent for dproc_{thread}"
            );
        }
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

    #[test]
    fn test_usage_dry_run_writes_log_but_no_sent() {
        let dir = tempfile::tempdir().unwrap();
        let capture = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&capture.path, b"bytes").unwrap();
        let sent_probe = capture.artifact.clone();
        let report = Report {
            captures: vec![capture],
            clip: None,
        };
        let scene_obj = scene_for("ui");
        let cfg = config_with(dir.path(), recording_send);
        unsafe { std::env::set_var("LELE_PREVIEW_DRY_RUN", "1") };
        let sent = scene_obj.deliver_report(&report, &cfg).unwrap();
        unsafe { std::env::remove_var("LELE_PREVIEW_DRY_RUN") };
        assert_eq!(sent, 0);
        let sent_path = deliver::SentPreviewHash::sent_path(dir.path(), &sent_probe);
        assert!(!sent_path.exists());
        let log = dir.path().join(deliver::basic::constants::LOG_FILE);
        assert!(log.exists());
    }

    #[test]
    fn test_usage_dry_run_with_missing_preview_leaves_sent_in_place() {
        let dir = tempfile::tempdir().unwrap();
        let first = scene().build_report(&config(dir.path())).unwrap();
        record_artifact(dir.path(), &first.captures[0].artifact);
        let sent_path =
            deliver::SentPreviewHash::sent_path(dir.path(), &first.captures[0].artifact);
        assert!(sent_path.exists());
        std::fs::remove_file(&first.captures[0].path).unwrap();
        unsafe { std::env::set_var("LELE_PREVIEW_DRY_RUN", "1") };
        let _ = scene().build_report(&config(dir.path())).unwrap();
        unsafe { std::env::remove_var("LELE_PREVIEW_DRY_RUN") };
        assert!(sent_path.exists());
    }

    #[test]
    fn test_usage_log_cache_reports_hit_missing_and_frames() {
        let dir = tempfile::tempdir().unwrap();
        let capture = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::New,
        );
        std::fs::write(&capture.path, b"bytes").unwrap();
        let report = Report {
            captures: vec![capture],
            clip: None,
        };
        let scene_obj = scene_for("ui");
        let cfg = config_with(dir.path(), recording_send);
        scene_obj.deliver_report(&report, &cfg).unwrap();
        let log_text =
            std::fs::read_to_string(dir.path().join(deliver::basic::constants::LOG_FILE)).unwrap();
        assert!(
            log_text.contains("\"cache\":\"missing\"")
                || log_text.contains("\"cache\": \"missing\"")
                || log_text.contains("missing")
        );
        let second = hand_capture(
            dir.path(),
            "ui",
            "one",
            "0123456789abcdef",
            deliver::Status::Same,
        );
        std::fs::write(&second.path, b"bytes").unwrap();
        let second_report = Report {
            captures: vec![second],
            clip: None,
        };
        scene_obj.deliver_report(&second_report, &cfg).unwrap();
        let log_again =
            std::fs::read_to_string(dir.path().join(deliver::basic::constants::LOG_FILE)).unwrap();
        assert!(log_again.contains("hit"));
    }
    // no test_usage necessary
}
