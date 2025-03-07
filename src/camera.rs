use std::time::Instant;
use rayon::prelude::*;

use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};

pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let transform = Matrix::identity();
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let half_height;
        let half_width;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        };
        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            pixel_size,
            transform,
        }
    }

    pub fn with_transform(mut self, transform: Matrix) -> Camera {
        self.transform = transform;
        self
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> crate::canvas::Canvas {
        let start = Instant::now();

        let mut image = Canvas::new(self.hsize, self.vsize);
        image.pixels_mut().par_chunks_mut(self.hsize).enumerate().for_each(|(y, row)| {
            for (x, pixel) in row.iter_mut().enumerate() {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                *pixel = color;
            }
        });

        let duration = start.elapsed();
        println!("Render time: {:.2?}", duration);

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use crate::transformation::Transformation;
    use crate::world::World;

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::PI / 2.0);
        let delta = 1e-6;
        assert!((c.pixel_size - 0.01).abs() < delta);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::PI / 2.0);
        let delta = 1e-6;
        assert!((c.pixel_size - 0.01).abs() < delta);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        let delta = 1e-6;
        assert!((r.origin.0 - 0.0).abs() < delta);
        assert!((r.origin.1 - 0.0).abs() < delta);
        assert!((r.origin.2 - 0.0).abs() < delta);
        assert!((r.direction.0 - 0.0).abs() < delta);
        assert!((r.direction.1 - 0.0).abs() < delta);
        assert!((r.direction.2 + 1.0).abs() < delta);
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        let delta = 1e-4;
        assert!((r.origin.0 - 0.0).abs() < delta);
        assert!((r.origin.1 - 0.0).abs() < delta);
        assert!((r.origin.2 - 0.0).abs() < delta);
        assert!((r.direction.0 - 0.66519).abs() < delta);
        assert!((r.direction.1 - 0.33259).abs() < delta);
        assert!((r.direction.2 + 0.66851).abs() < delta);
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
        c.transform = Matrix::rotation_y(std::f64::consts::PI / 4.0) * Matrix::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        let delta = 1e-6;
        assert!((r.origin.0 - 0.0).abs() < delta);
        assert!((r.origin.1 - 2.0).abs() < delta);
        assert!((r.origin.2 - -5.0).abs() < delta);
        assert!((r.direction.0 - 2f64.sqrt() / 2.0).abs() < delta);
        assert!((r.direction.1 - 0.0).abs() < delta);
        assert!((r.direction.2 + 2f64.sqrt() / 2.0).abs() < delta);
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, std::f64::consts::PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = Transformation::view_transform(from, to, up);
        let image = c.render(&w);
        let pixel = image.pixel_at(5, 5);
        let delta = 1e-5;
        assert!((pixel.0 - 0.38066).abs() < delta);
        assert!((pixel.1 - 0.47583).abs() < delta);
        assert!((pixel.2 - 0.2855).abs() < delta);
    }
}