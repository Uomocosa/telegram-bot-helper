use std::time::Duration;

use bevy::prelude::*;
use bevy::time::TimeUpdateStrategy;

pub fn advance(app: &mut App, step: Duration) {
    app.insert_resource(TimeUpdateStrategy::ManualDuration(step));
    app.update();
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
}
// no test_usage necessary
