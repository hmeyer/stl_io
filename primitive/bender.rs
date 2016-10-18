use Object;
use bounding_box::BoundingBox;
use xplicit_types::{Float, PI, Point, Vector};
use cgmath::{InnerSpace, Rotation, Rotation2};

type Point2 = ::cgmath::Point2<Float>;
type Vector2 = ::cgmath::Vector2<Float>;


#[derive(Clone, Debug)]
pub struct Bender {
    object: Box<Object>,
    width_scaler: Float, // width_for_full_rotation / (2. * PI),
    bbox: BoundingBox,
}

impl Object for Bender {
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx <= slack {
            let mut obj_p = self.to_polar(p);
            let r = obj_p.y;

            // If the bended object is a ring, and p is in the center, return the distance to inner
            // margin (bbox.min.y) of the (bent) bounding box.
            let center_to_bbox = self.object.bbox().min.y - r;
            if center_to_bbox > slack {
                return center_to_bbox;
            }

            // let circumference = 2. * PI * r;
            // let width_for_full_rotation = self.width_scaler * 2. * PI;
            // let x_scale = circumference / width_for_full_rotation;
            let x_scale = r / self.width_scaler;
            let x_scaler = x_scale.min(1.);

            obj_p.x *= self.width_scaler;
            self.object.approx_value(obj_p, slack / x_scaler) * x_scaler
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        let polar_p = self.to_polar(p);
        let mut obj_p = polar_p;
        obj_p.x *= self.width_scaler;
        self.bend_normal(self.object.normal(obj_p), polar_p)
    }
}

impl Bender {
    // o: Object to be twisted, w: width (x) for one full rotation
    pub fn new(o: Box<Object>, w: Float) -> Box<Bender> {
        let bbox = BoundingBox::new(Point::new(-o.bbox().max.y, -o.bbox().max.y, o.bbox().min.z),
                                    Point::new(o.bbox().max.y, o.bbox().max.y, o.bbox().max.z));

        Box::new(Bender {
            object: o,
            width_scaler: w / (2. * PI),
            bbox: bbox,
        })
    }
    fn to_polar(&self, p: Point) -> Point {
        let phi = p.x.atan2(-p.y);
        let r = p.x.hypot(p.y);
        Point::new(phi, r, p.z)
    }
    fn tilt_normal(&self, mut normal: Vector, polar_p: Point) -> Vector {
        let r = polar_p.y;
        let circumference = 2. * PI * r;
        let width_for_one_full_rotation = self.width_scaler * 2. * PI;
        let scale_along_x = circumference / width_for_one_full_rotation;
        normal.x /= scale_along_x;
        normal.normalize()
    }
    fn bend_normal(&self, v: Vector, polar_p: Point) -> Vector {
        let v = self.tilt_normal(v, polar_p);
        let phi = ::cgmath::Rad(polar_p.x + PI);
        let v2 = ::cgmath::Vector2::new(v.x, v.y);
        let trans = ::cgmath::Basis2::from_angle(phi);
        let rv2 = trans.rotate_vector(v2);
        Vector::new(rv2.x, rv2.y, v.z)
    }
}
