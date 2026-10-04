use crate::{color::RGB, ray::Ray, shapes::hittable::HitRecord, vec3::Vec3};
use rander::Rng;

pub struct ScatterRecord {
    pub attenuation: RGB,
    pub scattered: Ray,
}

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

/// Metal (mirror, optionally fuzzy)
pub struct Metal {
    pub albedo: RGB,
    fuzz: f64,
}

impl Metal {
    #[must_use]
    pub fn new(albedo: RGB, fuzz: f64) -> Self {
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
