use crate::{
    interval::Interval,
    ray::Ray,
    shapes::{hittable::Hittable, hittable_list::HittableList},
    vec3::{Point3, RGB, Vec3},
};
use rander::Rng;
use std::io::{self, Write};

/// Recursive ray marcher with:
/// * depth limiting (avoids stack overflow)
/// * shadow-acne tolerance (0.001 t-min instead of 0.0)
/// * Lambertian scattering (`normal + random_unit_vector`)
/// * 0.5 reflectance (i.e. 50% gray diffuse)
#[must_use]
pub fn ray_color(ray: &Ray, world: &HittableList, max_depth: usize, rng: &mut Rng) -> RGB {
    // No more light is gathered once the bounce limit is exhausted.
    if max_depth == 0 {
        return RGB::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
        let mut scattered = Ray::new(Point3::ZERO, Vec3::ZERO);
        let mut attenuation = RGB::new(0.0, 0.0, 0.0);

        if let Some(material) = &rec.material 
            && material.scatter(ray, &rec, &mut attenuation, &mut scattered, rng) {
                return attenuation * ray_color(&scattered, world, max_depth - 1, rng);
        }

        // Absorbed.
        return RGB::new(0.0, 0.0, 0.0);
    }

    // Background gradient.
    let unit_direction = ray.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * RGB::new(1.0, 1.0, 1.0) + a * RGB::new(0.5, 0.7, 1.0)
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
