use crate::Error;
use crate::deliver;

pub fn send_group(items: &[&deliver::Capture], caption: &str) -> Result<usize, Error> {
    let [only] = items else {
        return deliver::Capture::send_album(items, caption);
    };
    let sent = only.send_one(caption)?;
    Ok(usize::from(sent))
}

#[cfg(test)]
mod tests {
    use super::send_group;

    #[test]
    fn test_usage() {
        let _ = send_group;
    }
}
