use crate::{
    color::RGB,
    geometries::HitRecord,
    material::{ScatterRecord, Scatterable},
    ray::Ray,
    vec3::Vec3,
};
use rander::Rng;

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: RGB,
    fuzz: f32,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: RGB, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, rng: &mut Rng) -> Option<ScatterRecord> {
        let reflected = ray_in.direction().unit().reflect(rec.normal)
            + self.fuzz * Vec3::random_unit_vector(rng);

        if reflected.dot(rec.normal) <= 0.0 {
            return None; // absorbed
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.point, reflected),
        })
    }
}
