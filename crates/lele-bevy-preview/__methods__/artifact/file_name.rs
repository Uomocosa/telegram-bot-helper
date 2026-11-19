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
    use crate::testing::artifact;

    #[test]
    fn test_usage() {
        let mut png = artifact("ui", "Hover Here", "");
        png.media = deliver::Media::Png;
        assert_eq!(file_name(&png), "ui__hover_here.png");
        let mut mp4 = artifact("ui", "Hover Here", "");
        mp4.media = deliver::Media::Mp4;
        assert_eq!(file_name(&mp4), "ui__hover_here.mp4");
    }
}
