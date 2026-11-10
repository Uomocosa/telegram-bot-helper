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
    use super::check;

    #[test]
    fn test_usage() {
        let _ = check;
    }
}
