use crate::{
    hittable::{HitRecord, Hittable, sphere::Sphere},
    interval::Interval,
    ray::Ray,
};

pub enum Shapes {
    Sphere(Sphere),
}

impl Hittable for Shapes {
    fn hit<'a>(
        &self,
        ray: &Ray,
        ray_distance_interval: Interval,
        hit_record: &'a mut HitRecord,
    ) -> Option<&'a mut HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, ray_distance_interval, hit_record),
        }
    }
}
