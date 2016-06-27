use Float;
use primitive::Object;
use primitive::bounding_box::BoundingBox;
use types::{Point, Vector, Matrix};
use cgmath::{InnerSpace, SquareMatrix, Transform};

#[derive(Clone, Debug)]
pub struct AffineTransformer {
    object: Box<Object>,
    transform: Matrix,
    transposed: Matrix,
    value_scaler: Float,
    bbox: BoundingBox,
}

impl Object for AffineTransformer {
    fn value(&self, p: Point) -> Float {
        self.object.value(self.transform.transform_point(p)) * self.value_scaler
    }
    fn bbox(&self) -> &BoundingBox {
        &self.bbox
    }
    fn normal(&self, p: Point) -> Vector {
        self.transposed
            .transform_vector(self.object
                                  .normal(self.transform
                                              .transform_point(p)))
            .normalize()
    }
    fn translate(&self, v: Vector) -> Box<Object> {
        let other = Matrix::from_translation(-v);
        let new_trans = self.transform.concat(&other);
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.value_scaler)
    }
    fn rotate(&self, r: Vector) -> Box<Object> {
        let euler = ::cgmath::Euler::new(::cgmath::Rad { s: r.x },
                                         ::cgmath::Rad { s: r.y },
                                         ::cgmath::Rad { s: r.z });
        let new_trans = self.transform.concat(&Matrix::from(euler));
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.value_scaler)
    }
    fn scale(&self, s: Vector) -> Box<Object> {
        let new_trans = self.transform
                            .concat(&Matrix::from_nonuniform_scale(1. / s.x, 1. / s.y, 1. / s.z));
        AffineTransformer::new_with_scaler(self.object.clone(),
                                           new_trans,
                                           self.value_scaler * s.x.min(s.y.min(s.z)))
    }
}

impl AffineTransformer {
    fn identity(o: Box<Object>) -> Box<Object> {
        AffineTransformer::new(o, Matrix::identity())
    }
    fn new(o: Box<Object>, t: Matrix) -> Box<AffineTransformer> {
        AffineTransformer::new_with_scaler(o, t, 1.)
    }
    fn new_with_scaler(o: Box<Object>, t: Matrix, scaler: Float) -> Box<AffineTransformer> {
        let mut transposed = t.clone();
        transposed.transpose_self();
        let bbox = o.bbox().transform(&t.invert().unwrap());
        Box::new(AffineTransformer {
            object: o,
            transform: t,
            transposed: transposed,
            value_scaler: scaler,
            bbox: bbox,
        })
    }
    pub fn new_translate(o: Box<Object>, v: Vector) -> Box<Object> {
        AffineTransformer::identity(o).translate(v)
    }
    pub fn new_rotate(o: Box<Object>, r: Vector) -> Box<Object> {
        AffineTransformer::identity(o).rotate(r)
    }
    pub fn new_scale(o: Box<Object>, s: Vector) -> Box<Object> {
        AffineTransformer::identity(o).scale(s)
    }
}
