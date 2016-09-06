extern crate kiss3d;
extern crate nalgebra as na;

use std::rc::Rc;
use std::cell::RefCell;
use primitive::Object;
use self::kiss3d::resource::Mesh;


pub struct DualMarchingCubes {
    object: Box<Object>,
}

impl DualMarchingCubes {
    pub fn new(obj: Box<Object>) -> DualMarchingCubes {
        DualMarchingCubes { object: obj }
    }
    pub fn tesselate(&self) -> Rc<RefCell<Mesh>> {
        Rc::new(RefCell::new(Mesh::new(vec![na::Point3::new(0., 0., 0.),
                                            na::Point3::new(9., 0., 0.),
                                            na::Point3::new(0., 9., 0.),
                                            na::Point3::new(0., 0., 9.)],
                                       vec![
                na::Point3::new(0, 2, 1),
                na::Point3::new(0, 1, 3), na::Point3::new(1, 2, 3), na::Point3::new(2, 0, 3),
            ],
                                       None,
                                       None,
                                       true)))
    }
}
