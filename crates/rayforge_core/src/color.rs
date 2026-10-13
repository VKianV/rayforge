use crate::{
    hittable::{HitRecord, Hittable, hittable_list::HittableList},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use rander::Rng;
use std::io::{self, Write};

/// `RGB` is  aliase of `Vec3`
pub type RGB = Vec3;

/// blue color constant
pub const BLUE_COLOR: RGB = RGB::new(0.5, 0.7, 1.0);

/// Recursive ray marcher with:
/// * depth limiting (avoids stack overflow)
/// * shadow-acne tolerance (0.001 t-min instead of 0.0)
/// * Lambertian scattering (`normal + random_unit_vector`)
/// * 0.5 reflectance (i.e. 50% gray diffuse)
#[must_use]
pub fn ray_color(ray: &Ray, world: &HittableList, max_depth: usize, rng: &mut Rng) -> RGB {
    // No more light is gathered once the bounce limit is exhausted.
    if max_depth == 0 {
        return RGB::ZERO;
    }

    match world.hit(
        ray,
        Interval::new(0.001, f64::INFINITY),
        &mut HitRecord::default(),
    ) {
        Some(record) => {
            let mut scattered = Ray::new(Point3::ZERO, Vec3::ZERO);
            let mut attenuation = RGB::ZERO;

            if let Some(material) = &record.material
                && material.scatter(ray, record, &mut attenuation, &mut scattered, rng)
            {
                return attenuation * ray_color(&scattered, world, max_depth - 1, rng);
            }

            // Absorbed.
            RGB::ZERO
        }
        None => {
            // Background gradient.
            let unit_direction = ray.direction().unit();
            let a = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - a) * RGB::ONE + a * BLUE_COLOR
        }
    }
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
    output.write_all(&[
        (255.999 * INTENSITY.clamp(linear_to_gamma(pixel.x()))) as u8,
        (255.999 * INTENSITY.clamp(linear_to_gamma(pixel.y()))) as u8,
        (255.999 * INTENSITY.clamp(linear_to_gamma(pixel.z()))) as u8,
    ])
}
