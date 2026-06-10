use crate::vector::Vec3;

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(
            self.origin.x() + self.dir.x() * t,
            self.origin.y() + self.dir.y() * t,
            self.origin.z() + self.dir.z() * t,
        )
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }
}
