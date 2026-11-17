use crate::deliver;

pub fn append_line(log: &deliver::DeliveryLog, event: &str, fields: &[(&str, String)]) {
    let mut line = deliver::DeliveryLog::utc_timestamp();
    line.push_str("\trun=");
    line.push_str(&deliver::DeliveryLog::sanitized_field(&log.run_id));
    line.push_str("\tscene=");
    line.push_str(&deliver::DeliveryLog::sanitized_field(&log.scene));
    line.push_str("\tevent=");
    line.push_str(&deliver::DeliveryLog::sanitized_field(event));
    for (key, value) in fields {
        line.push('\t');
        line.push_str(&deliver::DeliveryLog::sanitized_field(key));
        line.push('=');
        line.push_str(&deliver::DeliveryLog::sanitized_field(value));
    }
    tracing::info!(target: "lele_preview::delivery", "{line}");
    let Some(parent) = log.path.parent() else {
        tracing::warn!(
            "delivery log has no parent directory: {}",
            log.path.display()
        );
        return;
    };
    if let Err(error) = std::fs::create_dir_all(parent) {
        tracing::warn!("delivery log directory unavailable: {error}");
        return;
    }
    let opened = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log.path);
    match opened {
        Ok(mut file) => {
            use std::io::Write;
            if let Err(error) = writeln!(file, "{line}") {
                tracing::warn!("delivery log write failed: {error}");
            }
        }
        Err(error) => {
            tracing::warn!("delivery log unavailable: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::append_line;
    use crate::deliver;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let log = deliver::DeliveryLog::from_out_dir_and_scene(dir.path(), "ui");
        append_line(&log, "skip", &[("why", String::from("dry_run"))]);
        let text = std::fs::read_to_string(&log.path).unwrap();
        assert!(text.contains("event=skip"));
        assert!(text.contains("why=dry_run"));
        assert!(text.ends_with('\n'));
    }
}
