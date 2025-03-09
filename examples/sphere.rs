use katerina::tuple::Tuple;
use katerina::canvas::Canvas;
use katerina::ray::Ray;
use katerina::object::Object;
use katerina::intersections::Intersections;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Tuple::color(1.0, 0.0, 0.0);
    let shape = Object::sphere();

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(&r);
            if let Some(_) = xs.hit() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.save("sphere.ppm").expect("Failed to save the canvas");
}