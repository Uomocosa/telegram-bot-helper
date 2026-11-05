use std::collections::HashMap;

use crate::Error;
use crate::deliver;
use crate::report::Report;

pub fn try_from_report_and_cache(
    report: &Report,
    out_dir: &std::path::Path,
) -> Result<deliver::FreshDelivery, Error> {
    let mut captures = Vec::new();
    let mut olds: HashMap<String, String> = HashMap::new();
    let mut hit = false;
    for capture in &report.captures {
        let recorded = deliver::SentPreviewHash::load_for(out_dir, &capture.artifact)?;
        if recorded.is_some() {
            hit = true;
        }
        let old = recorded.map_or(String::new(), |sent| sent.to_string());
        olds.insert(capture.artifact.fingerprint.clone(), old);
        let status = capture.artifact.status_in_cache(out_dir)?;
        captures.push(deliver::Capture {
            path: capture.path.clone(),
            artifact: capture.artifact.clone(),
            status,
        });
    }
    let clip = report
        .clip
        .as_ref()
        .map(|previous| -> Result<deliver::Capture, Error> {
            let recorded = deliver::SentPreviewHash::load_for(out_dir, &previous.artifact)?;
            if recorded.is_some() {
                hit = true;
            }
            let old = recorded.map_or(String::new(), |sent| sent.to_string());
            olds.insert(previous.artifact.fingerprint.clone(), old);
            let status = previous.artifact.status_in_cache(out_dir)?;
            Ok(deliver::Capture {
                path: previous.path.clone(),
                artifact: previous.artifact.clone(),
                status,
            })
        })
        .transpose()?;
    let cache_state = if hit {
        String::from("hit")
    } else {
        String::from("missing")
    };
    Ok(deliver::FreshDelivery {
        captures,
        clip,
        old_hashes: olds,
        cache_state,
    })
}

#[cfg(test)]
mod tests {
    use super::try_from_report_and_cache;

    #[test]
    fn test_usage() {
        let _ = try_from_report_and_cache;
    }
}
