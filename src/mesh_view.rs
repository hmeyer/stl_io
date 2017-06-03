use kiss3d::light::Light;
use kiss3d::resource::Mesh;
use kiss3d::window::Window;
use nalgebra as na;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::{Arc, Mutex, ONCE_INIT, Once};
use truescad_tessellation;

#[derive(Clone)]
struct SingletonWindow {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    inner: Arc<Mutex<Window>>,
}

fn singleton_window() -> SingletonWindow {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonWindow = 0 as *const SingletonWindow;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let window = SingletonWindow { inner: Arc::new(Mutex::new(Window::new("MeshView"))) };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(window));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

pub fn show_mesh(mesh: &truescad_tessellation::Mesh) {
    let window_mutex = singleton_window();
    let mut window = window_mutex.inner.lock().unwrap();
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
