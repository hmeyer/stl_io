use {Float, INFINITY, NEG_INFINITY};
use types::{Point, Matrix};
use cgmath::Transform;


pub struct BoundingBox {
    min: Point,
    max: Point,
}

fn PointMin(p: &[Point]) -> Point {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = PointMin(p1);
        let b = PointMin(p2);
        Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))

    }
}
fn PointMax(p: &[Point]) -> Point {
    if p.len() == 1 {
        p[0]
    } else {
        let (p1, p2) = p.split_at(p.len() / 2);
        let a = PointMax(p1);
        let b = PointMax(p2);
        Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z))
    }
}

impl BoundingBox {
    fn infinity() -> BoundingBox {
        BoundingBox {
            min: Point::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
            max: Point::new(INFINITY, INFINITY, INFINITY),
        }
    }
    fn new(min: Point, max: Point) -> BoundingBox {
        BoundingBox {
            min: min,
            max: max,
        }
    }
    fn union(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: PointMin(&[self.min, other.min]),
            max: PointMax(&[self.max, other.max]),
        }
    }
    fn intersection(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: PointMax(&[self.min, other.min]),
            max: PointMin(&[self.min, other.min]),
        }
    }
    fn transform(&self, mat: &Matrix) -> BoundingBox {
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
            min: PointMin(&corners),
            max: PointMax(&corners),
        }
    }
    fn value(&self, p: Point) -> Float {
        // If p is not inside (neg), then it is outside (pos) on only one side.
        // So so calculating the max of the diffs on both sides should result in the true value,
        // if positive.
        let xval = (p.x - self.max.x).max(self.min.x - p.x);
        let yval = (p.y - self.max.y).max(self.min.y - p.y);
        let zval = (p.z - self.max.z).max(self.min.z - p.z);
        xval.max(yval.max(zval))
    }
}
