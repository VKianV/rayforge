use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    shapes::sphere::Sphere,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<Arc<dyn Material>>,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[must_use]
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Option<Arc<dyn Material>>,
        distance: f64,
        front_face: bool,
    ) -> Self {
        Self {
            point,
            normal,
            material,
            distance,
            front_face,
        }
    }

    /// NOTE: `outward_normal` is assumed to have unit length.
    pub fn set_face_and_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord>;
}

pub enum Shapes {
    Sphere(Sphere),
}

impl Hittable for Shapes {
    fn hit(&self, ray: &Ray, ray_distance_interval: Interval) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, ray_distance_interval),
        }
    }
}
