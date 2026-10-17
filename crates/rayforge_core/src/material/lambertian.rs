use crate::{
    color::RGB,
    geometries::HitRecord,
    material::{ScatterRecord, Scatterable},
    ray::Ray,
    vec3::Vec3,
};
use rander::Rng;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: RGB,
}

impl Lambertian {
    #[must_use]
    pub const fn new(albedo: RGB) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord, rng: &mut Rng) -> Option<ScatterRecord> {
        let mut dir = rec.normal + Vec3::random_unit_vector(rng);

        if dir.near_zero() {
            dir = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.point, dir),
        })
    }
}
