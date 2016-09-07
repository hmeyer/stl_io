use primitive::Object;
use tessellation::Mesh;


pub struct DualMarchingCubes {
    object: Box<Object>,
}

impl DualMarchingCubes {
    pub fn new(obj: Box<Object>) -> DualMarchingCubes {
        DualMarchingCubes { object: obj }
    }
    pub fn tesselate(&self) -> Mesh {
        Mesh {
            vertices: vec![[0., 0., 0.], [9., 0., 0.], [0., 9., 0.], [0., 0., 9.]],
            faces: vec![[0, 2, 1], [0, 1, 3], [1, 2, 3], [2, 0, 3]],
        }
    }
}
