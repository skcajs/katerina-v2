use crate::material::Material;
use crate::shape::Shape;
use crate::shapes::sphere::Sphere;
use crate::tuple::Color;

pub fn glass_sphere() -> Shape {
    Shape::Sphere(Sphere::new())
        .with_material(
            Material::new()
                .with_color(Color::color(1.0, 1.0, 1.0))
                .with_ambient(0.0)
                .with_diffuse(0.3)
                .with_specular(0.7)
                .with_shininess(200.0)
                .with_reflectivity(0.5)
                .with_transparency(1.0)
                .with_refractive_index(1.5)
        )
}