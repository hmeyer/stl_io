use Float;
use primitive::Object;
use types::{Point, Vector, Matrix};
use cgmath::{InnerSpace, SquareMatrix, Transform};

#[derive(Clone, Debug)]
pub struct AffineTransformer {
    object: Box<Object>,
    transform: Matrix,
    transposed: Matrix,
    value_scaler: Float,
}

impl Object for AffineTransformer {
    fn value(&self, p: Point) -> Float {
        self.object.value(self.transform.transform_point(p)) * self.value_scaler
    }
    fn normal(&self, p: Point) -> Vector {
        self.transposed
            .transform_vector(self.object
                                  .normal(self.transform
                                              .transform_point(p)))
            .normalize()
    }
}

impl AffineTransformer {
    fn new(o: Box<Object>, t: Matrix) -> Box<AffineTransformer> {
        AffineTransformer::new_with_scaler(o, t, 1.)
    }
    fn new_with_scaler(o: Box<Object>, t: Matrix, scaler: Float) -> Box<AffineTransformer> {
        let mut transposed = t.clone();
        transposed.transpose_self();
        Box::new(AffineTransformer {
            object: o,
            transform: t,
            transposed: transposed,
            value_scaler: scaler,
        })
    }
    pub fn new_translate(o: Box<Object>, v: Vector) -> Box<AffineTransformer> {
        AffineTransformer::new(o, Matrix::from_translation(v))
    }
    pub fn new_rotate(o: Box<Object>, r: Vector) -> Box<AffineTransformer> {
        let euler = ::cgmath::Euler::new(::cgmath::Rad { s: r.x },
                                         ::cgmath::Rad { s: r.y },
                                         ::cgmath::Rad { s: r.z });
        AffineTransformer::new(o, Matrix::from(euler))
    }
    pub fn new_scale(o: Box<Object>, s: Vector) -> Box<AffineTransformer> {
        AffineTransformer::new_with_scaler(o,
                                           Matrix::from_nonuniform_scale(1. / s.x,
                                                                         1. / s.y,
                                                                         1. / s.z),
                                           s.x.min(s.y.min(s.z)))
    }
}
