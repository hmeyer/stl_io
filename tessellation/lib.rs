extern crate cgmath;
extern crate rand;
extern crate nalgebra as na;
extern crate nalgebra_lapack;
extern crate xplicit_types;
extern crate xplicit_primitive;
extern crate time;

pub use xplicit_types::Float;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<[Float; 3]>,
    pub faces: Vec<[usize; 3]>,
}

mod bitset;
pub use self::bitset::BitSet;
mod dual_marching_cubes;
pub use self::dual_marching_cubes::DualMarchingCubes;
mod dual_marching_cubes_cell_configs;
