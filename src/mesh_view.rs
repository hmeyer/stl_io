use std::rc::Rc;
use std::cell::RefCell;
use nalgebra as na;
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use truescad_tessellation;
use std::sync::{ONCE_INIT, Once};

pub fn show_mesh(mesh: &truescad_tessellation::Mesh) {
    static mut KISS3D_SINGLETON: Option<Window> = None;
    static INIT: Once = ONCE_INIT;
    let window = unsafe {
        INIT.call_once(|| {
            KISS3D_SINGLETON = Some(Window::new("MeshView"));
        });
        KISS3D_SINGLETON.as_mut().unwrap()
    };
    window.glfw_window_mut().set_should_close(false);
    window.glfw_window_mut().restore();

    let scale = na::Vector3::new(1.0, 1.0, 1.0);
    let mut object_node = window.add_mesh(tessellation_to_kiss3d_mesh(mesh), scale);

    object_node.set_color(1.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);
    window.show();

    while window.render() {}
    window.remove(&mut object_node);
    window.hide();
}

fn tessellation_to_kiss3d_mesh(mesh: &truescad_tessellation::Mesh) -> Rc<RefCell<Mesh>> {
    let mut na_verts = Vec::new();
    let mut na_faces = Vec::new();
    for face in mesh.faces.iter() {
        let i = na_verts.len();
        na_faces.push(na::Point3::new(i as u32, (i + 1) as u32, (i + 2) as u32));
        for index in face.iter() {
            let p = &mesh.vertices[*index];
            na_verts.push(na::Point3::new(p[0] as f32, p[1] as f32, p[2] as f32));
        }
    }
    Rc::new(RefCell::new(Mesh::new(na_verts, na_faces, None, None, true)))
}
