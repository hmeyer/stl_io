extern crate cgmath;

pub type Float = f64;
pub type Point = ::cgmath::Point3<Float>;
pub type Vector = ::cgmath::Vector3<Float>;
pub type Matrix = ::cgmath::Matrix4<Float>;


pub const INFINITY: Float = 1e10;
pub const NEG_INFINITY: Float = -1e10;
pub const NAN: Float = ::std::f64::NAN;
pub const PI: Float = ::std::f64::consts::PI;

pub const EPSILON: Float = 1e-10;
pub const MIN_POSITIVE: Float = ::std::f64::MIN_POSITIVE;
pub const EPSILON_X: Vector = Vector {
    x: EPSILON,
    y: 0.,
    z: 0.,
};
pub const EPSILON_Y: Vector = Vector {
    x: 0.,
    y: EPSILON,
    z: 0.,
};
pub const EPSILON_Z: Vector = Vector {
    x: 0.,
    y: 0.,
    z: EPSILON,
};


#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn new(o: Point, d: Vector) -> Ray {
        Ray {
            origin: o,
            dir: d,
        }
    }
}
