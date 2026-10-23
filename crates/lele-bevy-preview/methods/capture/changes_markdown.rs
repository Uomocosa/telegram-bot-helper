use std::fmt::Write;

use crate::deliver;
use crate::deliver::basic::enums::Status;

#[must_use]
pub fn changes_markdown(captures: &[deliver::Capture]) -> String {
    let mut out = String::from("# UI preview changes\n\n");
    let changed: Vec<&deliver::Capture> = captures
        .iter()
        .filter(|capture| capture.status == Status::Changed)
        .collect();
    let added: Vec<&deliver::Capture> = captures
        .iter()
        .filter(|capture| capture.status == Status::New)
        .collect();
    let unchanged = captures
        .len()
        .saturating_sub(changed.len())
        .saturating_sub(added.len());
    let _ = writeln!(
        out,
        "{} changed, {} new, {unchanged} unchanged.\n",
        changed.len(),
        added.len()
    );
    if changed.is_empty() && added.is_empty() {
        out.push_str("No UI state changed; nothing was sent.\n");
        return out;
    }
    section(&mut out, "New states", &added);
    section(&mut out, "Changed (same state, different pixels)", &changed);
    out
}

// needed helper: one markdown list of captures, each naming the artifact it points at
fn section(out: &mut String, title: &str, captures: &[&deliver::Capture]) {
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
    use super::changes_markdown;

    #[test]
    fn test_usage() {
        let _ = changes_markdown;
    }
}
