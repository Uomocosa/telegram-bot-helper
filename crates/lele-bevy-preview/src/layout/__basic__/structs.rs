#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
    pub width: u32,
    pub height: u32,
}
