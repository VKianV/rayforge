use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Material, // was Arc<dyn Material>
}

impl Sphere {
    #[must_use]
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit<'a>(
        &self,
        ray: &Ray,
        ray_distance_interval: Interval,
        hit_record: &'a mut HitRecord,
    ) -> Option<&'a mut HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let half_b = ray.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (half_b - sqrtd) / a;
        if !ray_distance_interval.surrounds(root) {
            root = (half_b + sqrtd) / a;
            if !ray_distance_interval.surrounds(root) {
                return None;
            }
        }

        let point = ray.at(root);

        hit_record.point = point;
        hit_record.distance = root;
        hit_record.material = Some(self.material); // Copy, no atomic
        hit_record.set_face_and_normal(ray, (point - self.center) / self.radius);

        Some(hit_record)
    }
}
