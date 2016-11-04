use xplicit_types::{EPSILON, Float, NAN, Point};
use xplicit_primitive::BoundingBox;
use Plane;
use cgmath::{EuclideanSpace, InnerSpace};
use na;
use na::Inverse;


// Quadratic error function

#[derive(Debug)]
pub struct Qef {
    // Point closest to all planes.
    pub solution: na::Vector3<Float>,
    sum: na::Vector3<Float>,
    pub num: usize,
    // Upper right triangle of AT * A
    ata: [Float; 6],
    // Vector AT * B
    atb: na::Vector3<Float>,
    // Scalar BT * B
    btb: Float,
    pub error: Float,
    bbox: BoundingBox,
}


impl Qef {
    pub fn new(planes: &[Plane], bbox: BoundingBox) -> Qef {
        let mut qef = Qef {
            solution: na::Vector3::new(NAN, NAN, NAN),
            sum: na::Vector3::new(0., 0., 0.),
            num: planes.len(),
            ata: [0.; 6],
            atb: na::Vector3::new(0., 0., 0.),
            btb: 0.,
            error: NAN,
            bbox: bbox,
        };
        for p in planes {
            qef.ata[0] += p.n[0] * p.n[0];
            qef.ata[1] += p.n[0] * p.n[1];
            qef.ata[2] += p.n[0] * p.n[2];
            qef.ata[3] += p.n[1] * p.n[1];
            qef.ata[4] += p.n[1] * p.n[2];
            qef.ata[5] += p.n[2] * p.n[2];
            let pn = p.p.to_vec().dot(p.n);
            qef.atb[0] += p.n[0] * pn;
            qef.atb[1] += p.n[1] * pn;
            qef.atb[2] += p.n[2] * pn;
            qef.btb += pn * pn;
            qef.sum[0] += p.p[0];
            qef.sum[1] += p.p[1];
            qef.sum[2] += p.p[2];
        }
        qef
    }
    pub fn solve(&mut self) {
        let m = &self.ata;
        let ma = na::Matrix3::new(m[0], m[1], m[2], m[1], m[3], m[4], m[2], m[4], m[5]);
        let mean = self.sum / self.num as Float;
        if let Some(inv) = ma.inverse() {
            let b_rel_mean = self.atb - ma * mean;
            self.solution = b_rel_mean * inv + mean;
        } else {
            self.solution = self.sum / self.num as Float;
        }
        // NAN-solution will also not be contained in the bbox.
        if !self.bbox.contains(Point::new(self.solution.x, self.solution.y, self.solution.z)) {
            let accuracy = (self.bbox.max.x - self.bbox.min.x) / 100.0;
            self.solution = self.search_solution(accuracy, &mut self.bbox.clone(), &ma);
            debug_assert!(self.bbox
                              .dilate(accuracy)
                              .contains(Point::new(self.solution.x,
                                                   self.solution.y,
                                                   self.solution.z)),
                          "{:?} outside of {:?}",
                          self.solution,
                          self);
        }
        self.error = self.error(&self.solution, &ma);
    }
    fn search_solution(&self,
                       accuracy: Float,
                       bbox: &mut BoundingBox,
                       ma: &na::Matrix3<Float>)
                       -> na::Vector3<Float> {
        let mid = (bbox.max.to_vec() + bbox.min.to_vec()) * 0.5;
        let na_mid = na::Vector3::new(mid.x, mid.y, mid.z);
        if bbox.max.x - bbox.min.x < accuracy {
            return na_mid;
        }
        let mid_error = self.error(&na_mid, ma);
        for dim in 0..3 {
            let mut d_mid = na_mid.clone();
            d_mid[dim] += EPSILON;
            let d_error = self.error(&d_mid, ma);
            if d_error < mid_error {
                bbox.min[dim] = mid[dim];
            } else {
                bbox.max[dim] = mid[dim];
            }
        }
        self.search_solution(accuracy, bbox, ma)
    }
    fn error(&self, point: &na::Vector3<Float>, ma: &na::Matrix3<Float>) -> Float {
        self.btb - 2. * na::dot(point, &self.atb) + na::dot(point, &(*ma * *point))
    }
    pub fn merge(&mut self, other: &Qef) {
        for i in 0..6 {
            self.ata[i] += other.ata[i];
        }
        self.atb += other.atb;
        self.btb += other.btb;
        self.sum += other.sum;
        self.num += other.num;
        self.bbox = self.bbox.union(&other.bbox);
    }
}


#[cfg(test)]
mod tests {
    use super::Qef;
    use xplicit_primitive::BoundingBox;
    use xplicit_types::{Point, Vector};
    use super::super::Plane;
    use na;
    use na::{ApproxEq, Norm};
    use cgmath::InnerSpace;

    #[test]
    fn origin() {
        let origin = Point::new(0., 0., 0.);
        let mut qef = Qef::new(&[Plane {
                                     p: origin.clone(),
                                     n: Vector::new(0., 1., 2.).normalize(),
                                 },
                                 Plane {
                                     p: origin.clone(),
                                     n: Vector::new(1., 2., 3.).normalize(),
                                 },
                                 Plane {
                                     p: origin.clone(),
                                     n: Vector::new(2., 3., 4.).normalize(),
                                 }],
                               BoundingBox::new(Point::new(0., 0., 0.), Point::new(1., 1., 1.)));
        qef.solve();
        assert!(qef.solution.norm() < 0.01,
                "{:?} nowhere near origin",
                qef.solution);
    }

    #[test]
    fn points_on_cube_solution_in_origin() {
        let mut qef = Qef::new(&[Plane {
                                     p: Point::new(1., 0., 0.),
                                     n: Vector::new(0., 1., 1.).normalize(),
                                 },
                                 Plane {
                                     p: Point::new(0., 1., 0.),
                                     n: Vector::new(1., 0., 1.).normalize(),
                                 },
                                 Plane {
                                     p: Point::new(0., 0., 1.),
                                     n: Vector::new(1., 1., 0.).normalize(),
                                 }],
                               BoundingBox::new(Point::new(0., 0., 0.), Point::new(1., 1., 1.)));
        qef.solve();
        assert!(qef.solution.approx_eq(&na::Vector3::new(0., 0., 0.)));
    }

    #[test]
    fn points_on_origin_solution_on_cube() {
        let mut qef = Qef::new(&[Plane {
                                     p: Point::new(1., 0., 0.),
                                     n: Vector::new(1., 0., 0.),
                                 },
                                 Plane {
                                     p: Point::new(0., 2., 0.),
                                     n: Vector::new(0., 1., 0.),
                                 },
                                 Plane {
                                     p: Point::new(0., 0., 3.),
                                     n: Vector::new(0., 0., 1.),
                                 }],
                               BoundingBox::new(Point::new(0., 0., 0.), Point::new(1., 2., 3.)));
        qef.solve();
        let expected_solution = na::Vector3::new(1., 2., 3.);
        assert!(qef.solution.approx_eq(&expected_solution),
                "{} != {}",
                qef.solution,
                expected_solution);
    }
}
