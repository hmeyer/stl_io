#![feature(const_fn)]
extern crate cgmath;
extern crate rand;
extern crate nalgebra as na;
extern crate rayon;
extern crate truescad_types;
extern crate truescad_primitive;
extern crate time;
extern crate byteorder;

use truescad_types::{Float, Point, Vector};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<[Float; 3]>,
    pub faces: Vec<[usize; 3]>,
}

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    pub p: Point,
    pub n: Vector,
}

mod bitset;
mod vertex_index;
mod manifold_dual_contouring;
pub use self::manifold_dual_contouring::ManifoldDualContouring;
mod cell_configs;
mod stl_writer;
pub use self::stl_writer::write_stl;
mod qef;
