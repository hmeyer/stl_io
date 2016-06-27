use Float;
use primitive::Object;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace};
use primitive::bounding_box::BoundingBox;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    radius: Float,
    bbox: BoundingBox,
}

impl Sphere {
    pub fn new(r: Float) -> Box<Sphere> {
        Box::new(Sphere {
            radius: r,
            bbox: BoundingBox::new(Point::new(-r, -r, -r), Point::new(r, r, r)),
        })

    }
}

impl Object for Sphere {
    fn value(&self, p: Point) -> Float {
        return p.to_vec().magnitude() - self.radius;
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        return p.to_vec().normalize();
    }
}
