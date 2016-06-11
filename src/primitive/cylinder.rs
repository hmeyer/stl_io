use Float;
use primitive::Object;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Cylinder {
    radius: Float,
}

impl Cylinder {
    pub fn new(r: Float) -> Box<Cylinder> {
        Box::new(Cylinder { radius: r })
    }
}

impl Object for Cylinder {
    fn value(&self, p: Point) -> Float {
        let mut pv = p.to_vec();
        pv.z = 0.;
        return pv.magnitude() - self.radius;
    }
    fn normal(&self, p: Point) -> Vector {
        let mut pv = p.to_vec();
        pv.z = 0.;
        return pv.normalize();
    }
}
