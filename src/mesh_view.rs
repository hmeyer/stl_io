extern crate kiss3d;
use self::kiss3d::window::Window;
use self::kiss3d::light::Light;

pub fn show_mesh() {
    let mut window = Window::new("Kiss3d: cube");
    let mut c = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    while window.render() {}
}
