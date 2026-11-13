use crate::PreviewApp;

pub fn advance(app: &mut PreviewApp, step: std::time::Duration) {
    app.app
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(step));
    app.app.update();
    app.app
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::ZERO,
        ));
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::prelude::*;
    use bevy::time::Real;

    use super::advance;
    use crate::PreviewApp;
    use crate::preview;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        let mut preview_app = PreviewApp {
            app,
            target: Handle::default(),
            config: preview::Config::default(),
        };
        advance(&mut preview_app, Duration::ZERO);
        let primed = preview_app.app.world().resource::<Time<Real>>().elapsed();
        advance(&mut preview_app, Duration::from_millis(50));
        let first = preview_app.app.world().resource::<Time<Real>>().elapsed();
        assert_eq!(first, primed + Duration::from_millis(50));
        advance(&mut preview_app, Duration::from_millis(20));
        let second = preview_app.app.world().resource::<Time<Real>>().elapsed();
        assert_eq!(second, first + Duration::from_millis(20));
    }
}
