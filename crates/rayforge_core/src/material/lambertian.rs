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
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut Rng,
    ) -> Option<ScatterRecord> {
        let mut dir = hit_record.normal + Vec3::random_unit_vector(rng);

        if dir.near_zero() {
            dir = hit_record.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.point, dir),
        })
    }
}
