use katerina::tuple::{Tuple, Point, Vector};
use katerina::canvas::Canvas;

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile { position, velocity }
}

fn main() {
    let start = Tuple::point(0.,1.,0.);
    let velocity = Tuple::vector(1.,1.8, 0.).normalize() * 11.25;
    let mut p = Projectile {
        position: start,
        velocity,
    };
    let gravity = Tuple::vector(0., -0.1, 0.);
    let wind = Tuple::vector(-0.01, 0., 0.);
    let e = Environment {
        gravity,
        wind,
    };
    let mut c = Canvas::new(900, 550);
    let color = Tuple::color(1., 0., 0.);
    while p.position.1 > 0. {
        let x = p.position.0.round() as usize;
        let y = c.height - p.position.1.round() as usize;
        c.write_pixel(x, y, color);
        p = tick(&e, p);
    }
    c.save("trajectory.ppm").unwrap();
}