use crate::{
    geometries::sphere::Sphere,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub mod hittable_list;
pub mod sphere;

pub trait Hittable: Send + Sync {
    fn hit<'a>(
        &self,
        ray: &Ray,
        ray_distance_interval: Interval,
        hit_record: &'a mut HitRecord,
    ) -> Option<&'a mut HitRecord>;
}

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Option<Material>,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
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

pub enum Geometries {
    Sphere(Sphere),
}

impl Hittable for Geometries {
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
