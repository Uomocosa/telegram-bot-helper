use crate::Error;
use crate::report::Report;

pub fn assert_centered(report: &Report, tolerance: u32) -> Result<(), Error> {
    for capture in &report.captures {
        capture.assert_centered(tolerance)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::assert_centered;

    #[test]
    fn test_usage() {
        let _ = assert_centered;
    }
}
