use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    shapes::hittable::{HitRecord, Hittable},
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = h.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_distance_interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_distance_interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);

        let mut hit_record = HitRecord::new(
            point,
            Vec3::default(),
            Some(Arc::clone(&self.material)),
            root,
            false,
        );
        hit_record.set_face_and_normal(ray, (point - self.center) / self.radius);

        Some(hit_record)
    }
}
