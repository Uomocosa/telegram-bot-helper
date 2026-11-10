use crate::Error;

#[must_use]
pub fn with_causes(error: &Error) -> String {
    let mut message = error.to_string();
    let mut cause = std::error::Error::source(error);
    while let Some(inner) = cause {
        message.push_str(": ");
        message.push_str(&inner.to_string());
        cause = inner.source();
    }
    message
}

#[cfg(test)]
mod tests {
    use crate::Error;

    #[test]
    fn test_usage() {
        let error = Error::Read {
            path: "missing.png".to_string(),
            source: std::io::Error::other("gone"),
        };
        assert_eq!(error.with_causes(), "telegram read missing.png: gone");
    }
}
