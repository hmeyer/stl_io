use {Float, INFINITY, NEG_INFINITY};
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use types::{Point, Vector};

#[derive(Clone, Debug, PartialEq)]
pub struct SlabX {
    distance_from_zero: Float,
    bbox: BoundingBox,
}

impl SlabX {
    pub fn new(thickness: Float) -> Box<SlabX> {
        let d = thickness * 0.5;
        Box::new(SlabX {
            distance_from_zero: d,
            bbox: BoundingBox::new(Point::new(-d, NEG_INFINITY, NEG_INFINITY),
                                   Point::new(d, INFINITY, INFINITY)),
        })
    }
}

impl Object for SlabX {
    fn approx_value(&self, p: Point, _: Float) -> Float {
        return p.x.abs() - self.distance_from_zero;
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        if p.x > 0. {
            return Vector::new(1., 0., 0.);
        } else {
            return Vector::new(-1., 0., 0.);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlabY {
    distance_from_zero: Float,
    bbox: BoundingBox,
}

impl SlabY {
    pub fn new(thickness: Float) -> Box<SlabY> {
        let d = thickness * 0.5;
        Box::new(SlabY {
            distance_from_zero: d,
            bbox: BoundingBox::new(Point::new(NEG_INFINITY, -d, NEG_INFINITY),
                                   Point::new(INFINITY, d, INFINITY)),
        })
    }
}

impl Object for SlabY {
    fn approx_value(&self, p: Point, _: Float) -> Float {
        return p.y.abs() - self.distance_from_zero;
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        if p.y > 0. {
            return Vector::new(0., 1., 0.);
        } else {
            return Vector::new(0., -1., 0.);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlabZ {
    distance_from_zero: Float,
    bbox: BoundingBox,
}

impl SlabZ {
    pub fn new(thickness: Float) -> Box<SlabZ> {
        let d = thickness * 0.5;
        Box::new(SlabZ {
            distance_from_zero: d,
            bbox: BoundingBox::new(Point::new(NEG_INFINITY, NEG_INFINITY, -d),
                                   Point::new(INFINITY, INFINITY, d)),
        })
    }
}

impl Object for SlabZ {
    fn approx_value(&self, p: Point, _: Float) -> Float {
        return p.z.abs() - self.distance_from_zero;
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        if p.z > 0. {
            return Vector::new(0., 0., 1.);
        } else {
            return Vector::new(0., 0., -1.);
        }
    }
}
