extern crate kiss3d;
extern crate nalgebra as na;
use std::rc::Rc;
use std::cell::RefCell;
use self::kiss3d::window::Window;
use self::kiss3d::light::Light;
use self::kiss3d::resource::Mesh;

pub fn show_mesh(mesh: Rc<RefCell<Mesh>>) {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_mesh(mesh, na::Vector3::new(1.0, 1.0, 1.0));

    c.set_color(1.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    while window.render() {}

}
