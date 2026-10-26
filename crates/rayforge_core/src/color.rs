use crate::{
    geometries::{HitRecord, Hittable, hittable_list::HittableList},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
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
    if max_depth == 0 {
        return RGB::ZERO;
    }

    let hit_record = &mut HitRecord::default();

    let Some(record) = world.hit(ray, Interval::new(0.001, f32::INFINITY), hit_record) else {
        // Background gradient.
        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);

        return (1.0 - a) * RGB::ONE + a * BLUE_COLOR;
    };

    if let Some(mat) = record.material
        && let Some(sc) = mat.scatter(ray, record, rng)
    {
        return sc.attenuation * ray_color(&sc.scattered, world, max_depth - 1, rng);
    }

    RGB::ZERO
}

#[inline]
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

#[inline]
pub fn write_color<W: Write>(output: &mut W, pixel_color: &RGB) -> io::Result<()> {
    let intensity = Interval::new(0.000, 0.999);

    // Apply a linear-to-gamma transform for gamma 2.
    output.write_all(&[
        (255.999 * intensity.clamp(linear_to_gamma(pixel_color.x()))) as u8,
        (255.999 * intensity.clamp(linear_to_gamma(pixel_color.y()))) as u8,
        (255.999 * intensity.clamp(linear_to_gamma(pixel_color.z()))) as u8,
    ])
}
