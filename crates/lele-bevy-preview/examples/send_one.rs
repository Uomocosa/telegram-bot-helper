use std::path::PathBuf;

use lele_bevy_preview::deliver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: send_one <path-to-png-or-mp4>")?;
    let caption = path.display().to_string();
    let sent = capture(&path).send_one(&caption)?;
    if sent {
        println!("sent {}", path.display());
    } else {
        println!("no credentials; left {} on disk", path.display());
    }
    Ok(())
}

// needed helper: an ad-hoc file needs the metadata the manifest would normally supply
fn capture(path: &std::path::Path) -> deliver::Capture {
    let label = path.file_name().map_or_else(
        || String::from("artifact"),
        |name| name.to_string_lossy().to_string(),
    );
    let media = if path.extension().is_some_and(|extension| extension == "mp4") {
        deliver::Media::Mp4
    } else {
        deliver::Media::Png
    };
    deliver::Capture {
        path: path.to_path_buf(),
        artifact: deliver::Artifact {
            scene: String::from("manual"),
            label: label.clone(),
            fingerprint: deliver::Artifact::fingerprint_of("manual", &label),
            pixel_hash: String::new(),
            media,
            kind: lele_bevy_preview::scene::Kind::System,
        },
        status: deliver::Status::New,
    }
}
