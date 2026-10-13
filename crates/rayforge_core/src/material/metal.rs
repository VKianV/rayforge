use crate::{color::RGB, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};
use rander::Rng;

/// Metal (mirror, optionally fuzzy)
pub struct Metal {
    pub albedo: RGB,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub const fn new(albedo: RGB, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut RGB,
        scattered: &mut Ray,
        rng: &mut Rng,
    ) -> bool {
        let reflected = ray_in.direction().unit().reflect(record.normal);
        let reflected = reflected + self.fuzz * Vec3::random_unit_vector(rng);

        *scattered = Ray::new(record.point, reflected);
        *attenuation = self.albedo;

        // Absorb rays that would scatter into the surface.
        scattered.direction().dot(record.normal) > 0.0
    }
}
