use std::collections::HashMap;

use crate::Error;
use crate::deliver;
use crate::report::Report;
use crate::scene;

pub fn try_from_report_and_baseline(
    report: &Report,
    out_dir: &std::path::Path,
    fps: u32,
) -> Result<deliver::FreshDelivery, Error> {
    let mut captures = Vec::new();
    let mut olds: HashMap<String, String> = HashMap::new();
    let mut present = false;
    for capture in &report.captures {
        let delivered = capture.artifact.delivered_path(out_dir);
        let old = if delivered.exists() {
            present = true;
            deliver::Artifact::hash_png_pixels(&delivered).map_err(|error| match error {
                Error::Image { path, source } => Error::Image { path, source },
                Error::Io(source) => Error::DeliveredPreviewUnreadable {
                    path: delivered.clone(),
                    source,
                },
                other => other,
            })?
        } else {
            String::new()
        };
        olds.insert(capture.artifact.fingerprint.clone(), old);
        let status = capture.artifact.status_against_baseline(out_dir)?;
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
            let delivered = previous.artifact.delivered_path(out_dir);
            let old = if delivered.exists() {
                let frames_dir =
                    deliver::Artifact::delivered_frames_dir(out_dir, &previous.artifact.scene);
                if frames_dir.is_dir() {
                    present = true;
                    let mut names: Vec<std::ffi::OsString> = std::fs::read_dir(&frames_dir)
                        .map_err(|source| Error::DeliveredPreviewUnreadable {
                            path: frames_dir.clone(),
                            source,
                        })?
                        .map(|entry| {
                            entry
                                .map_err(|source| Error::DeliveredPreviewUnreadable {
                                    path: frames_dir.clone(),
                                    source,
                                })
                                .map(|entry| entry.file_name())
                        })
                        .collect::<Result<Vec<std::ffi::OsString>, Error>>()?;
                    names.sort();
                    let mut frame_hashes = Vec::with_capacity(names.len());
                    for name in &names {
                        let path = frames_dir.join(name);
                        frame_hashes.push(deliver::Artifact::hash_png_pixels(&path)?);
                    }
                    scene::Scene::clip_hash(fps, &frame_hashes)
                } else {
                    String::new()
                }
            } else {
                String::new()
            };
            olds.insert(previous.artifact.fingerprint.clone(), old);
            let status = previous
                .artifact
                .clip_status_against_baseline(out_dir, fps)?;
            Ok(deliver::Capture {
                path: previous.path.clone(),
                artifact: previous.artifact.clone(),
                status,
            })
        })
        .transpose()?;
    let baseline_state = if present {
        String::from("present")
    } else {
        String::from("missing")
    };
    Ok(deliver::FreshDelivery {
        captures,
        clip,
        old_hashes: olds,
        baseline_state,
    })
}

#[cfg(test)]
mod tests {
    use super::try_from_report_and_baseline;

    #[test]
    fn test_usage() {
        let _ = try_from_report_and_baseline;
    }
}
