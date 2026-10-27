use std::fmt::Write;

use crate::deliver;

pub fn write_markdown_section(out: &mut String, title: &str, captures: &[&deliver::Capture]) {
    if captures.is_empty() {
        return;
    }
    let _ = writeln!(out, "## {title}\n");
    for capture in captures {
        let name = capture.path.file_name().map_or_else(
            || capture.artifact.label.clone(),
            |name| name.to_string_lossy().to_string(),
        );
        let _ = writeln!(out, "- `{}` ({name})", capture.artifact.label);
    }
    out.push('\n');
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::write_markdown_section;
    use crate::deliver;
    use crate::scene;

    #[test]
    fn test_usage() {
        let capture = deliver::Capture {
            path: PathBuf::from("out/ui__rest.png"),
            artifact: deliver::Artifact {
                scene: String::from("ui"),
                label: String::from("rest"),
                fingerprint: String::from("ui|rest"),
                pixel_hash: String::from("h"),
                media: deliver::Media::Png,
                kind: scene::Kind::System,
            },
            status: deliver::Status::New,
        };
        let mut empty = String::new();
        write_markdown_section(&mut empty, "New states", &[]);
        assert_eq!(empty, "");
        let mut out = String::new();
        write_markdown_section(&mut out, "New states", &[&capture]);
        assert_eq!(out, "## New states\n\n- `rest` (ui__rest.png)\n\n");
    }
}
