#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate nalgebra as na;
extern crate rayon;
extern crate truescad_types;
extern crate truescad_primitive;
extern crate time;
extern crate byteorder;

use truescad_types::{Float, Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub p: Point,
    pub n: Vector,
}

mod bitset;
mod vertex_index;
mod manifold_dual_contouring;
mod cell_configs;
mod qef;

pub use self::manifold_dual_contouring::ManifoldDualContouring;

// This is just exposed for the bench test - do not use!
pub use self::manifold_dual_contouring::ManifoldDualContouringImpl;
pub use self::manifold_dual_contouring::subsample_octtree;


#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<[Float; 3]>,
    pub faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn normal32(&self, face: usize) -> [f32; 3] {
        let v: Vec<na::Point3<f32>> = self.faces[face].iter()
            .map(|&i| na::Point3::<f32>::new(self.vertices[i][0] as f32, self.vertices[i][1] as f32, self.vertices[i][2] as f32))
            .collect();
            let r = (v[1] - v[0]).cross(&(v[2] - v[0])).normalize();
            [r[0], r[1], r[2]]
    }
    pub fn vertex32(&self, i: usize) -> [f32; 3] {
        let v = self.vertices[i];
        [v[0] as f32, v[1] as f32, v[2] as f32]
    }
}

#[cfg(test)]
#[macro_use]
extern crate approx;
