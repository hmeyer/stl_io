use Float;
use primitive::{ImplicitFunction, Primitive, PrimitiveWrapper};
use types::{Point, Vector, Transform};

#[derive(Clone, Debug, PartialEq)]
pub struct CylinderPrimitive {
    radius: Float,
}

impl CylinderPrimitive {
    pub fn new(r: Float) -> Box<CylinderPrimitive> {
        Box::new(CylinderPrimitive { radius: r })
    }
}

impl ImplicitFunction for CylinderPrimitive {
    fn value(&self, p: &Point) -> Float {
        let mut pv = p.to_vec();
        pv.v.z = 0.;
        return pv.length() - self.radius;
    }
    fn normal(&self, p: &Point) -> Vector {
        let mut pv = p.to_vec();
        pv.v.z = 0.;
        return pv.normalize();
    }
}

impl Primitive for CylinderPrimitive {}

pub type InfiniteCylinder = PrimitiveWrapper<CylinderPrimitive>;

impl InfiniteCylinder {
    pub fn new(r: Float) -> Box<InfiniteCylinder> {
        Box::new(InfiniteCylinder {
            primitive: CylinderPrimitive::new(r),
            transform: Transform::identity(),
        })
    }
}
