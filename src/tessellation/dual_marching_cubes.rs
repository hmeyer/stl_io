use Float;
use primitive::{BoundingBox, Object};
use tessellation::{BitSet, Mesh};
use tessellation::dual_marching_cubes_cell_configs::get_dmc_cell_configs;
use types::{Point, Vector};
use std::collections::HashMap;
use std::cell::RefCell;
use cgmath::EuclideanSpace;
use rand;

// How accurately find zero crossings.
const PRECISION: Float = 0.01;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Index(usize, usize, usize);


//  Corner indexes
//
//      6---------------7
//     /|              /|
//    / |             / |
//   /  |            /  |
//  4---------------5   |
//  |   |           |   |
//  |   2-----------|---3
//  |  /            |  /
//  | /             | /
//  |/              |/
//  0---------------1
#[derive(Clone, Copy)]
pub enum Corner {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}
// Corner connections
pub const CORNER_CONNS: [[Corner; 3]; 8] = [[Corner::B, Corner::C, Corner::E],
                                            [Corner::A, Corner::D, Corner::F],
                                            [Corner::A, Corner::D, Corner::G],
                                            [Corner::B, Corner::C, Corner::H],
                                            [Corner::A, Corner::F, Corner::G],
                                            [Corner::B, Corner::E, Corner::H],
                                            [Corner::C, Corner::E, Corner::H],
                                            [Corner::D, Corner::F, Corner::G]];

// Which corners does a edge connect:
pub const EDGE_DEF: [(Corner, Corner); 12] = [(Corner::A, Corner::B),
                                              (Corner::A, Corner::C),
                                              (Corner::A, Corner::E),
                                              (Corner::C, Corner::D),
                                              (Corner::B, Corner::D),
                                              (Corner::B, Corner::F),
                                              (Corner::E, Corner::F),
                                              (Corner::E, Corner::G),
                                              (Corner::C, Corner::G),
                                              (Corner::G, Corner::H),
                                              (Corner::F, Corner::H),
                                              (Corner::D, Corner::H)];
//  Edge indexes
//
//      +-------9-------+
//     /|              /|
//    7 |            10 |              ^
//   /  8            /  11            /
//  +-------6-------+   |     ^    higher indexes in y
//  |   |           |   |     |     /
//  |   +-------3---|---+     |    /
//  2  /            5  /  higher indexes
//  | 1             | 4      in z
//  |/              |/        |/
//  o-------0-------+         +-- higher indexes in x ---->
//
// Point o is the reference point of the current cell.
// All edges go from lower indexes to higher indexes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Edge {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
}

impl Edge {
    pub fn from_usize(e: usize) -> Edge {
        match e {
            0 => Edge::A,
            1 => Edge::B,
            2 => Edge::C,
            3 => Edge::D,
            4 => Edge::E,
            5 => Edge::F,
            6 => Edge::G,
            7 => Edge::H,
            8 => Edge::I,
            9 => Edge::J,
            10 => Edge::K,
            11 => Edge::L,
            _ => panic!("Not edge for {:?}", e),
        }
    }
    pub fn base(&self) -> Edge {
        Edge::from_usize(*self as usize % 3)
    }
}

// Offset of the end position of the first 3 edges - relative to current position.
const EDGE_END_OFFSET: [Index; 3] = [Index(1, 0, 0), Index(0, 1, 0), Index(0, 0, 1)];
const EDGE_END_OFFSET_VECTOR: [Vector; 3] = [Vector {
                                                 x: 1.,
                                                 y: 0.,
                                                 z: 0.,
                                             },
                                             Vector {
                                                 x: 0.,
                                                 y: 1.,
                                                 z: 0.,
                                             },
                                             Vector {
                                                 x: 0.,
                                                 y: 0.,

                                                 z: 1.,
                                             }];

// Cell offsets of edges
const EDGE_OFFSET: [Index; 12] = [Index(0, 0, 0),
                                  Index(0, 0, 0),
                                  Index(0, 0, 0),

                                  Index(0, 1, 0),
                                  Index(1, 0, 0),
                                  Index(1, 0, 0),

                                  Index(0, 0, 1),
                                  Index(0, 0, 1),
                                  Index(0, 1, 0),

                                  Index(0, 1, 1),
                                  Index(1, 0, 1),
                                  Index(1, 1, 0)];

// Quad definition for edges 0-2.
const QUADS: [[Edge; 4]; 3] = [[Edge::A, Edge::G, Edge::J, Edge::D],
                               [Edge::B, Edge::E, Edge::K, Edge::H],
                               [Edge::C, Edge::I, Edge::L, Edge::F]];

pub struct DualMarchingCubes {
    object: Box<Object>,
    bbox: BoundingBox,
    mesh: RefCell<Mesh>,
    // Map (EdgeSet, Index) -> index in mesh.vertices
    vertex_map: RefCell<HashMap<(BitSet, Index), usize>>,
    res: Float,
    value_grid: Vec<Vec<Vec<Float>>>,
    edge_grid: RefCell<HashMap<(Edge, Index), Point>>,
    cell_configs: Vec<Vec<BitSet>>,
}

impl DualMarchingCubes {
    // Constructor
    // obj: Object to tessellate
    // res: resolution
    pub fn new(obj: Box<Object>, res: Float) -> DualMarchingCubes {
        let bbox = obj.bbox().dilate(res * 1.1);
        DualMarchingCubes {
            object: obj,
            bbox: bbox,
            mesh: RefCell::new(Mesh {
                vertices: Vec::new(),
                faces: Vec::new(),
            }),
            vertex_map: RefCell::new(HashMap::new()),
            res: res,
            value_grid: Vec::new(),
            edge_grid: RefCell::new(HashMap::new()),
            cell_configs: get_dmc_cell_configs(),
        }
    }
    pub fn tesselate(&mut self) -> Mesh {
        loop {
            match self.try_tesselate() {
                Ok(mesh) => return mesh,
                Err(x) => {
                    let padding = self.res / (1. + rand::random::<f64>().abs());
                    println!("Error: {:?}. Padding bbox by {:?} and retrying.",
                             x,
                             padding);
                    self.bbox = self.bbox.dilate(padding);
                    self.value_grid.clear();
                    self.mesh.borrow_mut().vertices.clear();
                    self.mesh.borrow_mut().faces.clear();
                }
            }
        }
    }
    // This method does the main work of tessellation.
    fn try_tesselate(&mut self) -> Result<Mesh, String> {
        let res = self.res;
        let dim = [(self.bbox.dim().x / res).ceil() as usize,
                   (self.bbox.dim().y / res).ceil() as usize,
                   (self.bbox.dim().z / res).ceil() as usize];

        // Store object values in value_grid
        let mut p = Point::new(0., 0., self.bbox.min.z);
        self.value_grid.reserve(dim[2]);
        for _ in 0..dim[2] {
            let mut values_xy = Vec::with_capacity(dim[1]);
            p.y = self.bbox.min.y;
            for _ in 0..dim[1] {
                let mut values_x = Vec::with_capacity(dim[0]);
                p.x = self.bbox.min.x;
                for _ in 0..dim[0] {
                    let val = self.object.approx_value(p, res);
                    if val == 0. {
                        return Err(format!("Hit zero on grid position {:?}", p));
                    }
                    values_x.push(val);
                    p.x += res;
                }
                values_xy.push(values_x);
                p.y += res;
            }
            self.value_grid.push(values_xy);
            p.z += res;
        }

        let edge_end_offset: [Vector; 3] = [EDGE_END_OFFSET_VECTOR[0] * res,
                                            EDGE_END_OFFSET_VECTOR[1] * res,
                                            EDGE_END_OFFSET_VECTOR[2] * res];

        // Store crossing positions of edges in edge_grid
        let mut p = Point::new(0., 0., self.bbox.min.z);
        {
            let mut edge_grid = self.edge_grid.borrow_mut();
            edge_grid.clear();
            for z in 0..dim[2] - 1 {
                p.y = self.bbox.min.y;
                for y in 0..dim[1] - 1 {
                    p.x = self.bbox.min.x;
                    for x in 0..dim[0] - 1 {
                        for edge in [Edge::A, Edge::B, Edge::C].iter() {
                            // We don't need any start offset here, since edges 0-2 start at the
                            // current cell.
                            let eo = EDGE_END_OFFSET[*edge as usize];   // end offset
                            if let Some(point) =
                                   self.find_zero(p,
                                                  self.value_grid[z][y][x],
                                                  p + edge_end_offset[*edge as usize],
                                                  self.value_grid[z + eo.2][y + eo.1][x + eo.0]) {
                                edge_grid.insert((*edge, Index(x, y, z)), point);
                            }
                        }
                        p.x += res;
                    }
                    p.y += res;
                }
                p.z += res;
            }
        }

        for (&(edge_index, ref idx), _) in self.edge_grid.borrow().iter() {
            self.compute_quad(edge_index, *idx);
        }

        println!("computed mesh with {:?} faces.",
                 self.mesh.borrow().faces.len());

        Ok(self.mesh.borrow().clone())
    }

    fn offset(idx: Index, offset: Index) -> Index {
        let mut idx = idx;
        idx.0 += offset.0;
        idx.1 += offset.1;
        idx.2 += offset.2;
        idx
    }

    fn neg_offset(idx: Index, offset: Index) -> Index {
        let mut idx = idx;
        idx.0 -= offset.0;
        idx.1 -= offset.1;
        idx.2 -= offset.2;
        idx
    }

    fn get_edge_point(&self, edge: Edge, cell_idx: Index) -> Point {
        let data_idx = DualMarchingCubes::offset(cell_idx, EDGE_OFFSET[edge as usize]);
        let data_edge = edge.base();
        if let Some(p) = self.edge_grid
                             .borrow()
                             .get(&(edge.base(), data_idx)) {
            return p.clone();
        }
        panic!("could not find edge_point: {:?} {:?},-> {:?} {:?}",
               edge,
               data_edge,
               cell_idx,
               data_idx);
    }

    // Return the Point index (in self.mesh.vertices) the the point belonging to edge/idx.
    fn compute_cell_point(&self, edge: Edge, idx: Index) -> usize {
        let edge_set = self.get_connected_edges(edge, self.bitset_for_cell(idx));
        if let Some(index) = self.vertex_map.borrow().get(&(edge_set, idx)) {
            return *index;
        }
        let mut sum = Vector::new(0., 0., 0.);
        let mut num = 0.;
        for edge in edge_set.into_iter() {
            sum += self.get_edge_point(Edge::from_usize(edge), idx).to_vec();
            num += 1.;
        }
        sum /= num;
        let ref mut vertex_list = self.mesh.borrow_mut().vertices;
        let result = vertex_list.len();
        vertex_list.push([sum.x, sum.y, sum.z]);
        return result;
    }

    fn bitset_for_cell(&self, idx: Index) -> BitSet {
        let mut result = BitSet::new(0);
        for z in 0..2 {
            let plane = &self.value_grid[idx.2 + z];
            for y in 0..2 {
                let row = &plane[idx.1 + y];
                for x in 0..2 {
                    if row[idx.0 + x] < 0. {
                        result.set(z << 2 | y << 1 | x);
                    }
                }
            }
        }
        result
    }

    // Return a BitSet containing all egdes connected to "edge" in this cell.
    fn get_connected_edges(&self, edge: Edge, cell: BitSet) -> BitSet {
        for edge_set in self.cell_configs[cell.as_usize()].iter() {
            if edge_set.get(edge as usize) {
                return *edge_set;
            }
        }
        panic!("Did not find edge_set for {:?} and {:?}", edge, cell);
    }

    // Compute a quad for the given edge and append it to the list.
    fn compute_quad(&self, edge: Edge, idx: Index) {
        debug_assert!((edge as usize) < 4);
        debug_assert!(idx.0 > 0);
        debug_assert!(idx.1 > 0);
        debug_assert!(idx.2 > 0);

        let mut p = Vec::with_capacity(4);
        for quad_egde in QUADS[edge as usize].iter() {
            p.push(self.compute_cell_point(
                *quad_egde, DualMarchingCubes::neg_offset(idx, EDGE_OFFSET[*quad_egde as usize])));
        }
        let ref mut face_list = self.mesh.borrow_mut().faces;
        if self.value_grid[idx.2][idx.1][idx.0] > 0. {
            // Normal order, if vertex is outside.
            face_list.push([p[0], p[1], p[2]]);
            face_list.push([p[2], p[3], p[0]]);
        } else {
            // Reverse order, if vertex is inside.
            face_list.push([p[2], p[1], p[0]]);
            face_list.push([p[0], p[3], p[2]]);
        }
    }

    // If a is inside the object and b outside - this method return the point on the line between
    // a and b where the object edge is.
    // av and bv represent the object values at a and b.
    fn find_zero(&self, a: Point, av: Float, b: Point, bv: Float) -> Option<Point> {
        debug_assert!(av == self.object.approx_value(a, self.res));
        debug_assert!(bv == self.object.approx_value(b, self.res));
        assert!(a != b);
        if av.signum() == bv.signum() {
            return None;
        }
        if av.abs() < PRECISION * self.res {
            return Some(a);
        }
        if bv.abs() < PRECISION * self.res {
            return Some(b);
        }
        // Linear interpolation of the zero crossing.
        let n = a + (b - a) * (av.abs() / (bv - av).abs());
        let nv = self.object.approx_value(n, self.res);

        if av.signum() != nv.signum() {
            return self.find_zero(a, av, n, nv);
        } else {
            return self.find_zero(n, nv, b, bv);
        }
    }
}
