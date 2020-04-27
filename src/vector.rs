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
}

impl_op_ex!(+ |a: &Vector, b: &Vector| -> Vector { Vector(a.0 + b.0, a.1 + b.1, a.2 + b.2) });
impl_op_ex!(-|a: &Vector, b: &Vector| -> Vector { Vector(a.0 - b.0, a.1 - b.1, a.2 - b.2) });
impl_op_ex!(-|a: &Vector| -> Vector { Vector(-a.0, -a.1, -a.2) });
impl_op_ex!(*|a: &Vector, b: &Vector| -> Vector { Vector(a.0 * b.0, a.1 * b.1, a.2 * b.2) });
impl_op_ex!(*|a: &Vector, b: f64| -> Vector { Vector(a.0 * b, a.1 * b, a.2 * b) });
impl_op_ex!(*|a: f64, b: &Vector| -> Vector { Vector(a * b.0, a * b.1, a * b.2) });
impl_op_ex!(/ |a: &Vector, b: f64| -> Vector { (1.0 / b) * a });

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn constructors_give_reasonable_values() {
        let empty = Vector::empty();
        assert_eq!(0., empty.x());
        assert_eq!(0., empty.y());
        assert_eq!(0., empty.z());
        let vector = Vector::new(1., 2., 3.);
        assert_eq!(1., vector.x());
        assert_eq!(2., vector.y());
        assert_eq!(3., vector.z());
    }

    #[test]
    fn test_vector_dot_product() {
        let vector = Vector::new(1., 2., 3.);
        let dot = Vector::dot(&vector, &vector);
        assert!(approx_eq!(f64, 14., dot));
    }

    #[test]
    fn test_vector_length() {
        let vector = Vector::new(1., 1., 1.);
        let length = vector.length();
        assert!(approx_eq!(f64, (3. as f64).sqrt(), length));
    }

    #[test]
    fn test_vector_length_squared() {
        let vector = Vector::new(1., 2., 3.);
        let length_squared = vector.length_squared();
        assert!(approx_eq!(f64, 14., length_squared));
    }

    #[test]
    fn test_unit_vector_conversion() {
        let vector = Vector::new(3., 0., 0.);
        let unit = vector.unit_vector();
        assert_eq!(1., unit.x());
        assert_eq!(0., unit.y());
        assert_eq!(0., unit.z());

        let vector = Vector::new(1., 2., 3.);
        let unit = vector.unit_vector();
        assert!(approx_eq!(f64, 1., unit.length()));
    }

    #[test]
    fn test_vector_cross_product() {
        let vec1 = Vector::new(2., 3., 4.);
        let vec2 = Vector::new(5., 6., 7.);
        let cross = Vector::cross(&vec1, &vec2);
        assert_eq!(-3., cross.x());
        assert_eq!(6., cross.y());
        assert_eq!(-3., cross.z());
    }

    #[test]
    fn test_vector_add() {
        let vec1 = Vector::new(2., 3., 4.);
        let vec2 = Vector::new(5., 6., 7.);
        let result = vec1 + vec2;
        assert_eq!(7., result.x());
        assert_eq!(9., result.y());
        assert_eq!(11., result.z());
    }

    #[test]
    fn test_vector_subtract() {
        let vec1 = Vector::new(2., 3., 4.);
        let vec2 = Vector::new(5., 6., 7.);
        let result = vec1 - vec2;
        assert_eq!(-3., result.x());
        assert_eq!(-3., result.y());
        assert_eq!(-3., result.z());
    }

    #[test]
    fn test_vector_negate() {
        let vec1 = Vector::new(2., 3., 4.);
        let result = -vec1;
        assert_eq!(-2., result.x());
        assert_eq!(-3., result.y());
        assert_eq!(-4., result.z());
    }

    #[test]
    fn test_vector_elemwise_multiply() {
        let vec1 = Vector::new(2., 3., 4.);
        let vec2 = Vector::new(5., 6., 7.);
        let result = vec1 * vec2;
        assert_eq!(10., result.x());
        assert_eq!(18., result.y());
        assert_eq!(28., result.z());
    }

    #[test]
    fn test_vector_scalar_multiply() {
        let vec1 = Vector::new(2., 3., 4.);
        let result = &vec1 * 2.;
        assert_eq!(4., result.x());
        assert_eq!(6., result.y());
        assert_eq!(8., result.z());
        let result = 3. * &vec1;
        assert_eq!(6., result.x());
        assert_eq!(9., result.y());
        assert_eq!(12., result.z());
    }

    #[test]
    fn test_vector_scalar_divide() {
        let vec1 = Vector::new(2., 4., 8.);
        let result = vec1 / 2.;
        assert_eq!(1., result.x());
        assert_eq!(2., result.y());
        assert_eq!(4., result.z());
    }
}
