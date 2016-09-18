use {Float, INFINITY, NEG_INFINITY};
use types::{Point, Matrix, Vector};
use cgmath::Transform;

pub static INFINITY_BOX: BoundingBox = BoundingBox {
    min: Point {
        x: NEG_INFINITY,
        y: NEG_INFINITY,
        z: NEG_INFINITY,
    },
    max: Point {
        x: INFINITY,
        y: INFINITY,
        z: INFINITY,
    },
};

pub static NEG_INFINITY_BOX: BoundingBox = BoundingBox {
    min: Point {
        x: INFINITY,
        y: INFINITY,
        z: INFINITY,
    },
    max: Point {
        x: NEG_INFINITY,
        y: NEG_INFINITY,
        z: NEG_INFINITY,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

fn point_min(p: &[Point]) -> Point {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_min(p1);
        let b = point_min(p2);
        Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }
}
fn point_max(p: &[Point]) -> Point {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = point_max(p1);
        let b = point_max(p2);
        Point::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
    }
}

impl BoundingBox {
    pub fn infinity() -> BoundingBox {
        INFINITY_BOX.clone()
    }
    pub fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox {
            min: min,
            max: max,
        }
    }
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: point_min(&[self.min, other.min]),
            max: point_max(&[self.max, other.max]),
        }
    }
    pub fn intersection(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: point_max(&[self.min, other.min]),
            max: point_min(&[self.max, other.max]),
        }
    }
    pub fn transform(&self, mat: &Matrix) -> BoundingBox {
        let a = &self.min;
        let b = &self.max;
        let corners = [mat.transform_point(Point::new(a.x, a.y, a.z)),
                       mat.transform_point(Point::new(a.x, a.y, b.z)),
                       mat.transform_point(Point::new(a.x, b.y, a.z)),
                       mat.transform_point(Point::new(a.x, b.y, b.z)),
                       mat.transform_point(Point::new(b.x, a.y, a.z)),
                       mat.transform_point(Point::new(b.x, a.y, b.z)),
                       mat.transform_point(Point::new(b.x, b.y, a.z)),
                       mat.transform_point(Point::new(b.x, b.y, b.z))];
        BoundingBox {
            min: point_min(&corners),
            max: point_max(&corners),
        }
    }
    pub fn dilate(&self, d: Float) -> BoundingBox {
        BoundingBox {
            min: Point::new(self.min.x - d, self.min.y - d, self.min.z - d),
            max: Point::new(self.max.x + d, self.max.y + d, self.max.z + d),
        }
    }
    pub fn dim(&self) -> Vector {
        self.max - self.min
    }
    pub fn value(&self, p: Point) -> Float {
        // If p is not inside (neg), then it is outside (pos) on only one side.
        // So so calculating the max of the diffs on both sides should result in the true value,
        // if positive.
        let xval = (p.x - self.max.x).max(self.min.x - p.x);
        let yval = (p.y - self.max.y).max(self.min.y - p.y);
        let zval = (p.z - self.max.z).max(self.min.z - p.z);
        xval.max(yval.max(zval))
    }
}
