use crate::Error;
use crate::deliver;

pub fn fail_on_single_chunk(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let _ = caption;
    if items.len() < 10 {
        return Err(Error::Scene(String::from("injected failure")));
    }
    Ok(items.len())
}

#[cfg(test)]
mod tests {
    use super::fail_on_single_chunk;

    #[test]
    fn test_usage() {
        assert!(fail_on_single_chunk(&[], "caption").is_err());
    }
}
