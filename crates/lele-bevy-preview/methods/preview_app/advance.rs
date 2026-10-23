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
    use super::advance;

    #[test]
    fn test_usage() {
        let _ = advance;
    }
}
