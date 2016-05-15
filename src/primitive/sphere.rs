use Float;
use primitive::{ImplicitFunction, Primitive, PrimitiveWrapper};
use types::{Point, Vector, Transform};

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
