use crate::{color::Colors, 
    intersection::{Intersection, Record}, 
    intersections::Intersections, 
    light::Light, 
    material::Material, 
    matrix::Matrix, 
    ray::Ray,
    shape::Shape, 
    transformation::Transformation, 
    tuple::Tuple};

pub struct World {
    objects: Vec<Shape>,
    lights: Vec<Light>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn with_objects(mut self, objects: Vec<Shape>) -> World {
        self.objects = objects;
        self
    }

    pub fn add_objects(&mut self, objects: Vec<Shape>) {
        self.objects.extend(objects);
    }

    pub fn add_object(&mut self, object: Shape) {
        self.objects.push(object);
    }

    pub fn with_lights(mut self, lights: Vec<Light>) -> World {
        self.lights = lights;
        self
    }

    pub fn add_lights(&mut self, lights: Vec<Light>) {
        self.lights.extend(lights);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn default_world() -> World {
        let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let s1 = Shape::sphere()
            .with_material(Material::new()
            .with_color(Tuple::color(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2));
        let s2 = Shape::sphere()
            .with_transform(Matrix::scaling(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            lights: vec![light],
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];
        for object in &self.objects {
            let mut object_xs = object.intersect(ray);
            xs.append(&mut object_xs);
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn shade_hit(&self, record: &Record) -> Tuple {
        let mut color = Colors::black();
        for light in &self.lights {
            color = color + record.object.get_material().lighting(
                light,
                record.over_point,
                record.eyev,
                record.normalv,
                self.is_shadowed(&record.over_point),
            );
        }
        color
    }

    pub fn color_at(&self, ray: &Ray) -> Tuple {
        let xs = self.intersect(ray);
        match xs.hit() {
            Some(hit) => {
                let record = hit.prepare_computations(ray);
                self.shade_hit(&record)
            }
            None => Colors::black(),
        }
    }

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.lights[0].position() - *point;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*point, direction);
        let xs = self.intersect(&r);
        if let Some(hit) = xs.hit() {
            if hit.t < distance {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, ray::Ray, transformation::Transformation, tuple::Tuple};

    use super::*;

    #[test]
    fn creating_a_world() {
        let world = World::new();
        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Tuple::color(1.0, 1.0, 1.0));
        let s1 = Shape::sphere().with_material(Material::new().with_color(Tuple::color(0.8, 1.0, 0.6)).with_diffuse(0.7).with_specular(0.2));
        let s2 = Shape::sphere().with_transform(Matrix::scaling(0.5, 0.5, 0.5));
        let world = World::default_world();
        assert_eq!(world.lights[0], light);
        assert_eq!(world.objects[0], s1);
        assert_eq!(world.objects[1], s2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = world.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &world.objects[0];
        let i = Intersection::new(4.0, shape.clone());
        let comps = i.prepare_computations(&r);
        let c = world.shade_hit(&comps);
        let delta = 0.00001;
        assert!((c.0 - 0.38066).abs() < delta);
        assert!((c.1 - 0.47583).abs() < delta);
        assert!((c.2 - 0.2855).abs() < delta);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::default_world();
        world.lights = vec![Light::new(Tuple::point(0.0, 0.25, 0.0), Tuple::color(1.0, 1.0, 1.0))];
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &world.objects[1];
        let i = Intersection::new(0.5, shape.clone());
        let comps = i.prepare_computations(&r);
        let c = world.shade_hit(&comps);
        let delta = 0.00001;
        assert!((c.0 - 0.90498).abs() < delta);
        assert!((c.1 - 0.90498).abs() < delta);
        assert!((c.2 - 0.90498).abs() < delta);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = world.color_at(&r);
        assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = world.color_at(&r);
        let delta = 0.00001;
        assert!((c.0 - 0.38066).abs() < delta);
        assert!((c.1 - 0.47583).abs() < delta);
        assert!((c.2 - 0.2855).abs() < delta);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut world = World::default_world();
        world.objects[0].set_material(Material::new().with_ambient(1.0));
        world.objects[1].set_material(Material::new().with_ambient(1.0));
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));
        let c = world.color_at(&r);
        assert_eq!(c, world.objects[1].get_material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::default_world();
        let p = Tuple::point(0.0, 10.0, 0.0);
        assert_eq!(world.is_shadowed(&p), false);
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = World::default_world();
        let p = Tuple::point(10.0, -10.0, 10.0);
        assert_eq!(world.is_shadowed(&p), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = World::default_world();
        let p = Tuple::point(-20.0, 20.0, -20.0);
        assert_eq!(world.is_shadowed(&p), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = World::default_world();
        let p = Tuple::point(-2.0, 2.0, -2.0);
        assert_eq!(world.is_shadowed(&p), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let world = World::new()
            .with_lights(vec![Light::new(Tuple::point(0.0, 0.0, -10.0), Tuple::color(1.0, 1.0, 1.0))])
            .with_objects(vec![
                Shape::sphere(),
                Shape::sphere().with_transform(Matrix::translation(0.0, 0.0, 10.0)),
            ]);
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, world.objects[1].clone());
        let comps = i.prepare_computations(&r);
        let c = world.shade_hit(&comps);
        let delta = 0.00001;
        assert!((c.0 - 0.1).abs() < delta);
        assert!((c.1 - 0.1).abs() < delta);
        assert!((c.2 - 0.1).abs() < delta);
    }
}