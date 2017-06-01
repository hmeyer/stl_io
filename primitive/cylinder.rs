use Object;
use bounding_box::{BoundingBox, INFINITY_BOX};
use truescad_types::{Float, INFINITY, NEG_INFINITY, Point, Vector};


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
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx <= slack {
            let pv = Vector::new(p.x, p.y, 0.);
            return pv.norm() - self.radius;
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        let pv = Vector::new(p.x, p.y, 0.);
        return pv.normalize();
    }
}

// A cone along the Z-Axis
#[derive(Clone, Debug, PartialEq)]
pub struct Cone {
    slope: Float,
    distance_multiplier: Float,
    offset: Float, // Offset the singularity from Z-zero
    normal_multiplier: Float, // muliplier for the normal caclulation
    bbox: BoundingBox,
}

impl Cone {
    pub fn new(slope: Float, offset: Float) -> Box<Cone> {
        Box::new(Cone {
            slope: slope,
            distance_multiplier: 1. / (slope * slope + 1.).sqrt(), // cos(atan(slope))
            offset: offset,
            normal_multiplier: slope / (slope * slope + 1.).sqrt(), // sin(atan(slope))
            bbox: INFINITY_BOX.clone(),
        })
    }
}

impl Object for Cone {
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn set_bbox(&mut self, bbox: BoundingBox) {
        self.bbox = bbox
    }
    fn approx_value(&self, p: Point, _: Float) -> Float {
        let radius = self.slope * (p.z + self.offset).abs();
        let pv = Vector::new(p.x, p.y, 0.);
        return (pv.norm() - radius) * self.distance_multiplier;
    }
    fn normal(&self, p: Point) -> Vector {
        let s = (p.z + self.offset).signum();
        let mut pv = Vector::new(p.x, p.y, 0.);
        pv.normalize_mut();
        pv *= self.distance_multiplier;
        pv.z = -s * self.normal_multiplier;
        return pv;
    }
}
