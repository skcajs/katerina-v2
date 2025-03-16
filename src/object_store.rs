use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, MutexGuard},
};

use once_cell::sync::Lazy;
use slotmap::SlotMap;

use crate::{
    keys::ObjectKey,
    material::Material,
    matrix::Matrix,
    object::Object,
    shape::Shape,
    shapes::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere,
        test_shape::TestShape,
    }, tuple::Point,
};

pub struct ObjectGuard<'a> {
    guard: MutexGuard<'a, SlotMap<ObjectKey, Object>>,
    key: ObjectKey,
}

static OBJECT_STORE: Lazy<Arc<Mutex<ObjectStore>>> =
    Lazy::new(|| Arc::new(Mutex::new(ObjectStore::new())));

impl<'a> Deref for ObjectGuard<'a> {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.guard[self.key]
    }
}

impl<'a> DerefMut for ObjectGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard[self.key]
    }
}

pub struct ObjectStore {
    pub objects: SlotMap<ObjectKey, Object>,
}

impl ObjectStore {
    pub fn new() -> Self {
        ObjectStore {
            objects: SlotMap::with_key(),
        }
    }

    pub fn add(&mut self, mut object: Object) -> ObjectKey {
        self.objects.insert_with_key(|key| {
            object.key = Some(key);
            object
        })
    }

    pub fn test_shape(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::TestShape(TestShape::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn sphere(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn plane(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Plane(Plane::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cube(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cube(Cube::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cylinder(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cylinder(Cylinder::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cone(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cone(Cone::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn group(&mut self) -> ObjectKey {
        self.objects.insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Group(Group::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    // pub fn set_transform(&mut self, key: ObjectKey, transform: Matrix) {
    //     if let Some(mut object) = self.get(key) {
    //         object.transform = transform;
    //     }
    // }

    // pub fn set_material(&mut self, key: ObjectKey,  material: Material) {
    //     if let Some(mut object) = self.get(key) {
    //         object.material = material;
    //     }
    // }

    // pub fn add_child(&mut self, parent_key: ObjectKey, child_key: ObjectKey) {

    //     if let Some(mut parent) = self.get(parent_key) {
    //         if let Shape::Group(ref mut group) = parent.shape {
    //             group.children.push(child_key);
    //         }
    //     }

    //     if let Some(mut child) = self.get(child_key) {
    //         child.parent = Some(parent_key);
    //     }
    // }

    pub fn get_children(&self, key: ObjectKey) -> Vec<ObjectKey> {
        if let Some(object) = self.get(key) {
            if let Shape::Group(ref group) = object.shape {
                group.children.clone()   
            } else {
                panic!("Object is not a group");
            }
        } else {
            panic!("Object not found");
        }
    }

    pub fn world_to_object(&self, key: ObjectKey, world_point: &Point) -> Point {
        let (parent_key, transform) = {
            if let Some(object) = self.get(key) {
                (object.parent, object.transform.inverse())
            } else {
                return *world_point;
            }
        };
    
        if let Some(parent_key) = parent_key {
            let point = self.world_to_object(parent_key, world_point);
            transform * point
        } else {
            transform * *world_point
        }
    }

    // pub fn normal_to_world(&self, object_normal: &Vector) -> Vector {
    //     let objects = ObjectStore::get_object_store();
    //     let mut world_normal = self.transform.inverse().transpose() * *object_normal;

    //     world_normal.3 = 0.0;

    //     world_normal = world_normal.normalize();

    //     if let Some(parent_key) = self.parent {
    //         if let Some(parent) = objects.get(parent_key) {
    //             world_normal = parent.normal_to_world(&world_normal);
    //         }
    //     }

    //     world_normal
    // }

    pub fn get(&self, key: ObjectKey) -> Option<&Object> {
        if self.objects.contains_key(key) {
            Some(self.objects.get(key).unwrap())
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: ObjectKey) -> Option<&mut Object> {
        if self.objects.contains_key(key) {
            Some(self.objects.get_mut(key).unwrap())
        } else {
            None
        }
    }

    pub fn get_object_store() -> std::sync::MutexGuard<'static, ObjectStore> {
        OBJECT_STORE.lock().unwrap()
    }
}
