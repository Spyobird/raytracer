use crate::vec3::Vec3;

pub type Colour = Vec3;

pub fn write_colour(pixel_colour: Colour) {
    let r = pixel_colour.x;
    let g = pixel_colour.y;
    let b = pixel_colour.z;

    // Translate the [0,1] component values to the byte range [0,255]
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    print!("{} {} {}\n", ir, ig, ib)
}