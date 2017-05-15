#![feature(test)]

extern crate test;
extern crate truescad_primitive;
extern crate truescad_types;
use truescad_primitive::{Object, Sphere, SlabX, SlabY, SlabZ, Intersection, Twister};
use truescad_types::Float;

const STEPS: usize = 50;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn evaluate(obj: &Object) -> ::truescad_types::Float {
        let mut p = ::truescad_types::Point::new(0., 0., obj.bbox().min.z);
        let xd = (obj.bbox().max.x - obj.bbox().min.x) / (STEPS as Float);
        let yd = (obj.bbox().max.y - obj.bbox().min.y) / (STEPS as Float);
        let zd = (obj.bbox().max.z - obj.bbox().min.z) / (STEPS as Float);
        let slack = xd.min(yd.min(zd)) / 10.;
        let mut result = 0.;
        for _ in 0..STEPS {
            p.y = obj.bbox().min.y;
            for _ in 0..STEPS {
                p.x = obj.bbox().min.x;
                for _ in 0..STEPS {
                    result += obj.approx_value(p, slack);
                    p.x += xd;
                }
                p.y += yd;
            }
            p.z += zd;
        }
        return result;
    }

    fn normals(obj: &Object) -> ::truescad_types::Vector {
        let mut p = ::truescad_types::Point::new(0., 0., obj.bbox().min.z);
        let xd = (obj.bbox().max.x - obj.bbox().min.x) / (STEPS as Float);
        let yd = (obj.bbox().max.y - obj.bbox().min.y) / (STEPS as Float);
        let zd = (obj.bbox().max.z - obj.bbox().min.z) / (STEPS as Float);
        let mut result = ::truescad_types::Vector::new(0., 0., 0.);
        for _ in 0..STEPS {
            p.y = obj.bbox().min.y;
            for _ in 0..STEPS {
                p.x = obj.bbox().min.x;
                for _ in 0..STEPS {
                    result += obj.normal(p);
                    p.x += xd;
                }
                p.y += yd;
            }
            p.z += zd;
        }
        return result;
    }

    #[bench]
    fn sphere(b: &mut Bencher) {
        let object = Sphere::new(1.0);
        b.iter(|| evaluate(&*object as &Object));
    }
    #[bench]
    fn sphere_normals(b: &mut Bencher) {
        let object = Sphere::new(1.0);
        b.iter(|| normals(&*object as &Object));
    }

    fn create_cube() -> Box<Object> {
        Intersection::from_vec(vec![SlabX::new(1.), SlabY::new(1.), SlabZ::new(1.)], 0.).unwrap() as Box<Object>
    }

    #[bench]
    fn cube(b: &mut Bencher) {
        let object = create_cube();
        b.iter(|| evaluate(&*object as &Object));
    }
    #[bench]
    fn cube_normals(b: &mut Bencher) {
        let object = create_cube();
        b.iter(|| normals(&*object as &Object));
    }

    fn create_hollow_cube() -> Box<Object> {
        Intersection::difference_from_vec(vec![create_cube(), Sphere::new(0.5)], 0.2).unwrap() as Box<Object>
    }

    #[bench]
    fn hollow_cube(b: &mut Bencher) {
        let object = create_hollow_cube();
        b.iter(|| evaluate(&*object as &Object));
    }
    #[bench]
    fn hollow_cube_normals(b: &mut Bencher) {
        let object = create_hollow_cube();
        b.iter(|| normals(&*object as &Object));
    }

    #[bench]
    fn twisted_cube(b: &mut Bencher) {
        let object = Twister::new(create_cube(), 4.);
        b.iter(|| evaluate(&*object as &Object));
    }
    #[bench]
    fn twisted_cube_normals(b: &mut Bencher) {
        let object = Twister::new(create_cube(), 4.);
        b.iter(|| normals(&*object as &Object));
    }
}
