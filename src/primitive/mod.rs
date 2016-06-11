use std::fmt::Debug;
use Float;
use types::{Point, Vector, EPSILON_X, EPSILON_Y, EPSILON_Z};

use cgmath::InnerSpace;

mod transformer;
pub use self::transformer::AffineTransformer;

mod boolean;
pub use self::boolean::{Union, Intersection};

mod sphere;
pub use self::sphere::Sphere;

mod cylinder;
pub use self::cylinder::Cylinder;

pub fn normal_from_object(f: &Object, p: Point) -> Vector {
    let center = f.value(p);
    let dx = f.value(p + EPSILON_X) - center;
    let dy = f.value(p + EPSILON_Y) - center;
    let dz = f.value(p + EPSILON_Z) - center;
    Vector::new(dx, dy, dz).normalize()
}

pub trait Object: ObjectClone + Debug {
    fn value(&self, p: Point) -> Float;
    fn normal(&self, p: Point) -> Vector {
        let center = self.value(p);
        let dx = self.value(p + EPSILON_X) - center;
        let dy = self.value(p + EPSILON_Y) - center;
        let dz = self.value(p + EPSILON_Z) - center;
        Vector::new(dx, dy, dz).normalize()
    }
    fn translate(&self, v: Vector) -> Box<Object> {
        AffineTransformer::new_translate(self.clone_box(), v)
    }
    fn rotate(&self, r: Vector) -> Box<Object> {
        AffineTransformer::new_rotate(self.clone_box(), r)
    }
    fn scale(&self, s: Vector) -> Box<Object> {
        AffineTransformer::new_scale(self.clone_box(), s)
    }
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<Object>;
}

impl<T> ObjectClone for T
    where T: 'static + Object + Clone
{
    fn clone_box(&self) -> Box<Object> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<Object> {
    fn clone(&self) -> Box<Object> {
        self.clone_box()
    }
}

// Objects never equal each other
impl PartialEq for Box<Object> {
    fn eq(&self, _: &Box<Object>) -> bool {
        false
    }
}

// Objects are never ordered
impl PartialOrd for Box<Object> {
    fn partial_cmp(&self, _: &Box<Object>) -> Option<::std::cmp::Ordering> {
        None
    }
}
