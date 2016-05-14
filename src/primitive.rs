use std::fmt::Debug;
use std::f64;

use Float;

use types::{Point, Vector, Transform, EPSILON_X, EPSILON_Y, EPSILON_Z};

pub trait ImplicitFunction {
    fn value(&self, p: &Point) -> Float;
    fn normal(&self, p: &Point) -> Vector;
}

fn normal_from_implicit<T: ImplicitFunction>(f: &T, p: &Point) -> Vector {
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

#[derive(Clone, Debug, PartialEq)]
pub struct SpherePrimitive {
    radius: Float,
}

impl SpherePrimitive {
    pub fn new(r: Float) -> Box<SpherePrimitive> {
        Box::new(SpherePrimitive { radius: r })
    }
}

impl ImplicitFunction for SpherePrimitive {
    fn value(&self, p: &Point) -> Float {
        return p.to_vec().length() - self.radius;
    }
    fn normal(&self, p: &Point) -> Vector {
        return p.to_vec().normalize();
    }
}

impl Primitive for SpherePrimitive {}

pub type Sphere = PrimitiveWrapper<SpherePrimitive>;

impl Sphere {
    pub fn new(r: Float) -> Box<Sphere> {
        Box::new(Sphere {
            primitive: SpherePrimitive::new(r),
            transform: Transform::identity(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct Union {
    objs: Vec<Box<Object>>,
    r: Float,
}

impl Union {
    pub fn from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => Some(Box::new(Union { objs: v, r: r })),
        }
    }
}

impl ImplicitFunction for Union {
    fn value(&self, p: &Point) -> Float {
        rvmin(&self.objs.iter().map(|o| o.value(p)).collect::<Vec<f64>>(),
              self.r)
    }

    fn normal(&self, p: &Point) -> Vector {
        // Find the two smallest values with their indices.
        let (v0, v1) = self.objs
                           .iter()
                           .enumerate()
                           .fold(((0, f64::INFINITY), (0, f64::INFINITY)), |(v0, v1), x| {
                               let t = x.1.value(p);
                               if t < v0.1 {
                                   ((x.0, t), v0)
                               } else if t < v1.1 {
                                   (v0, (x.0, t))
                               } else {
                                   (v0, v1)
                               }
                           });
        // if they are far apart, use the min's normal
        if (v1.1 - v0.1) >= self.r {
            self.objs[v0.0].normal(p)
        } else {
            // else, calc normal from full implicit
            normal_from_implicit(self, p)
        }
    }
}

impl Object for Union {
    fn apply_transform(&mut self, other: &Transform) {
        for x in &mut self.objs {
            x.apply_transform(other);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Intersection {
    objs: Vec<Box<Object>>,
    r: Float,
}

impl Intersection {
    pub fn from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => Some(Box::new(Intersection { objs: v, r: r })),
        }
    }
}

impl ImplicitFunction for Intersection {
    fn value(&self, p: &Point) -> Float {
        rvmax(&self.objs.iter().map(|o| o.value(p)).collect::<Vec<f64>>(),
              self.r)
    }

    fn normal(&self, p: &Point) -> Vector {
        // Find the two largest values with their indices.
        let (v0, v1) = self.objs
                           .iter()
                           .enumerate()
                           .fold(((0, f64::NEG_INFINITY), (0, f64::NEG_INFINITY)),
                                 |(v0, v1), x| {
                                     let t = x.1.value(p);
                                     if t > v0.1 {
                                         ((x.0, t), v0)
                                     } else if t > v1.1 {
                                         (v0, (x.0, t))
                                     } else {
                                         (v0, v1)
                                     }
                                 });
        // if they are far apart, use the min's normal
        if (v0.1 - v1.1) >= self.r {
            self.objs[v0.0].normal(p)
        } else {
            // else, calc normal from full implicit
            normal_from_implicit(self, p)
        }
    }
}

impl Object for Intersection {
    fn apply_transform(&mut self, other: &Transform) {
        for x in &mut self.objs {
            x.apply_transform(other);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Difference {
    a: Box<Object>,
    b: Box<Object>,
    r: Float,
}

impl Difference {
    pub fn from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let rest = v.split_off(1);
                return Some(Box::new(Difference {
                    a: v.get_mut(0).unwrap().clone(),
                    b: Union::from_vec(rest, r).unwrap(),
                    r: r,
                }));
            }
        }
    }
}

impl ImplicitFunction for Difference {
    fn value(&self, p: &Point) -> Float {
        rmax(self.a.value(p), -self.b.value(p), self.r)
    }

    fn normal(&self, p: &Point) -> Vector {
        let va = self.a.value(p);
        let vb = -self.b.value(p);
        // if they are far apart, use the max's normal
        if (va - vb).abs() >= self.r {
            if va > vb {
                self.a.normal(p)
            } else {
                self.b.normal(p) * -1.
            }
        } else {
            // else, calc normal from full implicit
            normal_from_implicit(self, p)
        }
    }
}

impl Object for Difference {
    fn apply_transform(&mut self, other: &Transform) {
        self.a.apply_transform(other);
        self.b.apply_transform(other);
    }
}

fn rvmin(v: &[Float], r: Float) -> Float {
    let mut close_min = false;
    let minimum = v.iter().fold(f64::INFINITY, |min, x| {
        if x < &min {
            if (min - x) < r {
                close_min = true;
            } else {
                close_min = false;
            }
            *x
        } else {
            if (x - min) < r {
                close_min = true;
            }
            min
        }
    });
    if !close_min {
        return minimum;
    }
    let min_plus_r = minimum + r;
    let r4 = r / 4.;
    // Inpired by http://iquilezles.org/www/articles/smin/smin.htm
    let exp_sum = v.iter().filter(|&x| x < &min_plus_r).fold(0., |sum, x| sum + (-x / r4).exp());
    return exp_sum.ln() * -r4;
}

fn rmax(a: Float, b: Float, r: Float) -> Float {
    if (a - b).abs() < r {
        let r = r / 4.;
        // Inpired by http://iquilezles.org/www/articles/smin/smin.htm
        return ((a / r).exp() + (b / r).exp()).ln() * r;
    }
    a.max(b)
}

fn rvmax(v: &[Float], r: Float) -> Float {
    let mut close_max = false;
    let maximum = v.iter().fold(f64::NEG_INFINITY, |max, x| {
        if x > &max {
            if (x - max) < r {
                close_max = true;
            } else {
                close_max = false;
            }
            *x
        } else {
            if (max - x) < r {
                close_max = true;
            }
            max
        }
    });
    if !close_max {
        return maximum;
    }
    let max_minus_r = maximum - r;
    let r4 = r / 4.;
    let exp_sum = v.iter().filter(|&x| x > &max_minus_r).fold(0., |sum, x| sum + (x / r4).exp());
    return exp_sum.ln() * r4;
}
