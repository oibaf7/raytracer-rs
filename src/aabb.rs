use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Default, Copy, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_boxes(box0: AABB, box1: AABB) -> Self {
        Self {
            x: Interval::from_intervals(box0.x, box1.x),
            y: Interval::from_intervals(box0.y, box1.y),
            z: Interval::from_intervals(box0.z, box1.z),
        }
    }

    pub fn from_vecs(a: Vec3, b: Vec3) -> Self {
        Self {
            x: if a.x() <= b.x() {
                Interval::new(a.x(), b.x())
            } else {
                Interval::new(b.x(), a.x())
            },
            y: if a.y() <= b.y() {
                Interval::new(a.y(), b.y())
            } else {
                Interval::new(b.y(), a.y())
            },
            z: if a.z() <= b.z() {
                Interval::new(a.z(), b.z())
            } else {
                Interval::new(b.z(), a.z())
            },
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Interval> {
        let mut ray_t = ray_t;
        let ray_orig = ray.origin();
        let ray_dir = ray.dir();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let dim = match axis {
                0 => ray_orig.x(),
                1 => ray_orig.y(),
                _ => ray_orig.z(),
            };
            let dir_dim = match axis {
                0 => ray_dir.x(),
                1 => ray_dir.y(),
                _ => ray_dir.z(),
            };
            let adinv = 1.0 / dir_dim;
            let t0 = (ax.min() - dim) * adinv;
            let t1 = (ax.max() - dim) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min() {
                    ray_t.set_min(t0);
                }
                if t1 < ray_t.max() {
                    ray_t.set_max(t1);
                }
            } else {
                if t1 > ray_t.min() {
                    ray_t.set_min(t1);
                }
                if t0 < ray_t.max() {
                    ray_t.set_max(t0);
                }
            }

            if ray_t.min() >= ray_t.max() {
                return None;
            }
        }

        Some(ray_t)
    }
}
