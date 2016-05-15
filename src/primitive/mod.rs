use std::fmt::Debug;

use Float;

use types::{Point, Vector, Transform, EPSILON_X, EPSILON_Y, EPSILON_Z};

mod boolean;
pub use self::boolean::{Union, Intersection};

mod sphere;
pub use self::sphere::Sphere;

mod cylinder;
pub use self::cylinder::InfiniteCylinder;


pub trait ImplicitFunction {
    fn value(&self, p: &Point) -> Float;
    fn normal(&self, p: &Point) -> Vector;
}

pub fn normal_from_implicit<T: ImplicitFunction>(f: &T, p: &Point) -> Vector {
    let center = f.value(p);
    let dx = f.value(&(*p + EPSILON_X)) - center;
    let dy = f.value(&(*p + EPSILON_Y)) - center;
    let dz = f.value(&(*p + EPSILON_Z)) - center;
    Vector::new(dx, dy, dz).normalize()
}

pub trait Primitive: ImplicitFunction + Clone + Debug {}

pub trait Object: ImplicitFunction + ObjectClone + Debug {
    fn apply_transform(&mut self, other: &Transform);
    fn translate(&mut self, t: Vector) {
        let trans = Transform::translate(&t);
        self.apply_transform(&trans);
    }
    fn rotate(&mut self, r: Vector) {
        let trans = Transform::rotate(&r);
        self.apply_transform(&trans);
    }
    fn scale(&mut self, s: Float) {
        let trans = Transform::scale(s);
        self.apply_transform(&trans);
    }
    fn to_string(&self) -> String {
        format!("{:?}", self)
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

// TODO: This is a hack. Replace it with something sane.
impl PartialEq for Box<Object> {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

// TODO: This is a hack. Replace it with something sane.
impl PartialOrd for Box<Object> {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        let s = self.to_string();
        let o = other.to_string();
        if s < o {
            return Some(::std::cmp::Ordering::Less);
        } else if s > o {
            return Some(::std::cmp::Ordering::Greater);
        } else {
            return Some(::std::cmp::Ordering::Equal);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveWrapper<T: Primitive> {
    primitive: Box<T>,
    transform: Transform,
}

impl<T: Primitive + 'static> ImplicitFunction for PrimitiveWrapper<T> {
    fn value(&self, p: &Point) -> Float {
        self.primitive.value(&self.transform.t_point(*p))
    }
    fn normal(&self, p: &Point) -> Vector {
        self.transform
            .i_vector(self.primitive.normal(&self.transform.t_point(*p)))
            .normalize()
    }
}
impl<T: Primitive + 'static> Object for PrimitiveWrapper<T> {
    fn apply_transform(&mut self, other: &Transform) {
        self.transform = self.transform.concat(other)
    }
}
