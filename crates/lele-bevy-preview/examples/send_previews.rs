use std::path::PathBuf;

use lele_bevy_preview::deliver;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args_os().nth(1).map_or_else(
        || std::env::temp_dir().join("lele-bevy-preview"),
        PathBuf::from,
    );
    let mut previews: Vec<PathBuf> = std::fs::read_dir(&dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension().is_some_and(|extension| {
                extension.eq_ignore_ascii_case("png") || extension.eq_ignore_ascii_case("mp4")
            })
        })
        .collect();
    previews.sort();
    if previews.is_empty() {
        eprintln!("no previews in {}", dir.display());
        return Ok(());
    }
    let mut sent = 0usize;
    for path in previews {
        let label = path.file_name().map_or_else(
            || String::from("artifact"),
            |name| name.to_string_lossy().to_string(),
        );
        let media = if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("mp4"))
        {
            deliver::Media::Mp4
        } else {
            deliver::Media::Png
        };
        let capture = deliver::Capture {
            path: path.clone(),
            artifact: deliver::Artifact {
                scene: String::from("manual"),
                label: label.clone(),
                fingerprint: deliver::Artifact::fingerprint_of("manual", &label),
                pixel_hash: String::new(),
                media,
                kind: lele_bevy_preview::scene::Kind::System,
            },
            status: deliver::Status::New,
        };
        if capture.send_one(&label)? {
            sent = sent.saturating_add(1);
        }
    }
    println!("sent {sent} artifact(s) from {}", dir.display());
    Ok(())
}
