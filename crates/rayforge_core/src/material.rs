use crate::{color::RGB, hittable::HitRecord, ray::Ray};
use rander::Rng;

pub mod dielectric;
pub mod lambertian;
pub mod metal;

/// A material decides how an incoming ray scatters off a surface.
///
/// `Send + Sync` so `Arc<dyn Material>` can be shared across render threads.
pub trait Material: Send + Sync {
    /// Returns `true` if the ray was scattered. On `false`, the ray is
    /// considered absorbed and the caller should treat the hit as black.
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut RGB,
        scattered: &mut Ray,
        rng: &mut Rng,
    ) -> bool;
}

pub struct ScatterRecord {
    pub attenuation: RGB,
    pub scattered: Ray,
}
