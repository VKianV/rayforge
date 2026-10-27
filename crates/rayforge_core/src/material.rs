use crate::{
    color::RGB,
    geometries::HitRecord,
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    ray::Ray,
};
use rander::Rng;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut Rng)
    -> Option<ScatterRecord>;
}

/// What a material returns when a ray scatters off it.
pub struct ScatterRecord {
    pub attenuation: RGB,
    pub scattered: Ray,
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    /// `Some(rec)` if the ray scatters, `None` if absorbed.
    #[inline]
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut Rng,
    ) -> Option<ScatterRecord> {
        match self {
            Self::Lambertian(m) => m.scatter(ray_in, hit_record, rng),
            Self::Metal(m) => m.scatter(ray_in, hit_record, rng),
            Self::Dielectric(m) => m.scatter(ray_in, hit_record, rng),
        }
    }
}

impl From<Lambertian> for Material {
    fn from(m: Lambertian) -> Self {
        Self::Lambertian(m)
    }
}

impl From<Metal> for Material {
    fn from(m: Metal) -> Self {
        Self::Metal(m)
    }
}

impl From<Dielectric> for Material {
    fn from(m: Dielectric) -> Self {
        Self::Dielectric(m)
    }
}
