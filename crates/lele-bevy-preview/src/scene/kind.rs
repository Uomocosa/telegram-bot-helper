use atomic_delegate_macros::atomic_delegates;
use serde::{Deserialize, Serialize};

use crate::deliver;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Kind {
    #[default]
    System,
    Component,
    Plugin,
    App,
}

#[rustfmt::skip]
impl Kind {
    #[must_use]
    pub const fn tag(self) -> &'static str {
        match self {
            Self::System => "[system]",
            Self::Component => "[component]",
            Self::Plugin => "[plugin]",
            Self::App => "[app]",
        }
    }
}

#[atomic_delegates]
impl Kind {
    #[must_use]
    pub fn caption(self, scene_name: &str, items: &[&deliver::Capture]) -> String {}
}

#[cfg(test)]
mod tests {
    use super::Kind;

    #[test]
    fn test_usage() {
        assert_eq!(Kind::System.tag(), "[system]");
        assert_eq!(Kind::App.tag(), "[app]");
    }
}
