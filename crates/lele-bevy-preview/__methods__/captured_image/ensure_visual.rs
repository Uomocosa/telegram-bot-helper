use std::path::Path;

use crate::Error;
use crate::preview;

pub fn ensure_visual(image: &preview::CapturedImage, out: &Path, min: usize) -> Result<(), Error> {
    let distinct = image.distinct_colors();
    if distinct < min {
        return Err(Error::NoVisual {
            path: out.display().to_string(),
            distinct,
            min,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::ensure_visual;

    #[test]
    fn test_usage() {
        let _ = ensure_visual;
    }
}
