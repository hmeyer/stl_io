use Float;
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use primitive::normal_from_object;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace, Rotation, Rotation2};

type Point2 = ::cgmath::Point2<Float>;
type Vector2 = ::cgmath::Vector2<Float>;


#[derive(Clone, Debug)]
pub struct Bender {
    object: Box<Object>,
    width_scaler: Float, // width (x) for one full rotation
    value_scaler: Float,
    bbox: BoundingBox,
}

impl Object for Bender {
    fn approx_value(&self, p: Point, precision: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx < precision {
            self.object.approx_value(self.bend_point(p), precision / self.value_scaler) *
            self.value_scaler
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        normal_from_object(self, p)
    }
}

impl Bender {
    // o: Object to be twisted, w: width (x) for one full rotation
    pub fn new(o: Box<Object>, w: Float) -> Box<Bender> {
        let circumference = 2. * ::std::f64::consts::PI * o.bbox().max.y;

        let scaler = circumference / w;

        let bbox = BoundingBox::new(Point::new(-o.bbox().max.y, -o.bbox().max.y, o.bbox().min.z),
                                    Point::new(o.bbox().max.y, o.bbox().max.y, o.bbox().max.z));
        Box::new(Bender {
            object: o,
            width_scaler: w / (2. * ::std::f64::consts::PI),
            value_scaler: scaler,
            bbox: bbox,
        })
    }
    fn bend_point(&self, p: Point) -> Point {
        let phi = p.y.atan2(p.x);
        let r = p.x.hypot(p.y);
        Point::new(phi, r, p.z)
    }
}
