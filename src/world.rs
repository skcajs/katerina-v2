use crate::{color::Colors, 
    intersection::{Intersection, Record}, 
    intersections::Intersections, 
    light::Light, 
    material::Material, 
    matrix::Matrix, 
    ray::Ray,
    shape::Shape, 
    transformation::Transformation, 
    tuple::{Color, Tuple}};

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

    pub fn shade_hit(&self, record: &Record, depth: usize) -> Tuple {
        let mut surface: Color = Colors::black();
        for light in &self.lights {
            surface = surface + record.object.get_material().lighting(
                &record.object,
                light,
                record.over_point,
                record.eyev,
                record.normalv,
                self.is_shadowed(&record.over_point),
            );
        }

        let reflected = self.reflected_color(record, depth);
        let refracted = self.refracted_color(record, depth);

        let material = record.object.get_material();

        if material.reflectivity > 0. && material.transparency > 0. {
            let reflectance = record.schlick;
            return surface 
                + reflected * reflectance 
                + refracted * (1. - reflectance);
        }

        surface 
        + self.reflected_color(record, depth) 
        + self.refracted_color(record, depth)

    }

    pub fn color_at(&self, ray: &Ray, depth: usize) -> Tuple {
        let xs = self.intersect(ray);
        match xs.hit() {
            Some(hit) => {
                let record = hit.prepare_computations(ray, &vec![]);
                self.shade_hit(&record, depth)
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

    pub fn reflected_color(&self, record: &Record, depth: usize) -> Color {
        if depth <= 0 {
            return Colors::black();
        }

        if record.object.get_material().reflectivity == 0. {
            return Colors::black();
        }

        let reflect_ray = Ray::new(record.over_point, record.reflectv);
        let color = self.color_at(&reflect_ray, depth - 1);

        color * record.object.get_material().reflectivity
    }

    pub fn refracted_color(&self, record: &Record, depth: usize) -> Color {
        if depth <= 0 {
            return Colors::black();
        }

        if record.object.get_material().transparency == 0. {
            return Colors::black();
        }

        let n_ratio = record.n1 / record.n2;
        let cos_i = record.eyev.dot(record.normalv);
        let sin2_t = n_ratio.powi(2) * (1. - cos_i.powi(2));

        if sin2_t > 1. {
            return Colors::black();
        }

        let cos_t = (1. - sin2_t).sqrt();
        let direction = record.normalv * (n_ratio * cos_i - cos_t) - record.eyev * n_ratio;
        let refract_ray = Ray::new(record.under_point, direction);
 
        self.color_at(&refract_ray, depth - 1) * record.object.get_material().transparency
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, pattern::Pattern, ray::Ray, transformation::Transformation, tuple::Tuple};

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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = world.shade_hit(&comps, 4);
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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = world.shade_hit(&comps, 4);
        let delta = 0.0001;
        assert!((c.0 - 0.90498).abs() < delta);
        assert!((c.1 - 0.90498).abs() < delta);
        assert!((c.2 - 0.90498).abs() < delta);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let c = world.color_at(&r, 4);
        assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let c = world.color_at(&r, 4);
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
        let c = world.color_at(&r, 4);
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
        let comps = i.prepare_computations(&r, &vec![]);
        let c = world.shade_hit(&comps, 4);
        let delta = 0.00001;
        assert!((c.0 - 0.1).abs() < delta);
        assert!((c.1 - 0.1).abs() < delta);
        assert!((c.2 - 0.1).abs() < delta);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let world = World::default_world();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = &world.objects[1];
        let shape = shape.with_material(Material::new().with_ambient(1.0));
        let i = Intersection::new(1.0, shape.clone());
        let comps = i.prepare_computations(&r, &vec![]);
        let color = world.reflected_color(&comps, 4);
        assert_eq!(color, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut world = World::default_world();
        let shape = Shape::plane().with_material(Material::new().with_reflectivity(0.5)).with_transform(Matrix::translation(0.0, -1.0, 0.0));
        world.add_object(shape);
        let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        let i = Intersection::new(2_f64.sqrt(), world.objects[2].clone());
        let comps = i.prepare_computations(&r, &vec![]);
        let color = world.reflected_color(&comps, 4);
        let delta = 0.0001;
        assert!((color.0 - 0.19032).abs() < delta);
        assert!((color.1 - 0.2379).abs() < delta);
        assert!((color.2 - 0.14274).abs() < delta);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut world = World::default_world();
        let shape = Shape::plane().with_material(Material::new().with_reflectivity(0.5)).with_transform(Matrix::translation(0.0, -1.0, 0.0));
        world.add_object(shape);
        let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        let i = Intersection::new(2_f64.sqrt(), world.objects[2].clone());
        let comps = i.prepare_computations(&r, &vec![]);
        let color = world.shade_hit(&comps, 4);
        let delta = 0.0001;
        assert!((color.0 - 0.87677).abs() < delta);
        assert!((color.1 - 0.92436).abs() < delta);
        assert!((color.2 - 0.82918).abs() < delta);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut world = World::new();
        world.add_lights(vec![Light::new(Tuple::point(0.0, 0.0, 0.0), Tuple::color(1.0, 1.0, 1.0))]);
        let lower = Shape::plane().with_material(Material::new().with_reflectivity(1.0)).with_transform(Matrix::translation(0.0, -1.0, 0.0));
        let upper = Shape::plane().with_material(Material::new().with_reflectivity(1.0)).with_transform(Matrix::translation(0.0, 1.0, 0.0));
        world.add_objects(vec![lower, upper]);
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        world.color_at(&r, 4);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let world = World::default_world();
        let shape = &world.objects[0];
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = world.refracted_color(&comps, 5);
        assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let world = World::default_world();
        let shape = &world.objects[0]
            .with_material(Material::new()
            .with_transparency(1.0)
            .with_refractive_index(1.5)
        );
        
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape.clone()),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = world.refracted_color(&comps, 0);
        assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let world = World::default_world();
        let shape = &world.objects[0]
            .with_material(Material::new()
            .with_transparency(1.0)
            .with_refractive_index(1.5)
        );
        
        let r = Ray::new(Tuple::point(0.0, 0.0, 2_f64.sqrt() / 2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-2_f64.sqrt() / 2.0, shape.clone()),
            Intersection::new(2_f64.sqrt() / 2.0, shape.clone()),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let c = world.refracted_color(&comps, 5);
        assert_eq!(c, Tuple::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let world = World::default_world();
        let a = &world.objects[0]
            .with_material(Material::new()
            .with_ambient(1.0)
            .with_pattern(Pattern::test_pattern())
        );

        let b = &world.objects[1]
            .with_material(Material::new()
            .with_transparency(1.0)
            .with_refractive_index(1.5)
        );
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.1), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-0.9899, a.clone()),
            Intersection::new(-0.4899, b.clone()),
            Intersection::new(0.4899, b.clone()),
            Intersection::new(0.9899, a.clone()),
        ];
        let comps = xs[2].prepare_computations(&r, &xs);
        let c = world.refracted_color(&comps, 5);
        let delta = 0.01;
        assert!((c.0 - 0.08).abs() < delta);
        assert!((c.1 - 0.1).abs() < delta);
        assert!((c.2 - 0.05725).abs() < delta);
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut world = World::default_world();
        let floor = Shape::plane()
            .with_transform(Matrix::translation(0.0, -1.0, 0.0))
            .with_material(Material::new().with_transparency(0.5).with_refractive_index(1.5));
        let ball = Shape::sphere()
            .with_transform(Matrix::translation(0.0, -3.5, -0.5))
            .with_material(Material::new().with_color(Tuple::color(1.0, 0.0, 0.0)).with_ambient(0.5));
        world.add_objects(vec![floor.clone(), ball]);
        let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        let xs = vec![
            Intersection::new(2_f64.sqrt(), floor),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = world.shade_hit(&comps, 5);
        let delta = 0.0001;
        assert!((c.0 - 0.93642).abs() < delta);
        assert!((c.1 - 0.68642).abs() < delta);
        assert!((c.2 - 0.68642).abs() < delta);
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut world = World::default_world();
        let floor = Shape::plane()
            .with_transform(Matrix::translation(0.0, -1.0, 0.0))
            .with_material(Material::new()
            .with_reflectivity(0.5)
            .with_transparency(0.5)
            .with_refractive_index(1.5));
        let ball = Shape::sphere()
            .with_transform(Matrix::translation(0.0, -3.5, -0.5))
            .with_material(Material::new()
            .with_color(Tuple::color(1.0, 0.0, 0.0))
            .with_ambient(0.5));
        world.add_objects(vec![floor.clone(), ball]);
        let r = Ray::new(Tuple::point(0.0, 0.0, -3.0), Tuple::vector(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        let xs = vec![
            Intersection::new(2_f64.sqrt(), floor),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let c = world.shade_hit(&comps, 5);
        let delta = 0.0001;
        assert!((c.0 - 0.93391).abs() < delta);
        assert!((c.1 - 0.69643).abs() < delta);
        assert!((c.2 - 0.69243).abs() < delta);
    }
}