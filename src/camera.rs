use std::sync::Arc;

use rayon::prelude::*;

use crate::colour::{Colour, write_colour};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, random_in_unit_disk};
use crate::{INFINITY, camera, degrees_to_radians, random_f64};

#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: i32,       // Rendered image width in pixels
    pub samples_per_pixel: i32, // Num of random samples per pixel (anti-aliasing)
    pub max_depth: i32,         // Max num of ray bounces

    pub vfov: f64,        // Vertical FOV (in degrees)
    pub lookfrom: Point3, // Point camera is looking from
    pub lookat: Point3,   // Point camera is looking at
    pub vup: Vec3,        // Camera "up" direction

    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Camera lookfrom to plane of perfect focus

    image_height: i32,
    pixel_sample_scale: f64,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // Frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::zero(),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: Arc<dyn Hittable>) {
        self.initialize();

        let image_size = self.image_width * self.image_height;
        eprintln!("\rConstructing image with {} pixels", image_size);
        let mut image = vec![Colour::zero(); image_size as usize];
        (0..image_size)
            .into_par_iter()
            .map(|index| {
                let i = index % self.image_width;
                let j = index / self.image_width;

                if index % 1000 == 0 {
                    eprintln!("\rRendered: {}/{}", index, image_size);
                }

                let mut pixel_colour = Colour::zero();
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_colour += Camera::ray_colour(&r, self.max_depth, world.clone());
                }
                pixel_colour
            })
            .collect_into_vec(&mut image);

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for pixel_colour in image {
            write_colour(self.pixel_sample_scale * pixel_colour);
        }
        eprintln!("\rDone!");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.centre = self.lookfrom;

        // Camera
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // calculate u, v, w basis vectors
        self.w = (self.lookfrom - self.lookat).unit();
        self.u = self.vup.cross(self.w).unit();
        self.v = self.w.cross(self.u);

        // viewport edge vectors
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // pixel to pixel delta vectors
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // upper left pixel
        let viewport_upper_left =
            self.centre - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // defocus disk basis vectors
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_colour(ray: &Ray, depth: i32, world: Arc<dyn Hittable>) -> Colour {
        // Hit ray bounce limit
        if depth <= 0 {
            return Colour::zero();
        }

        let mut rec = HitRecord::default();
        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Colour::default();
            // WARN may panic in unwrap()
            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(ray, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_colour(&scattered, depth - 1, world.clone());
            }
            return Colour::zero();
        }

        // Background colour
        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x) * self.pixel_delta_u
            + (j as f64 + offset.y) * self.pixel_delta_v;

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.centre + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
