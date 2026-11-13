use crate::deliver;

#[must_use]
pub fn file_name(artifact: &deliver::Artifact) -> String {
    let extension = match artifact.media {
        deliver::Media::Png => "png",
        deliver::Media::Mp4 => "mp4",
    };
    format!(
        "{}__{}.{extension}",
        artifact.scene,
        deliver::Artifact::slug(&artifact.label)
    )
}

#[cfg(test)]
mod tests {
    use super::file_name;
    use crate::deliver;
    use crate::scene;

    fn artifact(media: deliver::Media) -> deliver::Artifact {
        deliver::Artifact {
            scene: String::from("ui"),
            label: String::from("Hover Here"),
            fingerprint: String::new(),
            pixel_hash: String::new(),
            media,
            kind: scene::Kind::System,
        }
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            file_name(&artifact(deliver::Media::Png)),
            "ui__hover_here.png"
        );
        assert_eq!(
            file_name(&artifact(deliver::Media::Mp4)),
            "ui__hover_here.mp4"
        );
    }
}
