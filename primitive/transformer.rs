use Object;
use bounding_box::BoundingBox;
use xplicit_types::{Float, Matrix, Point, Vector};
use cgmath::{InnerSpace, SquareMatrix, Transform};
use std::{error, fmt};

#[derive(Clone, Debug)]
pub struct AffineTransformer {
    object: Box<Object>,
    transform: Matrix,
    transposed: Matrix,
    scale_min: Float,
    bbox: BoundingBox,
}

#[derive(Debug)]
enum TransformerError {
    FailedInversion(Matrix),
}

impl error::Error for TransformerError {
    fn description(&self) -> &str {
        match self {
            &TransformerError::FailedInversion(_) => "Failed to invert Matrix.",
        }
    }
}

impl fmt::Display for TransformerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &TransformerError::FailedInversion(m) => write!(f, "Failed to invert {:?}", m),
        }
    }
}

impl Object for AffineTransformer {
    fn approx_value(&self, p: Point, slack: Float) -> Float {
        let approx = self.bbox.value(p);
        if approx <= slack {
            self.object.approx_value(self.transform.transform_point(p), slack / self.scale_min) *
            self.scale_min
        } else {
            approx
        }
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
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.scale_min)
    }
    fn rotate(&self, r: Vector) -> Box<Object> {
        let euler = ::cgmath::Euler::new(::cgmath::Rad(r.x),
                                         ::cgmath::Rad(r.y),
                                         ::cgmath::Rad(r.z));
        let new_trans = self.transform.concat(&Matrix::from(euler));
        AffineTransformer::new_with_scaler(self.object.clone(), new_trans, self.scale_min)
    }
    fn scale(&self, s: Vector) -> Box<Object> {
        let new_trans = self.transform
                            .concat(&Matrix::from_nonuniform_scale(1. / s.x, 1. / s.y, 1. / s.z));
        AffineTransformer::new_with_scaler(self.object.clone(),
                                           new_trans,
                                           self.scale_min * s.x.min(s.y.min(s.z)))
    }
}

impl AffineTransformer {
    fn identity(o: Box<Object>) -> Box<Object> {
        AffineTransformer::new(o, Matrix::identity())
    }
    fn new(o: Box<Object>, t: Matrix) -> Box<AffineTransformer> {
        AffineTransformer::new_with_scaler(o, t, 1.)
    }
    fn new_with_scaler(o: Box<Object>, t: Matrix, scale_min: Float) -> Box<AffineTransformer> {
        // TODO: Calculate scale_min from t.
        // This should be something similar to
        // 1./Vector::new(t.x.x, t.y.x, t.z.x).magnitude().min(
        // 1./Vector::new(t.x.y, t.y.y, t.z.y).magnitude().min(
        // 1./Vector::new(t.x.z, t.y.z, t.z.z).magnitude()))

        let mut transposed = t.clone();
        transposed.transpose_self();
        match t.invert() {
            None => panic!("Failed to invert {:?}", t),
            Some(t_inv) => {
                let bbox = o.bbox().transform(&t_inv);
                Box::new(AffineTransformer {
                    object: o,
                    transform: t,
                    transposed: transposed,
                    scale_min: scale_min,
                    bbox: bbox,
                })
            }
        }
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
