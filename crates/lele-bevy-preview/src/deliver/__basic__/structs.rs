#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: u32,
    pub is_drift: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixelDiff {
    pub changed_pixels: u32,
    pub regions: Vec<ChangedRegion>,
}
