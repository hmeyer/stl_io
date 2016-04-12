use std::fmt::Debug;

use Float;

use types::{Point, Vector, Transform, EPSILON_X, EPSILON_Y, EPSILON_Z};

pub trait Object: ObjectClone + Debug {
    fn basic_value(&self, p: &Point) -> Float;
    fn basic_normal(&self, p: &Point) -> Vector {
        let center = self.value(p);
        let dx = self.value(&(*p + EPSILON_X)) - center;
        let dy = self.value(&(*p + EPSILON_Y)) - center;
        let dz = self.value(&(*p + EPSILON_Z)) - center;
        return Vector::new(dx, dy, dz).normalize();
    }
    fn value(&self, p: &Point) -> Float {
        self.basic_value(&self.transform().t_point(*p))
    }
    fn normal(&self, p: &Point) -> Vector {
        self.transform().i_vector(self.basic_normal(&self.transform().t_point(*p))).normalize()
    }
    fn concat_transform(&mut self, other: &Transform);
    fn transform(&self) -> &Transform;
    fn translate(&mut self, t: Vector) {
        let trans = Transform::translate(&t);
        self.concat_transform(&trans);
    }
    fn rotate(&mut self, r: Vector) {
        let trans = Transform::rotate(&r);
        self.concat_transform(&trans);
    }
    fn scale(&mut self, s: Float) {
        let trans = Transform::scale(s);
        self.concat_transform(&trans);
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
pub struct Sphere {
    radius: Float,
    trans: Transform,
}

impl Sphere {
    pub fn new(r: Float) -> Box<Sphere> {
        Box::new(Sphere {
            radius: r,
            trans: Transform::identity(),
        })
    }
}

impl Object for Sphere {
    fn basic_value(&self, p: &Point) -> Float {
        return p.to_vec().length() - self.radius;
    }

    fn basic_normal(&self, p: &Point) -> Vector {
        return p.to_vec().normalize();
    }
    fn concat_transform(&mut self, other: &Transform) {
        self.trans = self.trans.concat(other)
    }
    fn transform(&self) -> &Transform {
        &self.trans
    }
}

pub trait Mixer: Clone + Debug {
    fn new() -> Box<Self>;
    fn mixval(&self, a: Float, b: Float) -> Float;
    fn mixnormal(&self,
                     a: Float,
                     b: Float,
                     get_an: &Fn() -> Vector,
                     get_bn: &Fn() -> Vector)
                     -> Vector;
}

#[derive(Clone, Debug)]
pub struct Bool<T: Mixer> {
    a: Box<Object>,
    b: Box<Object>,
    mixer: Box<T>,
}

impl<T: Mixer + 'static> Bool<T> {
    pub fn new(a: Box<Object>, b: Box<Object>) -> Box<Bool<T>> {
        Box::new(Bool::<T> { a: a, b: b, mixer: T::new() })
    }
    pub fn from_vec(mut v: Vec<Box<Object>>) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let l2 = v.len() / 2;
                let v2 = v.split_off(l2);
                Some(Bool::<T>::new(Bool::<T>::from_vec(v).unwrap(), Bool::<T>::from_vec(v2).unwrap()))
            }
        }
    }
}


impl<T: Mixer + 'static> Object for Bool<T> {
    fn value(&self, p: &Point) -> Float {
        return self.mixer.mixval(self.a.value(p), self.b.value(p));
    }

    fn normal(&self, p: &Point) -> Vector {
        let va = self.a.value(p);
        let vb = self.b.value(p);
        self.mixer.mixnormal(va,
                             vb,
                             &|| self.a.normal(&p.clone()),
                             &|| self.b.normal(&p.clone()))
    }
    fn concat_transform(&mut self, other: &Transform) {
        self.a.concat_transform(other);
        self.b.concat_transform(other);
    }
    fn basic_value(&self, _: &Point) -> Float {
        panic!("undefined");
    }
    fn basic_normal(&self, _: &Point) -> Vector {
        panic!("undefined");
    }
    fn transform(&self) -> &Transform {
        panic!("undefined");
    }
}

#[derive(Clone, Debug)]
pub struct UnionMixer {}
impl Mixer for UnionMixer {
    fn new() -> Box<Self> { Box::new(UnionMixer{})}
    fn mixval(&self, a: Float, b: Float) -> Float {
        a.min(b)
    }
    fn mixnormal(&self,
                     a: Float,
                     b: Float,
                     get_an: &Fn() -> Vector,
                     get_bn: &Fn() -> Vector)
                     -> Vector {
                         if a < b {
                             get_an()
                         } else {
                             get_bn()
                         }
                     }
}

pub type Union = Bool<UnionMixer>;

#[derive(Clone, Debug)]
pub struct IntersectionMixer {}
impl Mixer for IntersectionMixer {
    fn new() -> Box<Self> { Box::new(IntersectionMixer{})}
    fn mixval(&self, a: Float, b: Float) -> Float {
        a.max(b)
    }
    fn mixnormal(&self,
                     a: Float,
                     b: Float,
                     get_an: &Fn() -> Vector,
                     get_bn: &Fn() -> Vector)
                     -> Vector {
                         if a > b {
                             get_an()
                         } else {
                             get_bn()
                         }
                     }
}

pub type Intersection = Bool<IntersectionMixer>;

#[derive(Clone, Debug)]
pub struct SubtractionMixer {}
impl Mixer for SubtractionMixer {
    fn new() -> Box<Self> { Box::new(SubtractionMixer{})}
    fn mixval(&self, a: Float, b: Float) -> Float {
        a.max(-b)
    }
    fn mixnormal(&self,
                     a: Float,
                     b: Float,
                     get_an: &Fn() -> Vector,
                     get_bn: &Fn() -> Vector)
                     -> Vector {
                         if a > -b {
                             get_an()
                         } else {
                             get_bn() * -1.
                         }
                     }
}

pub type Subtraction = Bool<SubtractionMixer>;
