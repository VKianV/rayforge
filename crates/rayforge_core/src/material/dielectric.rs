use crate::{
    color::RGB,
    geometries::HitRecord,
    material::{ScatterRecord, Scatterable},
    ray::Ray,
};
use rander::Rng;

#[derive(Clone, Copy)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    #[must_use]
    pub const fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut Rng) -> Option<ScatterRecord> {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = ray_in.direction().unit();
        let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rng.next_f32() {
            unit_dir.reflect(rec.normal)
        } else {
            unit_dir.refract(rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation: RGB::ONE,
            scattered: Ray::new(rec.point, direction),
        })
    }
}
