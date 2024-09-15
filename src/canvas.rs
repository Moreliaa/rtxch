use crate::Tuples;
use crate::utils::color_to_256;

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

    pub fn write_pixel(&mut self, x: usize, y: usize, color: &Tuples) {
        let pixel = self.pixel_at_mut(x, y);
        pixel.x = color.x;
        pixel.y = color.y;
        pixel.z = color.z;
    }

    pub fn clear(&mut self, clear_color: &Tuples) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.write_pixel(x, y, clear_color);
            }
        }
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

    pub fn canvas_to_ppm(&self) -> String {
        let mut output: Vec<String> = vec![];
        output.push(String::from("P3\n"));
        let canvas_dim = format!("{} {}\n", self.width, self.height);
        output.push(String::from(&canvas_dim));
        output.push(String::from("255\n"));
        
        let max_characters = 70;

        let newline = String::from("\n");
        let space = String::from(" ");

        for y in 0..self.height {
            let mut characters_in_line = 0;
            for x in 0..self.width {
                if x > 0 {
                    characters_in_line += 1;
                    if characters_in_line > max_characters {
                        output.push(newline.clone());
                        characters_in_line = 0;
                    } else {
                        output.push(space.clone());
                    }
                }
                let pixel = self.pixel_at(x, y);
                let colors = [pixel.x, pixel.y, pixel.z];
                for (i, c) in colors.iter().enumerate() {
                    let c_str = color_to_256(*c).to_string();
                    
                    characters_in_line += c_str.len();
                    if characters_in_line > max_characters {
                        while output.last().unwrap() == &" " {
                            output.pop();
                        }
                        output.push(newline.clone());
                        characters_in_line = c_str.len();
                    }
                    output.push(c_str);

                    if i < colors.len() - 1 {
                        characters_in_line += 1;
                        if characters_in_line > max_characters {
                            output.push(newline.clone());
                            characters_in_line = 0;
                        } else {
                            output.push(space.clone());
                        }
                    }
                }
            }
            output.push(newline.clone());
        }
        output.join("")
    }
}