use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{random_unit_vector, reflect};

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected_direction = reflect(ray.direction, rec.normal);
        reflected_direction = reflected_direction.unit() + self.fuzz * random_unit_vector();
        *scattered = Ray::new(rec.p, reflected_direction);
        *attenuation = self.albedo;
        true
    }
}
