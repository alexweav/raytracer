use crate::vector::Vector;

pub trait Image {
    fn write_pixel(&mut self, color: &Vector);
}
