use crate::Tuples;

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Tuples>,
}

impl Default for Canvas {
    fn default() -> Self {
        Canvas {width: 0, height: 0, pixels: Vec::new() }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {width, height, pixels: vec![Tuples::color(0.0, 0.0, 0.0); width * height] }
    }
}

// Methods
// top left pixel = 0,0
impl Canvas {
    pub fn get_pixels(&self) -> &Vec<Tuples> {
        &self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuples) {
        let pixel = self.pixel_at_mut(x, y);
        *pixel = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Tuples {
        &self.pixels[self.get_pixel_offset(x, y)]
    }

    pub fn pixel_at_mut(&mut self, x: usize, y: usize) -> &mut Tuples {
        let pos = self.get_pixel_offset(x, y);
        &mut self.pixels[pos]
    }

    fn get_pixel_offset(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }
}