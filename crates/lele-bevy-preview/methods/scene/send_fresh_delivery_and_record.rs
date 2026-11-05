use crate::Error;
use crate::deliver;
use crate::preview;
use crate::scene;

const ALBUM_LIMIT: usize = 10;

pub fn send_fresh_delivery_and_record(
    scene: &scene::Scene,
    fresh: &mut deliver::FreshDelivery,
    config: &preview::Config,
    staging_root: &std::path::Path,
) -> Result<usize, Error> {
    let mut order = Vec::with_capacity(
        fresh
            .captures
            .len()
            .saturating_add(usize::from(fresh.clip.is_some())),
    );
    for index in 0..fresh.captures.len() {
        order.push((index, false));
    }
    if fresh.clip.is_some() {
        order.push((0, true));
    }
    let caption = {
        let mut all_refs: Vec<&deliver::Capture> = fresh.captures.iter().collect();
        if let Some(clip) = fresh.clip.as_ref() {
            all_refs.push(clip);
        }
        scene.kind.caption(&scene.name, &all_refs)
    };
    let mut total: usize = 0;
    let resend_all = deliver::Capture::is_resend_all_requested();
    let chunks: Vec<&[(usize, bool)]> = order.chunks(ALBUM_LIMIT).collect();
    for (chunk_index, chunk) in chunks.iter().enumerate() {
        if !resend_all && !fresh.chunk_is_sendable(chunk) {
            scene.drop_positions_staged(fresh, chunk, config, staging_root);
            let refs = fresh.chunk_refs(chunk);
            fresh.log_captures_with_outcome(&refs, "skipped_same", &config.out_dir);
            continue;
        }
        let refs = fresh.chunk_refs(chunk);
        let send_result = (config.send_preview_group)(&refs, &caption);
        drop(refs);
        match send_result {
            Ok(0) => {
                scene.stash_positions_to_unsent(fresh, chunk, config, staging_root);
                let pending = fresh.chunk_refs(chunk);
                fresh.log_captures_with_outcome(&pending, "no_credentials", &config.out_dir);
            }
            Ok(sent) => {
                total = total.saturating_add(sent);
                scene.settle_positions_sent(fresh, chunk, config, staging_root, resend_all);
                let updated = fresh.chunk_refs(chunk);
                fresh.log_captures_with_outcome(&updated, "sent", &config.out_dir);
            }
            Err(error) => {
                scene.stash_positions_to_unsent(fresh, chunk, config, staging_root);
                let failed = fresh.chunk_refs(chunk);
                fresh.log_captures_with_outcome(&failed, "failed", &config.out_dir);
                for rest in chunks.iter().skip(chunk_index.saturating_add(1)) {
                    scene.stash_positions_to_unsent(fresh, rest, config, staging_root);
                    let rest_refs = fresh.chunk_refs(rest);
                    fresh.log_captures_with_outcome(&rest_refs, "failed", &config.out_dir);
                }
                return Err(error);
            }
        }
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
