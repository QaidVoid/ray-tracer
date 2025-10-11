use crate::color::Color;

#[derive(Debug, Default)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); (width * height) as usize],
        }
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = color;
        }
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx]
        } else {
            Color::default()
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::with_capacity((self.width * self.height * 12) as usize);

        // PPM header
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        for y in 0..self.height {
            let mut line = String::new();

            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let pixel = self.pixels[idx];
                let rgb = pixel.to_rgb8();

                for c in rgb {
                    let value = c.to_string();
                    let space_len = if line.is_empty() { 0 } else { 1 };
                    let len = line.len() + space_len + value.len();

                    if len > 70 {
                        ppm.push_str(line.trim_end());
                        ppm.push('\n');
                        line.clear();
                    }

                    if !line.is_empty() {
                        line.push(' ');
                    }
                    line.push_str(&value);
                }
            }

            if !line.is_empty() {
                ppm.push_str(line.trim_end());
                ppm.push('\n');
            }
        }
        ppm.push('\n');
        ppm
    }
}
