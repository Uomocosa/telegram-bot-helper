use crate::Error;
use crate::deliver;

pub fn no_credentials(items: &[&deliver::Capture], _caption: &str) -> Result<usize, Error> {
    if items.is_empty() {
        return Err(Error::Scene(String::from("empty chunk")));
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::no_credentials;
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
        assert_eq!(no_credentials(&[&item], "caption").unwrap(), 0);
    }
}
