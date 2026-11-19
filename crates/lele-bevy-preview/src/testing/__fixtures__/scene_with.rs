use crate::scene;
use crate::testing;

pub fn scene_with(name: &str, labels: &[&str], timeline: Option<scene::Timeline>) -> scene::Scene {
    scene::Scene {
        name: String::from(name),
        kind: scene::Kind::System,
        build: testing::noop_build,
        states: labels
            .iter()
            .map(|label| scene::State {
                label: String::from(*label),
                apply: testing::noop,
            })
            .collect(),
        timeline,
    }
}

#[cfg(test)]
mod tests {
    use super::scene_with;

    #[test]
    fn test_usage() {
        assert_eq!(scene_with("ui", &["rest"], None).states.len(), 1);
    }
}
