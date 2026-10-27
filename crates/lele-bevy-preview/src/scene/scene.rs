use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::App;

use crate::Error;
use crate::deliver;
use crate::preview;
use crate::report::Report;
use crate::scene;
use crate::timeline;

pub struct Scene {
    pub name: String,
    pub kind: scene::Kind,
    pub build: fn(&mut App),
    pub states: Vec<scene::State>,
    pub timeline: Option<scene::Timeline>,
}

#[atomic_delegates]
impl Scene {
    pub fn centered_root() -> bevy::prelude::Node {}
    #[must_use]
    pub fn scratch_dir(&self, config: &preview::Config) -> PathBuf {}
    pub fn check(&self) -> Result<(), Error> {}
    pub fn send_changed(
        &self,
        captures: &[deliver::Capture],
        clip: Option<&deliver::Capture>,
    ) -> Result<usize, Error> {
    }
    pub fn render_states(
        &self,
        config: &preview::Config,
    ) -> Result<Vec<preview::RenderedState>, Error> {
    }
    pub fn render_frames(
        &self,
        config: &preview::Config,
    ) -> Result<Vec<timeline::RenderedFrame>, Error> {
    }
    pub fn build_clip(
        &self,
        config: &preview::Config,
        previous: Option<&deliver::Manifest>,
    ) -> Result<Option<deliver::Capture>, Error> {
    }
    pub fn build_report(&self, config: &preview::Config) -> Result<Report, Error> {}
    pub fn run(&self, config: &preview::Config, crate_name: &str) -> Result<Report, Error> {}
    pub(crate) fn check_timeline(&self) -> Result<(), Error> {}
    pub(crate) fn render_state_in_fresh_app(
        &self,
        config: &preview::Config,
        state: &scene::State,
        out: &std::path::Path,
    ) -> Result<String, Error> {
    }
    pub(crate) fn ffmpeg_encode(
        frames: &[PathBuf],
        fps: u32,
        out: &std::path::Path,
        scratch: &std::path::Path,
    ) -> Result<PathBuf, Error> {
    }
    pub(crate) fn ffmpeg_binary() -> String {}
    pub(crate) fn announce_artifacts(
        captures: &[deliver::Capture],
        clip: Option<&deliver::Capture>,
    ) {
    }
    pub(crate) fn clip_hash(fps: u32, frame_hashes: &[String]) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Scene;
    use crate::preview;
    use crate::scene;

    #[test]
    fn test_usage() {
        let root = Scene::centered_root();
        assert_eq!(root.width, bevy::prelude::Val::Percent(100.0));
        let dir = tempfile::tempdir().unwrap();
        let scene = Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: |_app| {},
            states: Vec::new(),
            timeline: None,
        };
        let config = preview::Config {
            out_dir: dir.path().to_path_buf(),
            ..preview::Config::default()
        };
        assert!(scene.scratch_dir(&config).ends_with("ui_frames"));
        let frames = [String::from("a"), String::from("b")];
        assert_eq!(Scene::clip_hash(10, &frames), Scene::clip_hash(10, &frames));
        assert_ne!(Scene::clip_hash(10, &frames), Scene::clip_hash(12, &frames));
    }
}
