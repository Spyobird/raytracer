use std::vec;

use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub mod sphere;

#[derive(Debug, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Some magic is happening here
    pub fn add<T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_range.max;

        for object in self.objects.iter() {
            if object.hit(
                ray,
                Interval::new(t_range.min, closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
            }
        }
        // Only works if temp_record is not updated if object misses
        *rec = temp_record;

        hit_anything
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Interval, rec: &mut HitRecord) -> bool;
}
