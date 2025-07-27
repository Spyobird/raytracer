use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Colour = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}

pub fn write_colour(pixel_colour: Colour) {
    let mut r = pixel_colour.x;
    let mut g = pixel_colour.y;
    let mut b = pixel_colour.z;

    // Linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255]
    const INTENSITY: Interval = Interval::new(0.000, 0.999);
    let ir = (256.0 * INTENSITY.clamp(r)) as i32;
    let ig = (256.0 * INTENSITY.clamp(g)) as i32;
    let ib = (255.0 * INTENSITY.clamp(b)) as i32;

    print!("{} {} {}\n", ir, ig, ib)
}
