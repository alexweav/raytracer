use crate::camera::Camera;
use crate::scenery::Scenery;

pub struct Environment {
    pub scenery: Scenery,
    pub camera: Camera,
    pub image: Image,
}

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub supersampling_ratio: i32,
}
