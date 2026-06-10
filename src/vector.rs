use crate::interval::Interval;
use crate::utility::random_double;
use rand::Rng;
use std::{fmt, ops};

#[derive(Default, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Self {
        Self {
            x: self.y * rhs.x - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    fn random_vector() -> Self {
        Vec3 {
            x: rand::thread_rng().r#gen::<f64>(),
            y: rand::thread_rng().r#gen::<f64>(),
            z: rand::thread_rng().r#gen::<f64>(),
        }
    }

    fn random_bounded_vector(min: f64, max: f64) -> Self {
        Vec3 {
            x: random_double(min, max),
            y: random_double(min, max),
            z: random_double(min, max),
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_bounded_vector(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p * (1.0 / lensq.sqrt());
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if (on_unit_sphere.dot(normal) > 0.0) {
            on_unit_sphere
        } else {
            on_unit_sphere * -1.0
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s)
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * v.dot(n) * 2.0
    }

    pub fn refract(uv: Vec3, n: Vec3, index: f64) -> Vec3 {
        let cos = f64::min(n.dot(uv * -1.0), 1.0);
        let r_out_perp = (uv + n * cos) * index;
        let r_out_parallel = n * f64::abs(1.0 - r_out_perp.length_squared()).sqrt() * -1.0;

        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[derive(Clone, Copy)]
pub struct Color(Vec3);

impl Color {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }
    pub fn from_vec(vec: Vec3) -> Self {
        Self(vec)
    }

    pub fn vec(&self) -> Vec3 {
        self.0
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }

        0.0
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = Self::linear_to_gamma(self.0.x);
        let g = Self::linear_to_gamma(self.0.y);
        let b = Self::linear_to_gamma(self.0.z);

        let interval = Interval::new(0.0, 0.999);
        write!(
            f,
            "{} {} {}\n",
            (interval.clamp(r) * 256.0) as u8,
            (interval.clamp(g) * 256.0) as u8,
            (interval.clamp(b) * 256.0) as u8
        )
    }
}
