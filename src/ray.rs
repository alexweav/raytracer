use crate::vector::Vector;

#[derive(Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: &Vector, direction: &Vector) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin(&self) -> &Vector {
        &self.origin
    }

    pub fn direction(&self) -> &Vector {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vector {
        self.origin() + (t * self.direction())
    }
}
