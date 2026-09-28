use crate::{
    constants::{BLUE_COLOR, WHITE_COLOR},
    interval::Interval,
    ray::Ray,
    shapes::hittable::Hittable,
    vec3::{RGB, Vec3},
};
use rander::Rng;
use std::io::{self, Write};

/// Recursive ray marcher with:
/// * depth limiting (avoids stack overflow)
/// * shadow-acne tolerance (0.001 t-min instead of 0.0)
/// * Lambertian scattering (`normal + random_unit_vector`)
/// * 0.5 reflectance (i.e. 50% gray diffuse)
#[must_use]
pub fn ray_color(ray: &Ray, world: &impl Hittable, depth: usize, rng: &mut Rng) -> RGB {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return RGB::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        // True Lambertian: normal + random unit vector
        let direction = hit_record.normal + Vec3::random_unit_vector(rng);
        return 0.5
            * ray_color(
                &Ray::new(hit_record.point, direction),
                world,
                depth - 1,
                rng,
            );
    }

    let a = 0.5 * (ray.direction().unit().y() + 1.0);

    (1.0 - a) * WHITE_COLOR + a * BLUE_COLOR
}

#[inline]
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

#[inline]
pub fn write_color<W: Write>(output: &mut W, pixel: &RGB) -> io::Result<()> {
    const INTENSITY: Interval = Interval::new(0.000, 0.999);

    // Apply a linear-to-gamma transform for gamma 2.
    let r = linear_to_gamma(pixel.x());
    let g = linear_to_gamma(pixel.y());
    let b = linear_to_gamma(pixel.z());

    output.write_all(&[
        (255.999 * INTENSITY.clamp(r)) as u8,
        (255.999 * INTENSITY.clamp(g)) as u8,
        (255.999 * INTENSITY.clamp(b)) as u8,
    ])
}
