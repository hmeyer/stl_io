use {ALWAYS_PRECISE, Object, normal_from_object};
use bounding_box::{BoundingBox, INFINITY_BOX, NEG_INFINITY_BOX};
use xplicit_types::{Float, INFINITY, NEG_INFINITY, Point, Vector};
use cgmath::InnerSpace;

pub const FADE_RANGE: Float = 0.1;
pub const R_MULTIPLIER: Float = 1.0;

#[derive(Clone, Debug)]
pub struct Union {
    objs: Vec<Box<Object>>,
    r: Float,
    exact_range: Float, // Calculate smooth transitions over this range
    fade_range: Float, // Fade normal over this fraction of the smoothing range
    bbox: BoundingBox,
}

impl Union {
    pub fn from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let bbox = v.iter()
                            .fold(NEG_INFINITY_BOX.clone(),
                                  |union_box, x| union_box.union(x.bbox()))
                            .dilate(r * 0.2); // dilate by some factor of r
                Some(Box::new(Union {
                    objs: v,
                    r: r,
                    bbox: bbox,
                    exact_range: r * R_MULTIPLIER,
                    fade_range: FADE_RANGE,
                }))
            }
        }
    }
}

impl Object for Union {
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx <= slack {
            rvmin(&self.objs
                       .iter()
                       .map(|o| o.approx_value(p, slack + self.r))
                       .collect::<Vec<Float>>(),
                  self.r,
                  self.exact_range)
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        // Find the two smallest values with their indices.
        let (v0, v1) = self.objs
                           .iter()
                           .enumerate()
                           .fold(((0, INFINITY), (0, INFINITY)), |(v0, v1), x| {
                               let t = x.1.approx_value(p, ALWAYS_PRECISE);
                               if t < v0.1 {
                                   ((x.0, t), v0)
                               } else if t < v1.1 {
                                   (v0, (x.0, t))
                               } else {
                                   (v0, v1)
                               }
                           });

        match (v0.1 - v1.1).abs() {
            // if they are close together, calc normal from full object
            diff if diff < (self.exact_range * (1. - self.fade_range)) => {
                // else,
                normal_from_object(self, p)
            }
            diff if diff < self.exact_range => {
                let fader = (diff / self.exact_range - 1. + self.fade_range) / self.fade_range;
                (self.objs[v0.0].normal(p) * fader + normal_from_object(self, p) * (1. - fader))
                    .normalize()
            }
            // they are far apart, use the min's normal
            _ => self.objs[v0.0].normal(p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Intersection {
    objs: Vec<Box<Object>>,
    r: Float,
    exact_range: Float, // Calculate smooth transitions over this range
    fade_range: Float, // Fade normal over this fraction of the smoothing range
    bbox: BoundingBox,
}

impl Intersection {
    pub fn from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let bbox = v.iter().fold(INFINITY_BOX.clone(),
                                         |union_box, x| union_box.intersection(x.bbox()));
                Some(Box::new(Intersection {
                    objs: v,
                    r: r,
                    bbox: bbox,
                    exact_range: r * R_MULTIPLIER,
                    fade_range: FADE_RANGE,
                }))
            }
        }
    }
    pub fn difference_from_vec(mut v: Vec<Box<Object>>, r: Float) -> Option<Box<Object>> {
        match v.len() {
            0 => None,
            1 => Some(v.pop().unwrap()),
            _ => {
                let neg_rest = Negation::from_vec(v.split_off(1));
                v.extend(neg_rest);
                Intersection::from_vec(v, r)
            }
        }

    }
}

impl Object for Intersection {
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx <= slack {
            rvmax(&self.objs
                       .iter()
                       .map(|o| o.approx_value(p, slack + self.r))
                       .collect::<Vec<Float>>(),
                  self.r,
                  self.exact_range)
        } else {
            approx
        }
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        // Find the two largest values with their indices.
        let (v0, v1) = self.objs
                           .iter()
                           .enumerate()
                           .fold(((0, NEG_INFINITY), (0, NEG_INFINITY)), |(v0, v1), x| {
                               let t = x.1.approx_value(p, ALWAYS_PRECISE);
                               if t > v0.1 {
                                   ((x.0, t), v0)
                               } else if t > v1.1 {
                                   (v0, (x.0, t))
                               } else {
                                   (v0, v1)
                               }
                           });
        match (v0.1 - v1.1).abs() {
            // if they are close together, calc normal from full object
            diff if diff < (self.exact_range * (1. - self.fade_range)) => {
                // else,
                normal_from_object(self, p)
            }
            diff if diff < self.exact_range => {
                let fader = (diff / self.exact_range - 1. + self.fade_range) / self.fade_range;
                (self.objs[v0.0].normal(p) * fader + normal_from_object(self, p) * (1. - fader))
                    .normalize()
            }
            // they are far apart, use the min's normal
            _ => self.objs[v0.0].normal(p),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Negation {
    object: Box<Object>,
}

impl Negation {
    pub fn from_vec(v: Vec<Box<Object>>) -> Vec<Box<Object>> {
        v.iter()
         .map(|o| Box::new(Negation { object: o.clone() }) as Box<Object>)
         .collect()
    }
}

impl Object for Negation {
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        -self.object.approx_value(p, slack)
    }
    fn normal(&self, p: Point) -> Vector {
        self.object.normal(p) * -1.
    }
}

fn rvmin(v: &[Float], r: Float, exact_range: Float) -> Float {
    let mut close_min = false;
    let minimum = v.iter().fold(INFINITY, |min, x| {
        if x < &min {
            if (min - x) < exact_range {
                close_min = true;
            } else {
                close_min = false;
            }
            *x
        } else {
            if (x - min) < exact_range {
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

fn rvmax(v: &[Float], r: Float, exact_range: Float) -> Float {
    let mut close_max = false;
    let maximum = v.iter().fold(NEG_INFINITY, |max, x| {
        if x > &max {
            if (x - max) < exact_range {
                close_max = true;
            } else {
                close_max = false;
            }
            *x
        } else {
            if (max - x) < exact_range {
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
