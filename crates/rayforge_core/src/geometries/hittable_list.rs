use crate::{
    geometries::{Geometries, HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Geometries>,
}

impl HittableList {
    #[must_use]
    pub fn with_object(object: Geometries) -> Self {
        Self::default().add(object)
    }

    #[must_use]
    pub fn clear(mut self) -> Self {
        self.objects.clear();

        self
    }

    #[must_use]
    pub fn add(mut self, object: Geometries) -> Self {
        self.objects.push(object);

        self
    }
}

impl Hittable for HittableList {
    fn hit<'a>(
        &self,
        ray: &Ray,
        ray_distance_interval: Interval,
        hit_record: &'a mut HitRecord,
    ) -> Option<&'a mut HitRecord> {
        // Seed with the far bound so we can tell "nothing hit" apart from "hit".
        hit_record.distance = ray_distance_interval.max;

        for object in &self.objects {
            // Reborrow trick, see below
            let distance_max = hit_record.distance;
            object.hit(
                ray,
                Interval::new(ray_distance_interval.min, distance_max),
                hit_record,
            );
        }

        (hit_record.distance < ray_distance_interval.max).then_some(hit_record)
    }
}
