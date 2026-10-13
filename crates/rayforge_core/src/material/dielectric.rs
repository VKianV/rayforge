use crate::{color::RGB, hittable::HitRecord, material::Material, ray::Ray};
use rander::Rng;

/// Dielectric (glass / water / air bubble)
pub struct Dielectric {
    /// Refractive index in vacuum/air, OR (when the material is embedded
    /// inside another dielectric) the ratio of the object's IOR over the
    /// enclosing medium's IOR.
    refraction_index: f64,
}

impl Dielectric {
    #[must_use]
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    /// Schlick's approximation for Fresnel reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut RGB,
        scattered: &mut Ray,
        rng: &mut Rng,
    ) -> bool {
        // Glass absorbs nothing.
        *attenuation = RGB::ONE;

        // When the ray is leaving the object, the ratio flips.
        let ri = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().unit();
        let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Snell's law has no solution -> total internal reflection.
        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rng.next_f64() {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, ri)
        };

        *scattered = Ray::new(record.point, direction);

        true
    }
}
