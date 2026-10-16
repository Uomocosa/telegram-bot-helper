use crate::deliver;
use crate::deliver::basic::enums::Media;
use crate::deliver::basic::structs::Artifact;

#[must_use]
pub fn file_name(artifact: &Artifact) -> String {
    let extension = match artifact.media {
        Media::Png => "png",
        Media::Mp4 => "mp4",
    };
    format!(
        "{}__{}.{extension}",
        artifact.scene,
        deliver::slug::slug(&artifact.label)
    )
}

#[cfg(test)]
mod tests {
    use crate::deliver::basic::enums::Media;
    use crate::deliver::basic::structs::Artifact;
    use crate::deliver::file_name::file_name;
    use crate::scene;

    #[test]
    fn test_usage() {
        let mut artifact = Artifact {
            scene: String::from("lobby_ui"),
            label: String::from("two rooms"),
            fingerprint: String::from("lobby_ui|two rooms"),
            pixel_hash: String::from("h"),
            media: Media::Png,
            kind: scene::Kind::System,
        };
        assert_eq!(file_name(&artifact), "lobby_ui__two_rooms.png");
        artifact.media = Media::Mp4;
        artifact.label = String::from("add gamma");
        assert_eq!(file_name(&artifact), "lobby_ui__add_gamma.mp4");
    }
}
