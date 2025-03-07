use crate::tuple::Tuple;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Tuple::color(0.0, 0.0, 0.0); (width * height) as usize];
        Canvas { width, height, pixels }
    }

    pub fn pixels_mut(&mut self) -> &mut [Tuple] {
        &mut self.pixels
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Tuple) {
        let index = y * self.width + x;
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Tuple {
        let index = y * self.width + x;
        self.pixels[index]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str("P3\n");
        ppm.push_str(&format!("{} {}\n", self.width, self.height));
        ppm.push_str("255\n");

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);
                let r = (pixel.0 * 255.0).round().clamp(0.0, 255.0) as u64;
                let g = (pixel.1 * 255.0).round().clamp(0.0, 255.0) as u64;
                let b = (pixel.2 * 255.0).round().clamp(0.0, 255.0) as u64;
                ppm.push_str(&format!("{} {} {}\n", r, g, b));
            }
        }

        ppm
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        let filepath = format!("./images/{}", filename);
        std::fs::write(filepath, self.to_ppm())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, Tuple::color(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Tuple::color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Tuple::color(1.5, 0.0, 0.0);
        let c2 = Tuple::color(0.0, 0.5, 0.0);
        let c3 = Tuple::color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0");
        assert_eq!(lines[10], "0 128 0");
        assert_eq!(lines[17], "0 0 255");
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    #[test]
    fn saving_a_canvas_to_ppm_file() {
        let mut c = Canvas::new(5, 3);
        let c1 = Tuple::color(1.5, 0.0, 0.0);
        let c2 = Tuple::color(0.0, 0.5, 0.0);
        let c3 = Tuple::color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        c.save("canvas.ppm").unwrap();
        let contents = std::fs::read_to_string("./images/canvas.ppm").unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
        assert_eq!(lines[3], "255 0 0");
        assert_eq!(lines[10], "0 128 0");
        assert_eq!(lines[17], "0 0 255");
    }
}

