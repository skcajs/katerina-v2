use katerina::light::Light;
use katerina::object::Object;
use katerina::tuple::Tuple;
use katerina::material::Material;
use katerina::world::World;
use katerina::camera::Camera;
use katerina::matrix::Matrix;
use katerina::transformation::Transformation;

fn main() {

    let floor = Object::plane()
        .with_transform(Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(Material::new()
            .with_color(Tuple::color(1.0, 0.9, 0.9))
            .with_specular(0.0)
        );

    let middle = Object::sphere()
        .with_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .with_material(Material::new()
            .with_color(Tuple::color(0.1, 1.0, 0.5))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let right = Object::sphere()
        .with_transform(Matrix::translation(1.5, 0.5, -0.5)
            * Matrix::scaling(0.5, 0.5, 0.5))
        .with_material(Material::new()
            .with_color(Tuple::color(0.5, 1.0, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let left = Object::sphere()
        .with_transform(Matrix::translation(-1.5, 0.33, -0.75)
            * Matrix::scaling(0.33, 0.33, 0.33))
        .with_material(Material::new()
            .with_color(Tuple::color(1.0, 0.8, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));

    let world = World::new()
        .with_objects(vec![floor, middle, right, left])
        .with_lights(vec![light]);

    let camera = Camera::new(800, 400, std::f64::consts::PI / 3.0).with_transform(
        Matrix::view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        )
    );

    let canvas = camera.render(&world);

    canvas.save("plane.ppm").expect("Failed to save the canvas");
}