use katerina::tuple::Tuple;
use katerina::canvas::Canvas;
use katerina::matrix::Matrix;
use katerina::transformation::Transformation;

fn main() {
    let mut canvas = Canvas::new(512, 512);
    let white = Tuple::color(1.0, 1.0, 1.0);
    let center = Tuple::point(256.0, 256.0, 0.0);
    let radius = 200.0;

    for hour in 0..12 {
        let angle = (hour as f64) * std::f64::consts::PI / 6.0;
        let rotation = Matrix::rotation_z(angle);
        let hour_point = rotation * Tuple::point(0.0, radius, 0.0);
        let x = hour_point.0 + center.0;
        let y = hour_point.1 + center.1;
        let hour_point = Tuple::point(x, y, 0.0);
        canvas.write_pixel(hour_point.0 as usize, hour_point.1 as usize, white);
    }

    canvas.save("clock.ppm").expect("Failed to save the canvas");
}