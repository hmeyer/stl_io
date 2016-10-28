#![feature(const_fn)]
#![feature(step_by)]
extern crate cgmath;
extern crate rand;
extern crate nalgebra as na;
extern crate xplicit_types;
extern crate xplicit_primitive;
extern crate time;
extern crate byteorder;

use xplicit_types::{Float, Point, Vector};

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
mod dual_marching_cubes;
pub use self::dual_marching_cubes::DualMarchingCubes;
mod cell_configs;
mod stl_writer;
pub use self::stl_writer::write_stl;
mod qef;
