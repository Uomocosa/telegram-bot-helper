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
    use std::path::Path;

    use super::ensure_visual;
    use crate::Error;
    use crate::testing::captured;

    #[test]
    fn test_usage() {
        let out = Path::new("out.png");
        let image = captured(&[[1, 2, 3, 255], [9, 9, 9, 255], [0, 0, 0, 255]]);
        assert!(ensure_visual(&image, out, 2).is_ok());
        assert!(matches!(
            ensure_visual(&image, out, 4),
            Err(Error::NoVisual {
                distinct: 3,
                min: 4,
                ..
            })
        ));
    }
}
