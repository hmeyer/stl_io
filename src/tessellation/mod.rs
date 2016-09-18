use Float;

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
