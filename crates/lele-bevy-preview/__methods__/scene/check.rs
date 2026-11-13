use crate::Error;
use crate::scene;

pub fn check(scene: &scene::Scene) -> Result<(), Error> {
    if scene.name.trim().is_empty() {
        return Err(Error::Scene(String::from("scene name must not be empty")));
    }
    if scene.states.is_empty() {
        return Err(Error::Scene(format!(
            "scene `{}` declares no states, so it would render nothing",
            scene.name
        )));
    }
    if scene.states.len() > scene::MAX_STATES {
        return Err(Error::Scene(format!(
            "scene `{}` declares {} states, over the cap of {}",
            scene.name,
            scene.states.len(),
            scene::MAX_STATES
        )));
    }
    for state in &scene.states {
        if state.label.trim().is_empty() {
            return Err(Error::Scene(format!(
                "scene `{}` has a state with an empty label",
                scene.name
            )));
        }
    }
    scene.check_timeline()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::world::World;
    use bevy::prelude::App;

    use super::check;
    use crate::Error;
    use crate::scene;

    fn noop_world(_world: &mut World) {}
    fn noop_build(_app: &mut App) {}

    fn scene_with(name: &str, labels: &[&str]) -> scene::Scene {
        scene::Scene {
            name: String::from(name),
            kind: scene::Kind::System,
            build: noop_build,
            states: labels
                .iter()
                .map(|label| scene::State {
                    label: String::from(*label),
                    apply: noop_world,
                })
                .collect(),
            timeline: None,
        }
    }

    #[test]
    fn test_usage() {
        assert!(check(&scene_with("ui", &["rest"])).is_ok());
        assert!(matches!(
            check(&scene_with("ui", &[])),
            Err(Error::Scene(_))
        ));
        assert!(matches!(
            check(&scene_with("  ", &["rest"])),
            Err(Error::Scene(_))
        ));
    }
}
