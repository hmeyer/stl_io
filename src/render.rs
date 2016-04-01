//pub use primitive::{Sphere};


//pub type Ray = ray::Ray3<float>;
//pub type Point = Point<float>;
use std::cmp;
use Float;
use types::{Point, Vector, Ray, Transform};
use primitive::{Object, Sphere, Union, /*Neg, Intersection, */Subtraction};

const EPSILON: Float = 0.001;
const MAXVAL: Float = 100.;
// Normalized Vector for diagonally left above

#[derive(Clone, Copy)]
pub struct Renderer {
    light_dir: Vector,
    trans: Transform
}

fn create_object() -> Box<Object> {
    let sphere1 = Box::new(Sphere::new(0.8));

    let mut sphere2 = Box::new(Sphere::new(0.3));
    sphere2.translate(Vector::new(0.15, -0.1, 1.));

    let mut sphere3 = Box::new(Sphere::new(0.5));
    sphere3.translate(Vector::new(-0.1, -0.1, 0.3));

    return Box::new(Union::new(Box::new(Subtraction::new(sphere1, sphere2)), sphere3))
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector::new(-0.6666666666666666,
                                           0.6666666666666666,
                                          -0.3333333333333333),
                   trans: Transform::identity() }
    }

    pub fn rotate_from_screen(&mut self, x: f64, y: f64) {
        let v = Vector::new(y as Float, x as Float, 0.);
        let other = Transform::rotate(&v);
        self.trans = self.trans.concat(&other);
    }

    pub fn translate_from_screen(&mut self, x: f64, y: f64) {
        let v = Vector::new(-x as Float, y as Float, 0.);
        let other = Transform::translate(&v);
        self.trans = self.trans.concat(&other);
    }

    fn cast_ray(&self, obj: &Object, r: &Ray, light_dir: &Vector, origin_value: Float) -> Float {
        let mut cr = *r;
        let mut value = origin_value;

        loop {
            cr.dir = cr.dir.normalize();
            cr.origin = cr.origin + cr.dir * value;
            value = obj.value(&cr.origin);
            if value >  MAXVAL {
                return 0.;
            }

            if value < EPSILON {
                break;
            }
        }
        let norm = obj.normal(&cr.origin);
        let dot = norm.dot(*light_dir);
        if dot < 0. { return 0.; }
        return dot;
    }

    pub fn draw_on_buf(&self, buf: &mut [u8], width: i32, height: i32) {
        let scale = 1. / cmp::min(width, height) as Float;
        let w2 = width / 2;
        let h2 = height / 2;

        let dir_front = self.trans.t_vector(Vector::new(0., 0., 1.));
        let dir_rl = self.trans.t_vector(Vector::new(1., 0., 0.));
        let dir_tb = self.trans.t_vector(Vector::new(0., -1., 0.));

        let light_dir = self.trans.t_vector(self.light_dir);

        let ray_origin = self.trans.t_point(Point::new(0., 0., -2.));
        let mut ray = Ray::new(ray_origin, dir_front);

        let my_obj = create_object();
        let origin_value = my_obj.value(&ray.origin);


        let mut index = 0 as usize;
        for y in 0..height {
            let dir_row = dir_front + dir_tb * ((y - h2) as Float * scale);

            for x in 0..width {
                ray.dir = dir_row + dir_rl * ((x - w2) as Float * scale);

                let v = self.cast_ray(&*my_obj, &ray, &light_dir, origin_value);

                let b = (255.0 *  v * v) as u8;

                buf[index] = b; index += 1;
                buf[index] = b; index += 1;
                buf[index] = b; index += 1;
                index +=1;
            }
        }
    }

}
