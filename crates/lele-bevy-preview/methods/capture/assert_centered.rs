use crate::Error;
use crate::deliver;
use crate::layout;

pub fn assert_centered(capture: &deliver::Capture, tolerance: u32) -> Result<(), Error> {
    let path = capture.path.display().to_string();
    let bounds =
        layout::Bounds::of_image(&capture.path)?.ok_or_else(|| Error::NoContent(path.clone()))?;
    let x = bounds.min_x.saturating_add(bounds.max_x).div_ceil(2);
    let y = bounds.min_y.saturating_add(bounds.max_y).div_ceil(2);
    let expected_x = bounds.width / 2;
    let expected_y = bounds.height / 2;
    if x.abs_diff(expected_x) > tolerance || y.abs_diff(expected_y) > tolerance {
        return Err(Error::NotCentered {
            path,
            x,
            y,
            expected_x,
            expected_y,
            tolerance,
        });
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
