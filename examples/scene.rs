use katerina::light::Light;
use katerina::tuple::Tuple;
use katerina::shape::Shape;
use katerina::material::Material;
use katerina::world::World;
use katerina::camera::Camera;
use katerina::matrix::Matrix;
use katerina::transformation::Transformation;

fn main() {

    let floor = Shape::sphere()
        .with_transform(Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(Material::new()
            .with_color(Tuple::color(1.0, 0.9, 0.9))
            .with_specular(0.0)
        );

    let left_wall = Shape::sphere()
        .with_transform(Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(floor.get_material().clone());

    let right_wall = Shape::sphere()
        .with_transform(Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0))
        .with_material(floor.get_material().clone());

    let middle = Shape::sphere()
        .with_transform(Matrix::translation(-0.5, 1.0, 0.5))
        .with_material(Material::new()
            .with_color(Tuple::color(0.1, 1.0, 0.5))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let right = Shape::sphere()
        .with_transform(Matrix::translation(1.5, 0.5, -0.5)
            * Matrix::scaling(0.5, 0.5, 0.5))
        .with_material(Material::new()
            .with_color(Tuple::color(0.5, 1.0, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let left = Shape::sphere()
        .with_transform(Matrix::translation(-1.5, 0.33, -0.75)
            * Matrix::scaling(0.33, 0.33, 0.33))
        .with_material(Material::new()
            .with_color(Tuple::color(1.0, 0.8, 0.1))
            .with_diffuse(0.7)
            .with_specular(0.3)
        );

    let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));

    let world = World::new()
        .with_objects(vec![floor, left_wall, right_wall, middle, right, left])
        .with_lights(vec![light]);

    let camera = Camera::new(800, 400, std::f64::consts::PI / 3.0).with_transform(
        Matrix::view_transform(
            Tuple::point(0.0, 1.5, -5.0),
            Tuple::point(0.0, 1.0, 0.0),
            Tuple::vector(0.0, 1.0, 0.0),
        )
    );

    let canvas = camera.render(&world);

    canvas.save("scene.ppm").expect("Failed to save the canvas");
}