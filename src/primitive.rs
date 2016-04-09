use std::fmt::Debug;

use Float;

use types::{Point, Vector, Transform, EPSILON_X, EPSILON_Y, EPSILON_Z};

pub trait Object: Debug {
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
}

#[derive(Debug)]
pub struct Sphere {
    radius: Float,
    trans: Transform,
}

impl Sphere {
    pub fn new(r: Float) -> Sphere {
        Sphere {
            radius: r,
            trans: Transform::identity(),
        }
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

#[derive(Debug)]
pub struct Neg {
    a: Box<Object>,
}

impl Neg {
    pub fn new(a: Box<Object>) -> Neg {
        Neg { a: a }
    }
}

impl Object for Neg {
    fn value(&self, p: &Point) -> Float {
        return -self.a.value(p);
    }
    fn normal(&self, p: &Point) -> Vector {
        self.a.normal(p) * -1.0
    }
    fn concat_transform(&mut self, other: &Transform) {
        self.a.concat_transform(other);
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

#[derive(Debug)]
pub struct Union {
    a: Box<Object>,
    b: Box<Object>,
}

impl Union {
    pub fn new(a: Box<Object>, b: Box<Object>) -> Union {
        Union { a: a, b: b }
    }
    pub fn from_vec(mut v: Vec<Box<Object>>) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let l2 = v.len() / 2;
                let v2 = v.split_off(l2);
                Some(Box::new(Union::new(Union::from_vec(v).unwrap(),
                                         Union::from_vec(v2).unwrap())))
            }
        }
    }
}

impl Object for Union {
    fn value(&self, p: &Point) -> Float {
        return self.a.value(p).min(self.b.value(p));
    }

    fn normal(&self, p: &Point) -> Vector {
        let va = self.a.value(p);
        let vb = self.b.value(p);
        if va < vb {
            self.a.normal(p)
        } else {
            self.b.normal(p)
        }
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

#[derive(Debug)]
pub struct Intersection {
    a: Box<Object>,
    b: Box<Object>,
}

impl Intersection {
    pub fn new(a: Box<Object>, b: Box<Object>) -> Intersection {
        Intersection { a: a, b: b }
    }
}

impl Object for Intersection {
    fn value(&self, p: &Point) -> Float {
        return self.a.value(p).max(self.b.value(p));
    }

    fn normal(&self, p: &Point) -> Vector {
        let va = self.a.value(p);
        let vb = self.b.value(p);
        if va > vb {
            self.a.normal(p)
        } else {
            self.b.normal(p)
        }
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


#[derive(Debug)]
pub struct Subtraction {
    i: Intersection,
}

impl Subtraction {
    pub fn new(a: Box<Object>, b: Box<Object>) -> Subtraction {
        Subtraction { i: Intersection::new(a, Box::new(Neg::new(b))) }
    }
}

impl Object for Subtraction {
    fn value(&self, p: &Point) -> Float {
        self.i.value(p)
    }

    fn normal(&self, p: &Point) -> Vector {
        self.i.normal(p)
    }
    fn concat_transform(&mut self, other: &Transform) {
        self.i.concat_transform(other)
    }
    fn basic_value(&self, p: &Point) -> Float {
        self.i.basic_value(p)
    }
    fn basic_normal(&self, p: &Point) -> Vector {
        self.i.basic_normal(p)
    }
    fn transform(&self) -> &Transform {
        self.i.transform()
    }
}
