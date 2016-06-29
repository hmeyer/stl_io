use {Float, INFINITY, NEG_INFINITY /* ); */};
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace};


// A cylinder along the Z-Axis
#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    radius: Float,
    bbox: BoundingBox,
}

impl Cylinder {
    pub fn new(r: Float) -> Box<Cylinder> {
        Box::new(Cylinder {
            radius: r,
            bbox: BoundingBox::new(Point::new(-r, -r, NEG_INFINITY), Point::new(r, r, INFINITY)),
        })
    }
}

impl Object for Cylinder {
    fn approx_value(&self, p: Point, precision: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx < precision {
            let mut pv = p.to_vec();
            pv.z = 0.;
            return pv.magnitude() - self.radius;
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        let mut pv = p.to_vec();
        pv.z = 0.;
        return pv.normalize();
    }
}

// A cone along the Z-Axis
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Cone {
    slope: Float,
    distance_multiplier: Float,
    offset: Float, // Offset the singularity from Z-zero
    normal_multiplier: Float, // muliplier for the normal caclulation
}

impl Cone {
    pub fn new(slope: Float, offset: Float) -> Box<Cone> {
        Box::new(Cone {
            slope: slope,
            distance_multiplier: 1. / (slope * slope + 1.).sqrt(), // cos(atan(slope))
            offset: offset,
            normal_multiplier: slope / (slope * slope + 1.).sqrt(), // sin(atan(slope))
        })
    }
}

impl Object for Cone {
    fn approx_value(&self, p: Point, _: Float) -> Float {
        let mut pv = p.to_vec();
        let radius = self.slope * (pv.z + self.offset).abs();
        pv.z = 0.;
        return (pv.magnitude() - radius) * self.distance_multiplier;
    }
    fn normal(&self, p: Point) -> Vector {
        let mut pv = p.to_vec();
        let s = (pv.z + self.offset).signum();
        pv.z = 0.;
        pv = pv.normalize_to(self.distance_multiplier);
        pv.z = -s * self.normal_multiplier;
        return pv;
    }
}
