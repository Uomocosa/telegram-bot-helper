#[cfg(test)]
mod tests {
    use crate::deliver;
    use crate::scene::Timeline;
    use crate::testing::fast_config;
    use crate::testing::noop;
    use crate::testing::scene_with;
    use crate::testing::spawn_visible_change;
    use crate::testing::timeline_with;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let clip = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                3,
                30,
                spawn_visible_change,
            )),
        )
        .build_clip(
            &fast_config(dir.path()),
            &deliver::Artifact::staging_root(dir.path(), "press"),
        )
        .unwrap()
        .expect("a declared timeline must produce a clip");
        assert!(clip.path.exists(), "{} missing", clip.path.display());
        assert!(clip.path.extension().is_some_and(|ext| ext == "mp4"));
    }

    #[test]
    fn test_usage_renders_the_lead_in_then_one_png_per_declared_frame() {
        let dir = tempfile::tempdir().unwrap();
        let frames = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                3,
                30,
                spawn_visible_change,
            )),
        )
        .render_frames(
            &fast_config(dir.path()),
            &deliver::Artifact::staging_root(dir.path(), "press"),
        )
        .unwrap();
        assert_eq!(frames.len(), 5, "2 lead-in frames plus the 3 declared ones");
        for frame in &frames {
            assert!(frame.path.exists(), "{} missing", frame.path.display());
        }
    }

    #[test]
    fn test_usage_a_scene_without_a_timeline_produces_no_frames() {
        let dir = tempfile::tempdir().unwrap();
        let config = fast_config(dir.path());
        assert_eq!(
            scene_with("press", &["rest"], None)
                .render_frames(
                    &config,
                    &deliver::Artifact::staging_root(dir.path(), "press")
                )
                .unwrap()
                .len(),
            0
        );
        assert!(
            scene_with("press", &["rest"], None)
                .build_clip(
                    &config,
                    &deliver::Artifact::staging_root(dir.path(), "press")
                )
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_usage_two_runs_produce_identical_pixels() {
        let dir = tempfile::tempdir().unwrap();
        let config = fast_config(dir.path());
        let one = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                2,
                30,
                spawn_visible_change,
            )),
        )
        .render_frames(
            &config,
            &deliver::Artifact::staging_root(dir.path(), "press"),
        )
        .unwrap();
        let first = std::fs::read(&one[1].path).unwrap();
        let two = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                2,
                30,
                spawn_visible_change,
            )),
        )
        .render_frames(
            &config,
            &deliver::Artifact::staging_root(dir.path(), "press"),
        )
        .unwrap();
        let second = std::fs::read(&two[1].path).unwrap();
        assert_eq!(
            first, second,
            "fixed-step rendering must be reproducible frame to frame"
        );
    }

    #[test]
    fn test_usage_a_missing_ffmpeg_binary_is_reported_not_ignored() {
        let dir = tempfile::tempdir().unwrap();
        unsafe { std::env::set_var("LELE_FFMPEG", "/nonexistent/ffmpeg-binary") };
        let result = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                2,
                30,
                spawn_visible_change,
            )),
        )
        .build_clip(
            &fast_config(dir.path()),
            &deliver::Artifact::staging_root(dir.path(), "press"),
        );
        unsafe { std::env::remove_var("LELE_FFMPEG") };
        assert!(matches!(result, Err(crate::Error::FfmpegSpawn { .. })));
    }

    #[test]
    fn test_usage_a_clip_whose_frames_never_differ_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let flat = Timeline {
            apply: noop,
            ..timeline_with("hover_then_press", 3, 30, spawn_visible_change)
        };
        let result = scene_with("press", &["rest"], Some(flat)).build_clip(
            &fast_config(dir.path()),
            &deliver::Artifact::staging_root(dir.path(), "press"),
        );
        assert!(matches!(result, Err(crate::Error::StaticClip { .. })));
    }

    #[test]
    fn test_usage_the_baseline_precedes_the_change() {
        let dir = tempfile::tempdir().unwrap();
        let frames = scene_with(
            "press",
            &["rest"],
            Some(timeline_with(
                "hover_then_press",
                3,
                30,
                spawn_visible_change,
            )),
        )
        .render_frames(
            &fast_config(dir.path()),
            &deliver::Artifact::staging_root(dir.path(), "press"),
        )
        .unwrap();
        assert_eq!(
            frames[0].pixel_hash, frames[1].pixel_hash,
            "lead-in frames show the baseline"
        );
        assert_ne!(
            frames[1].pixel_hash, frames[4].pixel_hash,
            "the change shows up after the lead-in"
        );
    }
    // no test_usage necessary
}
