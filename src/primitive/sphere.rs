use Float;
use primitive::Object;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sphere {
    radius: Float,
}

impl Sphere {
    pub fn new(r: Float) -> Box<Sphere> {
        Box::new(Sphere { radius: r })
    }
}

impl Object for Sphere {
    fn value(&self, p: Point) -> Float {
        return p.to_vec().magnitude() - self.radius;
    }
    fn normal(&self, p: Point) -> Vector {
        return p.to_vec().normalize();
    }
}
