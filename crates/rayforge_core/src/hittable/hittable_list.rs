use crate::{
    hittable::{HitRecord, Hittable, shapes::Shapes},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Shapes>,
}

impl HittableList {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_object(object: Shapes) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Shapes) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit<'a>(
        &self,
        ray: &Ray,
        ray_distance_interval: Interval,
        hit_record: &'a mut HitRecord,
    ) -> Option<&'a mut HitRecord> {
        let mut closest_so_far = ray_distance_interval.max;
        let mut hit_anything = false;

        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(
                ray,
                Interval::new(ray_distance_interval.min, closest_so_far),
                hit_record,
            ) {
                closest_so_far = temp_hit_record.distance;
                hit_anything = true;
            }
        }

        if hit_anything { Some(hit_record) } else { None }
    }
}
