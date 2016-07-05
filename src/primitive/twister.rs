use Float;
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use types::{Point, Vector};
use cgmath::{Rotation, Rotation2};

type Point2 = ::cgmath::Point2<Float>;
type Vector2 = ::cgmath::Vector2<Float>;


#[derive(Clone, Debug)]
pub struct Twister {
    object: Box<Object>,
    height_scaler: Float,
    value_scaler: Float,
    bbox: BoundingBox,
}

impl Object for Twister {
    fn approx_value(&self, p: Point, precision: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx < precision {
            self.object.approx_value(self.twist_point(p), precision / self.value_scaler) *
            self.value_scaler
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        self.untwist_vector(self.object.normal(self.twist_point(p)), p.z)
    }
}

impl Twister {
    pub fn new(o: Box<Object>, h: Float) -> Box<Twister> {
        let mx = o.bbox().min.x.abs().max(o.bbox().max.x.abs());
        let my = o.bbox().min.y.abs().max(o.bbox().max.y.abs());
        let r = (mx * mx + my * my).sqrt();
        let bbox = BoundingBox::new(Point::new(-r, -r, o.bbox().min.z),
                                    Point::new(r, r, o.bbox().max.z));
        let v = 1.;
        Box::new(Twister {
            object: o,
            height_scaler: ::std::f64::consts::PI * 2. / h,
            value_scaler: v,
            bbox: bbox,
        })
    }
    fn twist_point(&self, p: Point) -> Point {
        let p2 = ::cgmath::Point2::new(p.x, p.y);
        let angle = ::cgmath::Rad { s: p.z * self.height_scaler };
        let trans = ::cgmath::Basis2::from_angle(angle);
        let rp2 = trans.rotate_point(p2);
        Point::new(rp2.x, rp2.y, p.z)
    }
    fn untwist_vector(&self, v: Vector, h: Float) -> Vector {
        let v2 = ::cgmath::Vector2::new(v.x, v.y);
        let angle = ::cgmath::Rad { s: -h * self.height_scaler };
        let trans = ::cgmath::Basis2::from_angle(angle);
        let rv2 = trans.rotate_vector(v2);
        Vector::new(rv2.x, rv2.y, v.z)
    }
}
