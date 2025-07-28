use std::sync::Arc;

use raytracer::camera::Camera;
use raytracer::colour::Colour;
use raytracer::hittable::sphere::Sphere;
use raytracer::hittable::{Hittable, HittableList};
use raytracer::material::Material;
use raytracer::material::dielectric::Dielectric;
use raytracer::material::lambertian::Lambertian;
use raytracer::material::metal::Metal;
use raytracer::vec3::{Point3, Vec3};
use raytracer::{random_f64, random_f64_in_range};

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let centre = Point3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(centre, 0.2, sphere_material.clone()));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzz = random_f64_in_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(centre, 0.2, sphere_material.clone()));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(centre, 0.2, sphere_material.clone()));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    ));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.),
        1.0,
        material2.clone(),
    ));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    ));

    let world = Arc::new(world);

    // Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(world);
}
