use crate::Error;
use crate::deliver;

pub fn fail_always(_items: &[&deliver::Capture], _caption: &str) -> Result<usize, Error> {
    Err(Error::Scene(String::from("injected failure")))
}

#[cfg(test)]
mod tests {
    use super::fail_always;

    #[test]
    fn test_usage() {
        assert!(fail_always(&[], "caption").is_err());
    }
}
