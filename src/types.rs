use std::cmp::PartialEq;
use std::ops::{Add, Mul};

use cgmath::EuclideanVector;

use Float;

type CGPoint3 = ::cgmath::Point3<Float>;
pub type CGVector3 = ::cgmath::Vector3<Float>;
type CGRotation3 = ::cgmath::Basis3<Float>;
type CGDecomposed3 = ::cgmath::Decomposed<CGVector3, CGRotation3>;

pub const EPSILON: Float = 1e-10;
pub const EPSILON_X: Vector = Vector {
    v: CGVector3 {
        x: EPSILON,
        y: 0.,
        z: 0.,
    },
};
pub const EPSILON_Y: Vector = Vector {
    v: CGVector3 {
        x: 0.,
        y: EPSILON,
        z: 0.,
    },
};
pub const EPSILON_Z: Vector = Vector {
    v: CGVector3 {
        x: 0.,
        y: 0.,
        z: EPSILON,
    },
};


#[derive(Copy, Clone, Debug)]
pub struct Point {
    p: CGPoint3,
}

impl Point {
    pub fn new(x: Float, y: Float, z: Float) -> Point {
        Point { p: CGPoint3::new(x, y, z) }
    }
    pub fn to_vec(self) -> Vector {
        Vector { v: ::cgmath::Point::to_vec(self.p) }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub v: CGVector3,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { v: self.v + other.v }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point { p: self.p + other.v }
    }
}


impl Mul<Float> for Vector {
    type Output = Vector;

    fn mul(self, other: Float) -> Vector {
        Vector { v: self.v * other }
    }
}


impl Vector {
    pub fn new(x: Float, y: Float, z: Float) -> Vector {
        Vector { v: CGVector3::new(x, y, z) }
    }
    pub fn normalize(&self) -> Vector {
        Vector { v: self.v.normalize() }
    }
    pub fn length(&self) -> Float {
        self.v.length()
    }
    pub fn dot(&self, o: Self) -> Float {
        ::cgmath::Vector::dot(self.v, o.v)
    }
    pub fn set_x(&mut self, v: Float) {
        self.v.x = v
    }
    pub fn set_y(&mut self, v: Float) {
        self.v.y = v;
    }
    pub fn set_z(&mut self, v: Float) {
        self.v.z = v
    }
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

#[derive(Copy, Clone)]
pub struct Transform {
    t: CGDecomposed3,
    i: CGDecomposed3,
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        self.t.scale == other.t.scale && self.t.rot == other.t.rot &&
        self.t.disp == other.t.disp && self.i.scale == other.i.scale &&
        self.i.rot == other.i.rot && self.i.disp == other.i.disp
    }
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            t: CGDecomposed3 {
                scale: 1 as Float,
                rot: ::cgmath::Rotation::one(),
                disp: CGVector3::new(0., 0., 0.),
            },
            i: CGDecomposed3 {
                scale: 1 as Float,
                rot: ::cgmath::Rotation::one(),
                disp: CGVector3::new(0., 0., 0.),
            },
        }
    }
    pub fn translate(t: &Vector) -> Transform {
        Transform {
            t: CGDecomposed3 {
                scale: 1 as Float,
                rot: ::cgmath::Rotation::one(),
                disp: t.v,
            },
            i: CGDecomposed3 {
                scale: 1 as Float,
                rot: ::cgmath::Rotation::one(),
                disp: t.v * -1.,
            },
        }
    }
    pub fn rotate(r: &Vector) -> Transform {
        let rotation = ::cgmath::Rotation3::from_euler(::cgmath::Angle::new(r.v.x),
                                                       ::cgmath::Angle::new(r.v.y),
                                                       ::cgmath::Angle::new(r.v.z));
        let inverted_rotation = rotation;
        // inverted_rotation.invert_self();
        Transform {
            t: CGDecomposed3 {
                scale: 1 as Float,
                rot: rotation,
                disp: CGVector3::new(0., 0., 0.),
            },
            i: CGDecomposed3 {
                scale: 1 as Float,
                rot: inverted_rotation,
                disp: CGVector3::new(0., 0., 0.),
            },
        }
    }
    pub fn scale(s: Float) -> Transform {
        Transform {
            t: CGDecomposed3 {
                scale: s,
                rot: ::cgmath::Rotation::one(),
                disp: CGVector3::new(0., 0., 0.),
            },
            i: CGDecomposed3 {
                scale: 1. / s,
                rot: ::cgmath::Rotation::one(),
                disp: CGVector3::new(0., 0., 0.),
            },
        }
    }
    pub fn t_point(&self, p: Point) -> Point {
        Point { p: ::cgmath::Transform::transform_point(&self.t, p.p) }
    }
    pub fn t_vector(&self, v: Vector) -> Vector {
        Vector { v: ::cgmath::Transform::transform_vector(&self.t, v.v) }
    }
    pub fn i_vector(&self, v: Vector) -> Vector {
        Vector { v: ::cgmath::Transform::transform_vector(&self.i, v.v) }
    }
    pub fn concat(&self, other: &Transform) -> Self {
        let c = ::cgmath::Transform::concat(&self.t, &other.t);
        let i = ::cgmath::Transform::invert(&c);
        Transform {
            t: c,
            i: i.expect("concat resulted in non-invertable transform"),
        }
    }
}

impl ::std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f,
               "Transform{{ t.scale: {:?}, t.disp: {:?}}}",
               self.t.scale,
               self.t.disp)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let t = Transform::identity();
        // {
        // scale: 1.,
        // rot: Rotation::one(),
        // disp: Vector::new(0., 0., 0.)
        // };
        //
        let v = Vector::new(0., 8., 15.);
        let p = Point::new(47., 1., 1.);

        assert_eq!(v, t.t_vector(v));
        assert_eq!(p, t.t_point(p));
    }
}
