use crate::{color::RGB, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};
use rander::Rng;

/// Lambertian (diffuse)
pub struct Lambertian {
    pub albedo: RGB,
}

impl Lambertian {
    #[must_use]
    pub const fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut RGB,
        scattered: &mut Ray,
        rng: &mut Rng,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(rng);

        // Catch degenerate scatter direction (anti-parallel to the normal).
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}
