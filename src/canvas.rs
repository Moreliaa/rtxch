#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Canvas {
    width: u64,
    height: u64
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas {width: 0, height: 0}
    }
}

impl Canvas {
    pub fn new(width: u64, height: u64) -> Canvas {
        Canvas {width, height}
    }
}