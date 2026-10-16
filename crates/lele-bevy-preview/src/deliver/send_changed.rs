use crate::Error;
use crate::deliver;
use crate::scene::Scene;

const ALBUM_LIMIT: usize = 10;

pub fn send_changed(
    scene: &Scene,
    captures: &[deliver::Capture],
    clip: Option<&deliver::Capture>,
) -> Result<usize, Error> {
    let all: Vec<&deliver::Capture> = captures.iter().chain(clip).collect();
    if !deliver::resend_all::resend_all()
        && !all
            .iter()
            .any(|capture| deliver::sendable::sendable(capture.status))
    {
        return Ok(0);
    }
    let caption = deliver::caption::caption(scene.kind, &scene.name, &all);
    let mut sent: usize = 0;
    for chunk in all.chunks(ALBUM_LIMIT) {
        sent = sent.saturating_add(deliver::send_group::send_group(chunk, &caption)?);
    }
    Ok(sent)
}
// no test_usage necessary
