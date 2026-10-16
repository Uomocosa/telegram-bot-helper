use bevy::prelude::*;

#[must_use]
pub fn centered_root() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(6.0),
        ..default()
    }
}
// no test_usage necessary
