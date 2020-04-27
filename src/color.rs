use crate::vector::Vector;

pub fn to_color(vec: Vector, scale_factor: i32) -> Vector {
    let scale = 1. / scale_factor as f64;
    let r = (scale * vec.x()).sqrt();
    let g = (scale * vec.y()).sqrt();
    let b = (scale * vec.z()).sqrt();

    Vector::new(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn clamp_returns_expected() {
        assert_eq!(2., clamp(1., 2., 3.));
        assert_eq!(3., clamp(4., 2., 3.));
        assert_eq!(2.5, clamp(2.5, 2.0, 3.0));
    }

    #[test]
    fn color_conversion_handles_supersampling() {
        let color = to_color(Vector::new(5., 5., 5.), 5);
        assert_eq!(255, color.x() as i32);
        assert_eq!(255, color.y() as i32);
        assert_eq!(255, color.z() as i32);
    }

    #[test]
    fn color_conversion_applies_gamma_correction() {
        let color = to_color(Vector::new(0.5, 0.5, 0.5), 1);
        assert_eq!(181, color.x() as i32);
        assert_eq!(181, color.y() as i32);
        assert_eq!(181, color.z() as i32);
    }

    #[test]
    fn color_conversion_clamps_unreasonable_values() {
        let color = to_color(Vector::new(3.0, 2.0, 0.5), 1);
        assert_eq!(255, color.x() as i32);
        assert_eq!(255, color.y() as i32);
        assert_eq!(181, color.z() as i32);
    }
}
