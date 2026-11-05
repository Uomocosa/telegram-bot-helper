use std::collections::BTreeMap;

use crate::Error;
use crate::deliver;
use crate::preview;
use crate::scene;

const ALBUM_LIMIT: usize = 10;

pub fn send_fresh_delivery_and_record(
    scene: &scene::Scene,
    fresh: &deliver::FreshDelivery,
    filtered: Option<deliver::Manifest>,
    config: &preview::Config,
    crate_name: &str,
) -> Result<usize, Error> {
    let mut all_refs: Vec<&deliver::Capture> = fresh.captures.iter().collect();
    if let Some(clip) = fresh.clip.as_ref() {
        all_refs.push(clip);
    }
    let caption = scene.kind.caption(&scene.name, &all_refs);
    let empty = deliver::Manifest {
        crate_name: crate_name.to_string(),
        artifacts: BTreeMap::new(),
    };
    let mut merged = filtered.unwrap_or(empty);
    merged.crate_name = crate_name.to_string();
    let mut total: usize = 0;
    let resend_all = deliver::Capture::is_resend_all_requested();
    for chunk in all_refs.chunks(ALBUM_LIMIT) {
        let chunk_sendable = resend_all || chunk.iter().any(|capture| capture.is_sendable());
        if !chunk_sendable {
            fresh.log_captures_with_outcome(chunk, "skipped_same", &config.out_dir);
            continue;
        }
        match (config.send_preview_group)(chunk, &caption) {
            Ok(0) => {
                fresh.log_captures_with_outcome(chunk, "no_credentials", &config.out_dir);
            }
            Ok(sent) => {
                total = total.saturating_add(sent);
                for capture in chunk {
                    merged.artifacts.insert(
                        capture.artifact.fingerprint.clone(),
                        capture.artifact.clone(),
                    );
                }
                fresh.log_captures_with_outcome(chunk, "sent", &config.out_dir);
                deliver::Manifest::write_merged_atomically(&config.out_dir, &merged)?;
            }
            Err(error) => {
                fresh.log_captures_with_outcome(chunk, "failed", &config.out_dir);
                return Err(error);
            }
        }
    }
    if total == 0 {
        for capture in &all_refs {
            if capture.is_sendable() {
                merged.artifacts.remove(&capture.artifact.fingerprint);
            }
        }
        deliver::Manifest::write_merged_atomically(&config.out_dir, &merged)?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::send_fresh_delivery_and_record;

    #[test]
    fn test_usage() {
        let _ = send_fresh_delivery_and_record;
    }
}
