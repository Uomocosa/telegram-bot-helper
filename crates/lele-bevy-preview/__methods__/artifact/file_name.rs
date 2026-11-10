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

    #[test]
    fn test_usage() {
        let _ = file_name;
    }
}
