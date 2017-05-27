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

fn creat_tessellation() -> truescad_tessellation::ManifoldDualContouring {
    let mut object = create_hollow_cube();
    object.set_parameters(&truescad_primitive::PrimitiveParameters {
        fade_range: 0.1,
        r_multiplier: 1.0,
    });
    return truescad_tessellation::ManifoldDualContouring::new(object, 0.015, 0.1);
}

fn sample_value_grid(b: &mut Bencher) {
    let tess = creat_tessellation();
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.tessellation_step1()
    });
}

fn compact_value_grid(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.compact_value_grid()
    });
}

fn generate_edge_grid(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    tess.compact_value_grid();
    b.iter(|| {
        let mut my_tess = tess.clone();
        my_tess.generate_edge_grid()
    });
}

fn generate_leaf_vertices(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    b.iter(|| {
        let my_tess = tess.clone();
        my_tess.generate_leaf_vertices()
    });
}

fn subsample_octtree(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    b.iter(|| {
        let mut my_tess = tess.clone();
        loop {
            let next = truescad_tessellation::subsample_octtree(my_tess.vertex_octtree.last().unwrap());
            if next.len() == my_tess.vertex_octtree.last().unwrap().len() {
                break;
            }
            my_tess.vertex_octtree.push(next);
        }
    });
}

fn solve_qefs(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    loop {
        let next = truescad_tessellation::subsample_octtree(tess.vertex_octtree.last().unwrap());
        if next.len() == tess.vertex_octtree.last().unwrap().len() {
            break;
        }
        tess.vertex_octtree.push(next);
    }
    b.iter(|| {
        let my_tess = tess.clone();
        my_tess.solve_qefs();
    });
}

fn compute_quad(b: &mut Bencher) {
    let mut tess = creat_tessellation();
    tess.tessellation_step1();
    tess.compact_value_grid();
    tess.generate_edge_grid();
    let (leafs, index_map) = tess.generate_leaf_vertices();
    tess.vertex_index_map = index_map;
    tess.vertex_octtree.push(leafs);
    loop {
        let next = truescad_tessellation::subsample_octtree(tess.vertex_octtree.last().unwrap());
        if next.len() == tess.vertex_octtree.last().unwrap().len() {
            break;
        }
        tess.vertex_octtree.push(next);
    }
    tess.solve_qefs();
    b.iter(|| {
        let my_tess = tess.clone();
        for edge_index in my_tess.edge_grid.borrow().keys() {
            my_tess.compute_quad(*edge_index);
        }
    });
}



benchmark_group!(bench_tessellation, sample_value_grid,
                                     compact_value_grid,
                                     generate_edge_grid,
                                     generate_leaf_vertices,
                                     subsample_octtree,
                                     solve_qefs,
                                     compute_quad);
benchmark_main!(bench_tessellation);
