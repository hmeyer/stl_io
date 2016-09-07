use Float;

pub struct Mesh {
    pub vertices: Vec<[Float; 3]>,
    pub faces: Vec<[usize; 3]>,
}

mod dual_marching_cubes;
pub use self::dual_marching_cubes::DualMarchingCubes;
