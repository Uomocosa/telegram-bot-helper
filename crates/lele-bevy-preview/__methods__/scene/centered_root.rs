use bevy::prelude::{AlignItems, FlexDirection, JustifyContent, Node, Val, default};

#[must_use]
pub fn centered_root() -> Node {
    let gap = Val::Px(6.0);
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: gap,
        ..default()
    }
}

#[cfg(test)]
mod tests {
    use super::centered_root;

    #[test]
    fn test_usage() {
        assert_eq!(centered_root().width, bevy::prelude::Val::Percent(100.0));
    }
}
