#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;

pub type Float = f64;
pub type Point = na::Point3<Float>;
pub type Vector = na::Vector3<Float>;
pub type Transform = na::Matrix4<Float>;


pub const INFINITY: Float = 1e10;
pub const NEG_INFINITY: Float = -1e10;
pub const NAN: Float = ::std::f64::NAN;
pub const PI: Float = ::std::f64::consts::PI;

pub const EPSILON: Float = 1e-10;
pub const MIN_POSITIVE: Float = ::std::f64::MIN_POSITIVE;

lazy_static! {
    pub static ref EPSILON_X: Vector = Vector::new(EPSILON, 0., 0.);
    pub static ref EPSILON_Y: Vector = Vector::new(0., EPSILON, 0.);
    pub static ref EPSILON_Z: Vector = Vector::new(0., 0., EPSILON);
}

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
