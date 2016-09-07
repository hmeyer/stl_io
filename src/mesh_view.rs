extern crate kiss3d;
extern crate nalgebra as na;
use std::rc::Rc;
use std::cell::RefCell;
use self::kiss3d::window::Window;
use self::kiss3d::light::Light;
use self::kiss3d::resource::Mesh;
use tessellation;

pub fn show_mesh(mesh: &tessellation::Mesh) {
    let mut window = Window::new("MeshView");
    let scale = na::Vector3::new(1.0, 1.0, 1.0);
    let mut c = window.add_mesh(tessellation_to_kiss3d_mesh(mesh), scale);

    c.set_color(1.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    while window.render() {}

}

fn tessellation_to_kiss3d_mesh(mesh: &tessellation::Mesh) -> Rc<RefCell<Mesh>> {
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
