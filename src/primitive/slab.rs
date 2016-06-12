use Float;
use primitive::Object;
use types::{Point, Vector};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SlabX {
    distance_from_zero: Float,
}

impl SlabX {
    pub fn new(thickness: Float) -> Box<SlabX> {
        Box::new(SlabX { distance_from_zero: thickness * 0.5 })
    }
}

impl Object for SlabX {
    fn value(&self, p: Point) -> Float {
        return p.x.abs() - self.distance_from_zero;
    }
    fn normal(&self, p: Point) -> Vector {
        if p.x.abs() > self.distance_from_zero && p.x > 0. {
            return Vector::new(1., 0., 0.);
        } else {
            return Vector::new(-1., 0., 0.);
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SlabY {
    distance_from_zero: Float,
}

impl SlabY {
    pub fn new(thickness: Float) -> Box<SlabY> {
        Box::new(SlabY { distance_from_zero: thickness * 0.5 })
    }
}

impl Object for SlabY {
    fn value(&self, p: Point) -> Float {
        return p.y.abs() - self.distance_from_zero;
    }
    fn normal(&self, p: Point) -> Vector {
        if p.y.abs() > self.distance_from_zero && p.y > 0. {
            return Vector::new(0., 1., 0.);
        } else {
            return Vector::new(0., -1., 0.);
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SlabZ {
    distance_from_zero: Float,
}

impl SlabZ {
    pub fn new(thickness: Float) -> Box<SlabZ> {
        Box::new(SlabZ { distance_from_zero: thickness * 0.5 })
    }
}

impl Object for SlabZ {
    fn value(&self, p: Point) -> Float {
        return p.z.abs() - self.distance_from_zero;
    }
    fn normal(&self, p: Point) -> Vector {
        if p.z.abs() > self.distance_from_zero && p.z > 0. {
            return Vector::new(0., 0., 1.);
        } else {
            return Vector::new(0., 0., -1.);
        }
    }
}
