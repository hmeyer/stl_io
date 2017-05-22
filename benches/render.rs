#[macro_use]
extern crate bencher;
extern crate truescad_luascad;
extern crate truescad_primitive;
extern crate truescad_types;
extern crate truescad;
use bencher::Bencher;



static TWISTED_CUBE: &'static str  = "
t=Twist(Difference({Box(1,1,1,.2), Sphere(0.5)},.2),4)
t:rotate(-math.pi/4,0,0)
build(t)";
const XRES: usize = 320;
const YRES: usize = 200;
const NUM_CHANNELS: usize = 4;

fn render(b: &mut Bencher) {

    let mut object = ::truescad_luascad::eval(TWISTED_CUBE).unwrap();
    object.as_mut().unwrap().set_parameters(&truescad_primitive::PrimitiveParameters {
        fade_range: 0.1,
        r_multiplier: 1.0,
    });

    let mut renderer = ::truescad::render::Renderer::new();
    renderer.set_object(object);
    let mut buffer = [0u8; XRES * YRES * NUM_CHANNELS];

    b.iter(|| renderer.draw_on_buf(&mut buffer, XRES as i32, YRES as i32));
}

benchmark_group!(bench_render, render);
benchmark_main!(bench_render);
