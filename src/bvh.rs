use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use rand::Rng;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.aabb.hit(ray, ray_t)?;

        let hit_left = self.left.hit(ray, ray_t);
        let max = if let Some(r) = hit_left.as_ref() {
            r.t
        } else {
            ray_t.max()
        };
        let hit_right = self.right.hit(ray, Interval::new(ray_t.min(), max));

        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

impl BVHNode {
    //look into making cleaner
    pub fn new(objs: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Self {
        let axis = rand::thread_rng().gen_range(0..3);

        let span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match span {
            1 => (Arc::clone(&objs[start]), Arc::clone(&objs[start])),
            2 => (Arc::clone(&objs[start]), Arc::clone(&objs[start + 1])),
            _ => {
                let mid = start + span / 2;
                let slice = &mut objs[start..end];
                slice.sort_by(|a, b| {
                    a.bounding_box()
                        .axis_interval(axis)
                        .min()
                        .total_cmp(&b.bounding_box().axis_interval(axis).min())
                });

                (
                    Arc::new(BVHNode::new(objs, start, mid)),
                    Arc::new(BVHNode::new(objs, mid, end)),
                )
            }
        };

        let aabb = AABB::from_boxes(left.bounding_box(), right.bounding_box());
        Self { left, right, aabb }
    }
}
