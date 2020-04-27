use rand::Rng;
use std::clone::Clone;
use std::ops;

#[derive(Clone)]
pub struct Vector(f64, f64, f64);

impl Vector {
    pub fn empty() -> Vector {
        Vector(0.0, 0.0, 0.0)
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vector {
        Vector(e0, e1, e2)
    }

    pub fn random(rng: &mut impl rand::Rng) -> Vector {
        Vector(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
    }

    pub fn random_range(rng: &mut impl rand::Rng, min: f64, max: f64) -> Vector {
        Vector(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vector {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Self::random_range(&mut rng, -1.0, 1.0);
            if vec.length_squared() >= 1.0 {
                continue;
            };
            return vec;
        }
    }

    pub fn random_unit() -> Vector {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z = rng.gen_range(-1.0, 1.0);
        let r = ((1.0 - z * z) as f64).sqrt();
        Vector(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_hemisphere(normal: &Vector) -> Vector {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Vector::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vector {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vector::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            };
            return p;
        }
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
    }

    pub fn dot(u: &Vector, v: &Vector) -> f64 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    pub fn cross(u: &Vector, v: &Vector) -> Vector {
        Vector::new(
            (u.1 * v.2) - (u.2 * v.1),
            (u.2 * v.0) - (u.0 * v.2),
            (u.0 * v.1) - (u.1 * v.0),
        )
    }

    pub fn unit_vector(&self) -> Vector {
        let len = self.length();
        self / len
    }

    pub fn gamma_correct(&mut self, scale: i32) {
        let scale = 1.0 / scale as f64;
        self.0 = (scale * self.0).sqrt();
        self.1 = (scale * self.1).sqrt();
        self.2 = (scale * self.2).sqrt();
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Sub<&Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<&Vector> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<&Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Vector {
        Vector(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        rhs * self
    }
}

impl ops::Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        (1.0 / rhs) * self
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        (1.0 / rhs) * self
    }
}