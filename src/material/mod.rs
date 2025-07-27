use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}
