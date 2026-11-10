use crate::Error;
use crate::scene;
use crate::scene::basic::constants::{MAX_FPS, MAX_TIMELINE_FRAMES};

pub fn check_timeline(scene: &scene::Scene) -> Result<(), Error> {
    let Some(timeline) = scene.timeline.as_ref() else {
        return Ok(());
    };
    if timeline.frames == 0 {
        return Err(Error::Scene(format!(
            "scene `{}` timeline `{}` declares zero frames",
            scene.name, timeline.label
        )));
    }
    if timeline.frames > MAX_TIMELINE_FRAMES {
        return Err(Error::Scene(format!(
            "scene `{}` timeline `{}` declares {} frames, over the cap of {MAX_TIMELINE_FRAMES}",
            scene.name, timeline.label, timeline.frames
        )));
    }
    if timeline.fps == 0 || timeline.fps > MAX_FPS {
        return Err(Error::Scene(format!(
            "scene `{}` timeline `{}` fps {} is outside 1..={MAX_FPS}",
            scene.name, timeline.label, timeline.fps
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check_timeline;
    use crate::scene;

    #[test]
    fn test_usage() {
        let scene = scene::Scene {
            name: String::from("ui"),
            kind: scene::Kind::System,
            build: |_app| {},
            states: Vec::new(),
            timeline: None,
        };
        assert!(check_timeline(&scene).is_ok());
    }
}
