#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    New,
    Changed,
    Same,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Media {
    #[default]
    Png,
    Mp4,
}
