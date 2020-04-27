use crate::vec3::Vec3;

pub fn to_color(vec: Vec3, scale_factor: i32) -> Vec3 {
    let scale = 1. / scale_factor as f64;
    let r = (scale * vec.x()).sqrt();
    let g = (scale * vec.y()).sqrt();
    let b = (scale * vec.z()).sqrt();

    Vec3::new(
        256. * clamp(r, 0., 0.999),
        256. * clamp(g, 0., 0.999),
        256. * clamp(b, 0., 0.999),
    )
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
