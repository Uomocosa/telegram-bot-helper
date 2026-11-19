#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Error;
    use crate::Report;
    use crate::basic;
    use crate::deliver;
    use crate::preview;
    use crate::scene;
    use crate::scene::Scene;
    use crate::testing::config_with;
    use crate::testing::fail_always;
    use crate::testing::fail_on_single_chunk;
    use crate::testing::fast_config;
    use crate::testing::file_recording_send;
    use crate::testing::no_credentials;
    use crate::testing::no_staging_left;
    use crate::testing::recording_send;
    use crate::testing::scene_with;
    use crate::testing::solid_png;
    use crate::testing::staging_capture;
    use crate::testing::write_delivered;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let report = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        assert_eq!(report.captures.len(), 1);
        assert_eq!(report.captures[0].status, deliver::Status::New);
        assert!(report.clip.is_none());
        assert!(report.captures[0].path.exists());
        assert!(
            report.captures[0]
                .path
                .to_string_lossy()
                .contains("_staging")
        );
        assert!(!dir.path().join("sync_room_list__two_rooms.png").exists());
    }

    #[test]
    fn test_usage_second_identical_run_hashes_identically() {
        let dir = tempfile::tempdir().unwrap();
        let report = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        let second = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        let first_hash = &report.captures[0].artifact.pixel_hash;
        assert_eq!(
            &second.captures[0].artifact.pixel_hash, first_hash,
            "two identical runs must hash identically, otherwise the change gate is noise"
        );
    }

    #[test]
    fn test_usage_differing_pixels_report_changed() {
        let dir = tempfile::tempdir().unwrap();
        write_delivered(dir.path(), "sync_room_list", "two rooms", 1, 2, 3);
        let third = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        assert_eq!(third.captures[0].status, deliver::Status::Changed);
        assert!(third.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_an_invalid_scene_is_reported_before_any_capture() {
        let dir = tempfile::tempdir().unwrap();
        let broken = Scene {
            states: Vec::new(),
            ..scene_with("sync_room_list", &["two rooms"], None)
        };
        let result = broken.run(&fast_config(dir.path()));
        assert!(matches!(result, Err(Error::Scene(_))));
        assert!(
            !dir.path()
                .join(basic::constants::LOCKS_DIR)
                .join("sync_room_list.lock")
                .exists()
        );
    }

    #[test]
    fn test_usage_artifacts_are_named_after_the_scene_and_state() {
        let dir = tempfile::tempdir().unwrap();
        let report = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        let path: PathBuf = report.captures[0].path.clone();
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        assert_eq!(name, "sync_room_list__two_rooms.png");
    }

    #[test]
    fn test_usage_a_deleted_preview_is_pending_again() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "sync_room_list", "two rooms", 10, 20, 30);
        let _ = delivered;
        std::fs::remove_file(dir.path().join("sync_room_list__two_rooms.png")).unwrap();
        let second = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        assert_eq!(second.captures[0].status, deliver::Status::New);
        assert!(second.captures[0].is_sendable());
    }

    #[test]
    fn test_usage_an_unrecorded_preview_stays_pending() {
        let dir = tempfile::tempdir().unwrap();
        scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        let second = scene_with("sync_room_list", &["two rooms"], None)
            .build_report(&fast_config(dir.path()))
            .unwrap();
        assert_eq!(
            second.captures[0].status,
            deliver::Status::New,
            "a preview never delivered must keep being offered"
        );
    }

    #[test]
    fn test_usage_same_pixels_send_nothing_and_do_not_rewrite() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "sync_room_list", "two rooms", 50, 60, 70);
        let before_bytes = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        let before_meta = std::fs::metadata(delivered.delivered_path(dir.path())).unwrap();
        let before_modified = before_meta.modified().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let staged = staging_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            50,
            60,
            70,
            deliver::Status::Same,
        );
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("sync_room_list", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 0);
        let after_bytes = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        assert_eq!(before_bytes, after_bytes);
        let after_modified = std::fs::metadata(delivered.delivered_path(dir.path()))
            .unwrap()
            .modified()
            .unwrap();
        assert_eq!(
            before_modified, after_modified,
            "same pixels must not rewrite the file"
        );
        assert!(
            !report.captures[0]
                .path
                .to_string_lossy()
                .contains("_staging")
        );
        assert!(
            !report.captures[0]
                .path
                .to_string_lossy()
                .contains("_unsent")
        );
        assert_eq!(
            report.captures[0].path,
            delivered.delivered_path(dir.path())
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_changed_pixels_send_then_replace() {
        let dir = tempfile::tempdir().unwrap();
        write_delivered(dir.path(), "sync_room_list", "two rooms", 1, 2, 3);
        let staged = staging_capture(
            dir.path(),
            "sync_room_list",
            "two rooms",
            9,
            9,
            9,
            deliver::Status::Changed,
        );
        let staged_hash = staged.artifact.pixel_hash.clone();
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("sync_room_list", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 1);
        let delivered_path = dir.path().join("sync_room_list__two_rooms.png");
        assert!(delivered_path.exists());
        let delivered_hash = deliver::Artifact::hash_png_pixels(&delivered_path).unwrap();
        assert_eq!(delivered_hash, staged_hash);
        assert_eq!(report.captures[0].path, delivered_path);
        assert!(
            !dir.path()
                .join("_unsent")
                .join("sync_room_list__two_rooms.png")
                .exists()
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_failed_send_keeps_old_and_retries() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "ui", "one", 10, 10, 10);
        let old_bytes = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        let staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            90,
            90,
            90,
            deliver::Status::Changed,
        );
        let staged_hash = staged.artifact.pixel_hash.clone();
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let failing = config_with(dir.path(), fail_always);
        assert!(scene_obj.deliver_report(&mut report, &failing).is_err());
        let still_old = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        assert_eq!(old_bytes, still_old, "failed send must keep the old file");
        let unsent_path = dir.path().join("_unsent").join("ui__one.png");
        assert!(unsent_path.exists(), "failed render waits in _unsent");
        let unsent_hash = deliver::Artifact::hash_png_pixels(&unsent_path).unwrap();
        assert_eq!(unsent_hash, staged_hash);
        no_staging_left(dir.path());
        let retry_staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            90,
            90,
            90,
            deliver::Status::Changed,
        );
        let mut retry = Report {
            captures: vec![retry_staged],
            clip: None,
        };
        let retry_cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut retry, &retry_cfg).unwrap();
        assert_eq!(sent, 1);
        let new_delivered_hash =
            deliver::Artifact::hash_png_pixels(&delivered.delivered_path(dir.path())).unwrap();
        assert_eq!(new_delivered_hash, staged_hash);
        assert!(!unsent_path.exists(), "successful send clears _unsent");
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_no_credentials_then_next_run_sends() {
        let dir = tempfile::tempdir().unwrap();
        write_delivered(dir.path(), "ui", "one", 5, 5, 5);
        let staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            80,
            80,
            80,
            deliver::Status::Changed,
        );
        let staged_hash = staged.artifact.pixel_hash.clone();
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), no_credentials);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 0);
        assert!(dir.path().join("ui__one.png").exists());
        let unsent = dir.path().join("_unsent").join("ui__one.png");
        assert!(unsent.exists());
        no_staging_left(dir.path());
        let retry_staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            80,
            80,
            80,
            deliver::Status::Changed,
        );
        let mut retry = Report {
            captures: vec![retry_staged],
            clip: None,
        };
        let working = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut retry, &working).unwrap();
        assert_eq!(sent, 1);
        let delivered_hash =
            deliver::Artifact::hash_png_pixels(&dir.path().join("ui__one.png")).unwrap();
        assert_eq!(delivered_hash, staged_hash);
        assert!(!unsent.exists());
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_chunk1_ok_chunk2_fails_then_retries_changed_only() {
        let dir = tempfile::tempdir().unwrap();
        let mut staged = Vec::new();
        for index in 0..11 {
            let red = u8::try_from(index).unwrap();
            staged.push(staging_capture(
                dir.path(),
                "ui",
                &format!("state {index}"),
                red,
                red,
                red,
                deliver::Status::New,
            ));
        }
        let mut report = Report {
            captures: staged,
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let failing = config_with(dir.path(), fail_on_single_chunk);
        assert!(scene_obj.deliver_report(&mut report, &failing).is_err());
        for index in 0..10 {
            let delivered = dir.path().join(format!("ui__state_{index}.png"));
            assert!(delivered.exists(), "chunk 1 replaced: {delivered:?}");
        }
        let missing = dir.path().join("ui__state_10.png");
        assert!(!missing.exists(), "chunk 2 not delivered");
        assert!(dir.path().join("_unsent").join("ui__state_10.png").exists());
        no_staging_left(dir.path());
        let mut retry_staged = Vec::new();
        for index in 0..11 {
            let red = u8::try_from(index).unwrap();
            retry_staged.push(staging_capture(
                dir.path(),
                "ui",
                &format!("state {index}"),
                red,
                red,
                red,
                deliver::Status::New,
            ));
        }
        let mut retry = Report {
            captures: retry_staged,
            clip: None,
        };
        let working = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut retry, &working).unwrap();
        assert_eq!(sent, 1, "only the failed chunk retries");
        assert!(dir.path().join("ui__state_10.png").exists());
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_dry_run_leaves_top_level_byte_identical() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "ui", "one", 11, 22, 33);
        let before = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        let staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            99,
            99,
            99,
            deliver::Status::Changed,
        );
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        unsafe { std::env::set_var("LELE_PREVIEW_DRY_RUN", "1") };
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        unsafe { std::env::remove_var("LELE_PREVIEW_DRY_RUN") };
        assert_eq!(sent, 0);
        let after = std::fs::read(delivered.delivered_path(dir.path())).unwrap();
        assert_eq!(before, after, "dry run never changes the top level");
        assert!(
            report.captures[0].path.exists(),
            "returned path stays valid"
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_dry_run_with_identical_pixels_leaves_unsent_empty() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "ui", "one", 50, 60, 70);
        let staged = staging_capture(dir.path(), "ui", "one", 50, 60, 70, deliver::Status::Same);
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        unsafe { std::env::set_var("LELE_PREVIEW_DRY_RUN", "1") };
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        unsafe { std::env::remove_var("LELE_PREVIEW_DRY_RUN") };
        assert_eq!(sent, 0);
        assert_eq!(
            report.captures[0].path,
            delivered.delivered_path(dir.path())
        );
        let leftover = std::fs::read_dir(dir.path().join(basic::constants::UNSENT_DIR))
            .map_or(0, Iterator::count);
        assert_eq!(
            leftover, 0,
            "_unsent must hold only renders that differ from what was delivered"
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_same_render_clears_older_unsent_entry() {
        let dir = tempfile::tempdir().unwrap();
        let delivered = write_delivered(dir.path(), "ui", "one", 10, 10, 10);
        let stale_unsent = dir
            .path()
            .join(basic::constants::UNSENT_DIR)
            .join("ui__one.png");
        solid_png(&stale_unsent, 90, 90, 90);
        assert!(stale_unsent.exists());
        let staged = staging_capture(dir.path(), "ui", "one", 10, 10, 10, deliver::Status::Same);
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 0);
        assert!(
            !stale_unsent.exists(),
            "a Same render removes its older _unsent entry"
        );
        assert_eq!(
            report.captures[0].path,
            delivered.delivered_path(dir.path())
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_dry_run_stashes_only_changed_item() {
        let dir = tempfile::tempdir().unwrap();
        let one = write_delivered(dir.path(), "ui", "one", 11, 22, 33);
        let two = write_delivered(dir.path(), "ui", "two", 44, 55, 66);
        let before_two = std::fs::read(two.delivered_path(dir.path())).unwrap();
        let staged_one = staging_capture(
            dir.path(),
            "ui",
            "one",
            99,
            99,
            99,
            deliver::Status::Changed,
        );
        let staged_two =
            staging_capture(dir.path(), "ui", "two", 44, 55, 66, deliver::Status::Same);
        let mut report = Report {
            captures: vec![staged_one, staged_two],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        unsafe { std::env::set_var("LELE_PREVIEW_DRY_RUN", "1") };
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        unsafe { std::env::remove_var("LELE_PREVIEW_DRY_RUN") };
        assert_eq!(sent, 0);
        assert!(
            dir.path()
                .join(basic::constants::UNSENT_DIR)
                .join(one.file_name())
                .exists(),
            "changed render waits in _unsent"
        );
        assert!(
            !dir.path()
                .join(basic::constants::UNSENT_DIR)
                .join(two.file_name())
                .exists(),
            "Same render must not be stashed in _unsent"
        );
        assert_eq!(report.captures[1].path, two.delivered_path(dir.path()));
        let after_two = std::fs::read(two.delivered_path(dir.path())).unwrap();
        assert_eq!(before_two, after_two);
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_corrupt_delivered_fails_with_nothing_sent() {
        let dir = tempfile::tempdir().unwrap();
        let probe = deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("one"),
            fingerprint: deliver::Artifact::fingerprint_of("ui", "one"),
            pixel_hash: String::from("0123456789abcdef"),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        let delivered = probe.delivered_path(dir.path());
        std::fs::create_dir_all(delivered.parent().unwrap()).unwrap();
        std::fs::write(&delivered, b"bad-content").unwrap();
        let before = std::fs::read(&delivered).unwrap();
        let staged = staging_capture(dir.path(), "ui", "one", 40, 40, 40, deliver::Status::New);
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let result = scene_obj.deliver_report(&mut report, &cfg);
        assert!(
            result.is_err(),
            "undecodable on-disk PNG is an error naming the path"
        );
        let after = std::fs::read(&delivered).unwrap();
        assert_eq!(before, after);
        assert!(
            !dir.path().join("_unsent").join("ui__one.png").exists(),
            "change nothing on invalid baseline"
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_usage_clip_frames_gate_status() {
        let dir = tempfile::tempdir().unwrap();
        let scene_name = "clipscene";
        let fps = 10;
        let frames_dir = deliver::Artifact::delivered_frames_dir(dir.path(), scene_name);
        std::fs::create_dir_all(&frames_dir).unwrap();
        let mut frame_hashes = Vec::new();
        for index in 0..3 {
            let path = frames_dir.join(format!("frame-{index:04}.png"));
            let hash = solid_png(&path, 20, 20, 20);
            frame_hashes.push(hash);
        }
        let clip_hash = Scene::clip_hash(fps, &frame_hashes);
        let artifact = deliver::Artifact {
            scene: scene_name.to_string(),
            label: String::from("tick"),
            fingerprint: deliver::Artifact::fingerprint_of(scene_name, "clip:tick"),
            pixel_hash: clip_hash,
            media: deliver::Media::Mp4,
            kind: scene::Kind::System,
        };
        let mp4 = artifact.delivered_path(dir.path());
        std::fs::write(&mp4, b"fake-mp4-a").unwrap();
        assert_eq!(
            artifact
                .clip_status_against_baseline(dir.path(), fps)
                .unwrap(),
            deliver::Status::Same
        );
        std::fs::write(&mp4, b"fake-mp4-b-different-bytes").unwrap();
        assert_eq!(
            artifact
                .clip_status_against_baseline(dir.path(), fps)
                .unwrap(),
            deliver::Status::Same,
            "re-encoded mp4 with identical frames stays Same"
        );
        solid_png(&frames_dir.join("frame-0001.png"), 99, 1, 1);
        assert_eq!(
            artifact
                .clip_status_against_baseline(dir.path(), fps)
                .unwrap(),
            deliver::Status::Changed,
            "one changed frame is Changed"
        );
        let _ = std::fs::remove_dir_all(&frames_dir);
        assert_eq!(
            artifact
                .clip_status_against_baseline(dir.path(), fps)
                .unwrap(),
            deliver::Status::New,
            "missing frames folder is New"
        );
        std::fs::create_dir_all(&frames_dir).unwrap();
        solid_png(&frames_dir.join("frame-0000.png"), 20, 20, 20);
        let _ = std::fs::remove_file(&mp4);
        assert_eq!(
            artifact
                .clip_status_against_baseline(dir.path(), fps)
                .unwrap(),
            deliver::Status::New,
            "missing mp4 is New"
        );
    }

    #[test]
    fn test_usage_sixteen_threads_delivering_different_scenes_send_each_once() {
        let dir = tempfile::tempdir().unwrap();
        for thread in 0..16 {
            let probe = deliver::Artifact {
                scene: format!("parallel_{thread}"),
                label: String::from("a"),
                fingerprint: deliver::Artifact::fingerprint_of(&format!("parallel_{thread}"), "a"),
                pixel_hash: String::new(),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            };
            let _ = std::fs::remove_file(probe.delivered_path(dir.path()));
        }
        let _ = std::fs::remove_dir_all(dir.path().join(basic::constants::STAGING_DIR));
        let path = dir.path().to_path_buf();
        std::thread::scope(|scope| {
            for thread in 0..16 {
                let path = path.clone();
                scope.spawn(move || {
                    let name = format!("parallel_{thread}");
                    for _ in 0..5 {
                        let capture =
                            staging_capture(&path, &name, "a", 70, 70, 70, deliver::Status::New);
                        let mut report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let send = file_recording_send;
                        let cfg = preview::Config {
                            out_dir: path.clone(),
                            send_preview_group: send,
                            ..preview::Config::default()
                        };
                        let _ = scene_with(&name, &[], None).deliver_report(&mut report, &cfg);
                    }
                });
            }
        });
        for thread in 0..16 {
            let name = format!("parallel_{thread}");
            let probe = deliver::Artifact {
                scene: name.clone(),
                label: String::from("a"),
                fingerprint: deliver::Artifact::fingerprint_of(&name, "a"),
                pixel_hash: String::new(),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            };
            assert!(
                probe.delivered_path(&path).exists(),
                "missing delivered for {name}"
            );
        }
        no_staging_left(&path);
    }

    #[test]
    fn test_usage_sixteen_threads_delivering_same_scene_send_exactly_once() {
        let dir = tempfile::tempdir().unwrap();
        let _ = std::fs::remove_dir_all(dir.path().join(basic::constants::STAGING_DIR));
        let path = dir.path().to_path_buf();
        let log_path = path.join("test-sent.log");
        unsafe { std::env::set_var("PREVIEW_SENT_LOG", log_path.as_os_str()) };
        let barrier = std::sync::Barrier::new(16);
        std::thread::scope(|scope| {
            for _ in 0..16 {
                let path = path.clone();
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    for _ in 0..5 {
                        let capture =
                            staging_capture(&path, "shared", "a", 75, 75, 75, deliver::Status::New);
                        let mut report = Report {
                            captures: vec![capture],
                            clip: None,
                        };
                        let cfg = config_with(&path, file_recording_send);
                        let _ = scene_with("shared", &[], None).deliver_report(&mut report, &cfg);
                    }
                });
            }
        });
        unsafe { std::env::remove_var("PREVIEW_SENT_LOG") };
        let probe = deliver::Artifact {
            scene: String::from("shared"),
            label: String::from("a"),
            fingerprint: deliver::Artifact::fingerprint_of("shared", "a"),
            pixel_hash: String::new(),
            media: deliver::Media::Png,
            kind: scene::Kind::System,
        };
        assert!(probe.delivered_path(&path).exists());
        let text = std::fs::read_to_string(&log_path).unwrap();
        let lines: Vec<&str> = text.lines().filter(|line| !line.is_empty()).collect();
        assert_eq!(
            lines.len(),
            1,
            "same scene must send exactly once, got {lines:?}"
        );
        no_staging_left(&path);
    }

    #[test]
    #[ignore = "worker for process tests"]
    fn preview_delivery_worker_delivers_one_scene() {
        let dir = std::env::var("PREVIEW_WORKER_OUT_DIR").unwrap();
        let scene_name = std::env::var("PREVIEW_WORKER_SCENE").unwrap();
        let iterations: usize = std::env::var("PREVIEW_WORKER_ITERATIONS")
            .unwrap()
            .parse()
            .unwrap();
        let dir = PathBuf::from(dir);
        for _ in 0..iterations {
            let capture = staging_capture(&dir, &scene_name, "a", 66, 66, 66, deliver::Status::New);
            let mut report = Report {
                captures: vec![capture],
                clip: None,
            };
            let config = preview::Config {
                out_dir: dir.clone(),
                send_preview_group: file_recording_send,
                ..preview::Config::default()
            };
            let _ = scene_with(&scene_name, &[], None).deliver_report(&mut report, &config);
        }
    }

    #[test]
    fn test_usage_real_processes_delivering_different_scenes_send_each_once() {
        let dir = tempfile::tempdir().unwrap();
        let exe = std::env::current_exe().unwrap();
        let mut children = Vec::new();
        for thread in 0..4 {
            let child = std::process::Command::new(&exe)
                .arg("run_tests::tests::preview_delivery_worker_delivers_one_scene")
                .arg("--exact")
                .arg("--ignored")
                .arg("--nocapture")
                .env("PREVIEW_WORKER_OUT_DIR", dir.path())
                .env("PREVIEW_WORKER_SCENE", format!("dproc_{thread}"))
                .env("PREVIEW_WORKER_ITERATIONS", "5")
                .env("TELEGRAM_BOT_TOKEN", "")
                .env("TELEGRAM_CHAT_ID", "")
                .spawn()
                .unwrap();
            children.push(child);
        }
        for mut child in children {
            assert!(child.wait().unwrap().success());
        }
        for thread in 0..4 {
            let probe = deliver::Artifact {
                scene: format!("dproc_{thread}"),
                label: String::from("a"),
                fingerprint: deliver::Artifact::fingerprint_of(&format!("dproc_{thread}"), "a"),
                pixel_hash: String::new(),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            };
            assert!(
                probe.delivered_path(dir.path()).exists(),
                "missing delivered for dproc_{thread}"
            );
        }
        no_staging_left(dir.path());
    }
    #[test]
    fn test_delivery_log_records_one_decision_per_capture() {
        let dir = tempfile::tempdir().unwrap();
        write_delivered(dir.path(), "ui", "one", 1, 2, 3);
        write_delivered(dir.path(), "ui", "two", 4, 5, 6);
        let staged_one = staging_capture(
            dir.path(),
            "ui",
            "one",
            90,
            90,
            90,
            deliver::Status::Changed,
        );
        let staged_two = staging_capture(
            dir.path(),
            "ui",
            "two",
            91,
            91,
            91,
            deliver::Status::Changed,
        );
        let mut report = Report {
            captures: vec![staged_one, staged_two],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 2);
        let text = std::fs::read_to_string(dir.path().join("_log").join("deliveries.log")).unwrap();
        let decisions: Vec<&str> = text
            .lines()
            .filter(|line| line.contains("event=decision"))
            .collect();
        assert_eq!(
            decisions.len(),
            2,
            "one decision per capture: {decisions:?}"
        );
        assert!(decisions.iter().any(|line| line.contains("label=one")));
        assert!(decisions.iter().any(|line| line.contains("label=two")));
        assert!(text.contains("event=send"));
        no_staging_left(dir.path());
    }

    #[test]
    fn test_delivery_log_flags_unchanged_image_sent_in_album() {
        let dir = tempfile::tempdir().unwrap();
        write_delivered(dir.path(), "ui", "one", 50, 60, 70);
        write_delivered(dir.path(), "ui", "two", 1, 2, 3);
        let staged_same =
            staging_capture(dir.path(), "ui", "one", 50, 60, 70, deliver::Status::Same);
        let staged_changed = staging_capture(
            dir.path(),
            "ui",
            "two",
            90,
            90,
            90,
            deliver::Status::Changed,
        );
        let mut report = Report {
            captures: vec![staged_same, staged_changed],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 2);
        let text = std::fs::read_to_string(dir.path().join("_log").join("deliveries.log")).unwrap();
        assert!(
            text.lines()
                .any(|line| line.contains("event=sent_unchanged")
                    && line.contains("label=one")
                    && line.contains("why=album_with_changed_sibling")),
            "Same image sent in an album must be flagged: {text}"
        );
        no_staging_left(dir.path());
    }

    #[test]
    fn test_delivery_log_write_failure_does_not_fail_delivery() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("_log"), b"not-a-directory").unwrap();
        write_delivered(dir.path(), "ui", "one", 1, 2, 3);
        let staged = staging_capture(
            dir.path(),
            "ui",
            "one",
            90,
            90,
            90,
            deliver::Status::Changed,
        );
        let mut report = Report {
            captures: vec![staged],
            clip: None,
        };
        let scene_obj = scene_with("ui", &[], None);
        let cfg = config_with(dir.path(), recording_send);
        let sent = scene_obj.deliver_report(&mut report, &cfg).unwrap();
        assert_eq!(sent, 1, "unwritable log must never fail a delivery");
        no_staging_left(dir.path());
    }
    // no test_usage necessary
}
