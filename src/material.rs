use crate::light::Light;
use crate::tuple::{Tuple, Color};
use crate::color::Colors;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn lighting(&self, light: &Light, position: Tuple, eyev: Tuple, normalv: Tuple, in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity();
        let lightv = (light.position() - position).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);

        let (diffuse, specular) = if light_dot_normal < 0.0 {
            (Color::black(), Color::black())
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0.0 {
                (diffuse, Color::black())
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                let specular = light.intensity() * self.specular * factor;
                (diffuse, specular)
            }
        };

        if in_shadow {
            ambient
        } else {
            ambient + diffuse + specular
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use crate::light::Light;

    #[test]
    fn the_default_material() {
        let m = Material::new();

        assert_eq!(m.color, Color::color(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv, false);

        assert_eq!(result, Tuple::color(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv, false);

        assert_eq!(result, Tuple::color(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv, false);

        let alpha = 1e-4;
        assert!((result.0 - 0.7364).abs() < alpha);
        assert!((result.1 - 0.7364).abs() < alpha);
        assert!((result.2 - 0.7364).abs() < alpha);
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv, false);

        let alpha = 1e-4;
        assert!((result.0 - 1.6364).abs() < alpha);
        assert!((result.1 - 1.6364).abs() < alpha);
        assert!((result.2 - 1.6364).abs() < alpha);
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, 10.0), Tuple::color(1.0, 1.0, 1.0));
        let result = m.lighting(&light, position, eyev, normalv, false);

        assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = Tuple::point(0.0, 0.0, 0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(&light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Tuple::color(0.1, 0.1, 0.1));
    }
}