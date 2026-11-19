use crate::Error;
use crate::deliver;

pub fn recording_send(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let _ = caption;
    if items.is_empty() {
        return Err(Error::Scene(String::from("empty chunk")));
    }
    Ok(items.len())
}

#[cfg(test)]
mod tests {
    use super::recording_send;
    use crate::deliver;
    use crate::testing::capture;

    #[test]
    fn test_usage() {
        let item = capture(
            "shot.png",
            "rest",
            deliver::Media::Png,
            deliver::Status::New,
        );
        assert_eq!(recording_send(&[&item], "caption").unwrap(), 1);
    }
}
