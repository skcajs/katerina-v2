use crate::tuple::{Point, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    position: Point,
    intensity: Color,
}

impl Light {
    pub fn new(position: Point, intensity: Color) -> Light {
        Light { position, intensity }
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Tuple::color(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);
        let light = Light::new(position, intensity);

        assert_eq!(light.position(), position);
        assert_eq!(light.intensity(), intensity);
    }
}