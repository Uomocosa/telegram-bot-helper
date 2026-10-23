use crate::Error;
use crate::deliver;
use crate::scene;

const ALBUM_LIMIT: usize = 10;

pub fn send_changed(
    scene: &scene::Scene,
    captures: &[deliver::Capture],
    clip: Option<&deliver::Capture>,
) -> Result<usize, Error> {
    let all: Vec<&deliver::Capture> = captures.iter().chain(clip).collect();
    if !deliver::Capture::is_resend_all_requested()
        && !all.iter().any(|capture| capture.is_sendable())
    {
        return Ok(0);
    }
    let caption = scene.kind.caption(&scene.name, &all);
    let mut sent: usize = 0;
    for chunk in all.chunks(ALBUM_LIMIT) {
        sent = sent.saturating_add(deliver::Capture::send_group(chunk, &caption)?);
    }
    Ok(sent)
}

#[cfg(test)]
mod tests {
    use super::send_changed;

    #[test]
    fn test_usage() {
        let _ = send_changed;
    }
}
