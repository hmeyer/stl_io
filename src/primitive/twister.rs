use Float;
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use types::{Point, Vector};
use cgmath::{EuclideanSpace, InnerSpace, Rotation, Rotation2};

type Point2 = ::cgmath::Point2<Float>;
type Vector2 = ::cgmath::Vector2<Float>;


#[derive(Clone, Debug)]
pub struct Twister {
    object: Box<Object>,
    height_scaler: Float, // 2 * pi / (height for full rotation)
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
        self.untwist_vector(self.object.normal(self.twist_point(p)), p)
    }
}

impl Twister {
    // o: Object to be twisted, h: height for one full rotation
    pub fn new(o: Box<Object>, h: Float) -> Box<Twister> {
        let mx = o.bbox().min.x.abs().max(o.bbox().max.x.abs());
        let my = o.bbox().min.y.abs().max(o.bbox().max.y.abs());
        let r = mx.hypot(my);

        // The ratio of height and circumference (slope on the outer edge).
        let tan_a = h / (2. * ::std::f64::consts::PI * r);
        // The scaler is 1 / sin(a)
        // sin(atan(x)) =   x / sqrt(x^2 + 1)
        let scaler = tan_a / (tan_a * tan_a + 1.).sqrt();

        let bbox = BoundingBox::new(Point::new(-r, -r, o.bbox().min.z),
                                    Point::new(r, r, o.bbox().max.z));
        Box::new(Twister {
            object: o,
            height_scaler: ::std::f64::consts::PI * 2. / h,
            value_scaler: scaler,
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
    // Apply tilt to the vector.
    // Since Surfaces are twisted, all normals will be tilted, depending on the radius.
    fn tilt_vector(&self, v: Vector, p: Point) -> Vector {
        let planar_v = ::cgmath::Vector2::new(v.x, v.y);
        let radius_v = ::cgmath::Vector2::new(p.x, p.y);
        let radius = radius_v.magnitude();
        let radius_v = radius_v / radius;
        let tangent_v = ::cgmath::Vector2::new(radius_v.y, -radius_v.x);

        let tangential_projection = tangent_v.dot(planar_v);

        let tangential_shear = radius * self.height_scaler;

        let mut result = v.clone();
        result.z -= tangential_shear * tangential_projection;

        return result.normalize();
    }
    fn untwist_vector(&self, v: Vector, p: Point) -> Vector {
        let v2 = ::cgmath::Vector2::new(v.x, v.y);
        let angle = ::cgmath::Rad { s: -p.z * self.height_scaler };
        let trans = ::cgmath::Basis2::from_angle(angle);
        let rv2 = trans.rotate_vector(v2);
        self.tilt_vector(Vector::new(rv2.x, rv2.y, v.z), p)
    }
}
