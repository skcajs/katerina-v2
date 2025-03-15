use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex, MutexGuard}};

use once_cell::sync::Lazy;
use slotmap::SlotMap;

use crate::{keys::ObjectKey, material::Material, matrix::Matrix, object::Object, shape::Shape, shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere, test_shape::TestShape}};

struct ObjectGuard<'a> {
    guard: MutexGuard<'a, SlotMap<ObjectKey, Object>>,
    key: ObjectKey,
}

static OBJECT_STORE: Lazy<Arc<Mutex<ObjectStore>>> = Lazy::new(|| Arc::new(Mutex::new(ObjectStore::new())));

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
    pub objects: Mutex<SlotMap<ObjectKey, Object>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        ObjectStore {
            objects: Mutex::new(SlotMap::with_key()),
        }
    }

    fn new_object() -> Object {
        Object {
            key: None,
            shape: Shape::TestShape(TestShape::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        }
    }

    pub fn test_shape(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::TestShape(TestShape::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn sphere(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn plane(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Plane(Plane::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cube(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cube(Cube::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cylinder(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cylinder(Cylinder::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn cone(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Cone(Cone::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn group(&self) -> ObjectKey {
        self.objects.lock().unwrap().insert_with_key(|key| Object {
            key: Some(key),
            shape: Shape::Group(Group::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        })
    }

    pub fn get(&self, key: ObjectKey) -> Option<ObjectGuard> {
        let guard = self.objects.lock().unwrap();
        if guard.contains_key(key) {
            Some(ObjectGuard { guard, key })
        } else {
            None
        }
    }

    pub fn get_object_store() -> std::sync::MutexGuard<'static, ObjectStore> {
        OBJECT_STORE.lock().unwrap()
    }

    // pub fn get_object(key: ObjectKey) -> Option<ObjectGuard<'static>> {
    //     let guard = OBJECT_STORE.lock().unwrap().objects;
    //     if guard.lock().unwrap().contains_key(key) {
    //         Some(ObjectGuard { guard: guard.lock().unwrap(), key })
    //     } else {
    //         None
    //     }
    // }

}