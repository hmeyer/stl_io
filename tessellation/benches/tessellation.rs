#[macro_use]
extern crate bencher;
extern crate truescad_tessellation;
extern crate truescad_primitive;
extern crate truescad_types;
use bencher::Bencher;
use truescad_primitive::{Object, Sphere, SlabX, SlabY, SlabZ, Intersection};

fn create_cube() -> Box<Object> {
    Intersection::from_vec(vec![SlabX::new(1.), SlabY::new(1.), SlabZ::new(1.)], 0.).unwrap() as Box<Object>
}

fn create_hollow_cube() -> Box<Object> {
    Intersection::difference_from_vec(vec![create_cube(), Sphere::new(0.5)], 0.2).unwrap() as Box<Object>
}

fn tessellation(b: &mut Bencher) {

    let mut object = create_hollow_cube();
    object.set_parameters(&truescad_primitive::PrimitiveParameters {
        fade_range: 0.1,
        r_multiplier: 1.0,
    });

    let mut tess = truescad_tessellation::ManifoldDualContouring::new(object, 0.01, 0.1);
    b.iter(|| tess.tessellate().expect("tessellation unsuccessful"));
}

// TODO: Create benchmarks for the different sub-methods
benchmark_group!(bench_tessellation, tessellation);
benchmark_main!(bench_tessellation);
