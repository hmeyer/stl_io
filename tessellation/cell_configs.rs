use bitset::BitSet;


// CELL_CONFIGS stores the different cell configurations as suggested by Nielson.
// Each cell has 8 corners. Each corner can be either inside (true) or outside (false).
// This results in 256 different configurations. Due to symmetry those can be reduced to 23
// distinct configs.

// For each corner config, return all edges, that are connected to a single point. These egdes are
// stored in a BitSet. Since there might be more than 1 point, store  a slice of BitSets.

pub const CELL_CONFIGS: &'static [&'static [BitSet]] = &[&[],
                                                         &[BitSet(7)],
                                                         &[BitSet(49)],
                                                         &[BitSet(54)],
                                                         &[BitSet(266)],
                                                         &[BitSet(269)],
                                                         &[BitSet(49), BitSet(266)],
                                                         &[BitSet(316)],
                                                         &[BitSet(2072)],
                                                         &[BitSet(7), BitSet(2072)],
                                                         &[BitSet(2089)],
                                                         &[BitSet(2094)],
                                                         &[BitSet(2322)],
                                                         &[BitSet(2325)],
                                                         &[BitSet(2339)],
                                                         &[BitSet(2340)],
                                                         &[BitSet(196)],
                                                         &[BitSet(195)],
                                                         &[BitSet(49), BitSet(196)],
                                                         &[BitSet(242)],
                                                         &[BitSet(266), BitSet(196)],
                                                         &[BitSet(457)],
                                                         &[BitSet(49), BitSet(266), BitSet(196)],
                                                         &[BitSet(504)],
                                                         &[BitSet(2072), BitSet(196)],
                                                         &[BitSet(195), BitSet(2072)],
                                                         &[BitSet(2089), BitSet(196)],
                                                         &[BitSet(2282)],
                                                         &[BitSet(2322), BitSet(196)],
                                                         &[BitSet(2513)],
                                                         &[BitSet(2339), BitSet(196)],
                                                         &[BitSet(2528)],
                                                         &[BitSet(1120)],
                                                         &[BitSet(7), BitSet(1120)],
                                                         &[BitSet(1105)],
                                                         &[BitSet(1110)],
                                                         &[BitSet(266), BitSet(1120)],
                                                         &[BitSet(269), BitSet(1120)],
                                                         &[BitSet(1105), BitSet(266)],
                                                         &[BitSet(1372)],
                                                         &[BitSet(2072), BitSet(1120)],
                                                         &[BitSet(7), BitSet(2072), BitSet(1120)],
                                                         &[BitSet(3145)],
                                                         &[BitSet(3150)],
                                                         &[BitSet(2322), BitSet(1120)],
                                                         &[BitSet(2325), BitSet(1120)],
                                                         &[BitSet(3395)],
                                                         &[BitSet(3396)],
                                                         &[BitSet(1188)],
                                                         &[BitSet(1187)],
                                                         &[BitSet(1173)],
                                                         &[BitSet(1170)],
                                                         &[BitSet(266), BitSet(1188)],
                                                         &[BitSet(1449)],
                                                         &[BitSet(1173), BitSet(266)],
                                                         &[BitSet(1432)],
                                                         &[BitSet(2072), BitSet(1188)],
                                                         &[BitSet(1187), BitSet(2072)],
                                                         &[BitSet(3213)],
                                                         &[BitSet(3210)],
                                                         &[BitSet(2322), BitSet(1188)],
                                                         &[BitSet(3505)],
                                                         &[BitSet(3463)],
                                                         &[BitSet(3456)],
                                                         &[BitSet(896)],
                                                         &[BitSet(7), BitSet(896)],
                                                         &[BitSet(49), BitSet(896)],
                                                         &[BitSet(54), BitSet(896)],
                                                         &[BitSet(650)],
                                                         &[BitSet(653)],
                                                         &[BitSet(49), BitSet(650)],
                                                         &[BitSet(700)],
                                                         &[BitSet(2072), BitSet(896)],
                                                         &[BitSet(7), BitSet(2072), BitSet(896)],
                                                         &[BitSet(2089), BitSet(896)],
                                                         &[BitSet(2094), BitSet(896)],
                                                         &[BitSet(2706)],
                                                         &[BitSet(2709)],
                                                         &[BitSet(2723)],
                                                         &[BitSet(2724)],
                                                         &[BitSet(836)],
                                                         &[BitSet(835)],
                                                         &[BitSet(49), BitSet(836)],
                                                         &[BitSet(882)],
                                                         &[BitSet(590)],
                                                         &[BitSet(585)],
                                                         &[BitSet(49), BitSet(590)],
                                                         &[BitSet(632)],
                                                         &[BitSet(2072), BitSet(836)],
                                                         &[BitSet(835), BitSet(2072)],
                                                         &[BitSet(2089), BitSet(836)],
                                                         &[BitSet(2922)],
                                                         &[BitSet(2646)],
                                                         &[BitSet(2641)],
                                                         &[BitSet(2663)],
                                                         &[BitSet(2656)],
                                                         &[BitSet(1120), BitSet(896)],
                                                         &[BitSet(7), BitSet(1120), BitSet(896)],
                                                         &[BitSet(1105), BitSet(896)],
                                                         &[BitSet(1110), BitSet(896)],
                                                         &[BitSet(650), BitSet(1120)],
                                                         &[BitSet(653), BitSet(1120)],
                                                         &[BitSet(1105), BitSet(650)],
                                                         &[BitSet(1756)],
                                                         &[BitSet(2072),
                                                           BitSet(1120),
                                                           BitSet(896)],
                                                         &[BitSet(7),
                                                           BitSet(2072),
                                                           BitSet(1120),
                                                           BitSet(896)],
                                                         &[BitSet(3145), BitSet(896)],
                                                         &[BitSet(3150), BitSet(896)],
                                                         &[BitSet(2706), BitSet(1120)],
                                                         &[BitSet(2709), BitSet(1120)],
                                                         &[BitSet(3779)],
                                                         &[BitSet(3780)],
                                                         &[BitSet(1828)],
                                                         &[BitSet(1827)],
                                                         &[BitSet(1813)],
                                                         &[BitSet(1810)],
                                                         &[BitSet(1582)],
                                                         &[BitSet(1577)],
                                                         &[BitSet(1567)],
                                                         &[BitSet(1560)],
                                                         &[BitSet(2072), BitSet(1828)],
                                                         &[BitSet(1827), BitSet(2072)],
                                                         &[BitSet(3853)],
                                                         &[BitSet(3850)],
                                                         &[BitSet(3638)],
                                                         &[BitSet(3633)],
                                                         &[BitSet(7), BitSet(3584)],
                                                         &[BitSet(3584)],
                                                         &[BitSet(3584)],
                                                         &[BitSet(7), BitSet(3584)],
                                                         &[BitSet(49), BitSet(3584)],
                                                         &[BitSet(54), BitSet(3584)],
                                                         &[BitSet(266), BitSet(3584)],
                                                         &[BitSet(269), BitSet(3584)],
                                                         &[BitSet(49), BitSet(266), BitSet(3584)],
                                                         &[BitSet(316), BitSet(3584)],
                                                         &[BitSet(1560)],
                                                         &[BitSet(7), BitSet(1560)],
                                                         &[BitSet(1577)],
                                                         &[BitSet(1582)],
                                                         &[BitSet(1810)],
                                                         &[BitSet(1813)],
                                                         &[BitSet(1827)],
                                                         &[BitSet(1828)],
                                                         &[BitSet(196), BitSet(3584)],
                                                         &[BitSet(195), BitSet(3584)],
                                                         &[BitSet(49), BitSet(196), BitSet(3584)],
                                                         &[BitSet(242), BitSet(3584)],
                                                         &[BitSet(266), BitSet(196), BitSet(3584)],
                                                         &[BitSet(457), BitSet(3584)],
                                                         &[BitSet(49),
                                                           BitSet(266),
                                                           BitSet(196),
                                                           BitSet(3584)],
                                                         &[BitSet(504), BitSet(3584)],
                                                         &[BitSet(1560), BitSet(196)],
                                                         &[BitSet(195), BitSet(1560)],
                                                         &[BitSet(1577), BitSet(196)],
                                                         &[BitSet(1770)],
                                                         &[BitSet(1810), BitSet(196)],
                                                         &[BitSet(2001)],
                                                         &[BitSet(1827), BitSet(196)],
                                                         &[BitSet(2016)],
                                                         &[BitSet(2656)],
                                                         &[BitSet(7), BitSet(2656)],
                                                         &[BitSet(2641)],
                                                         &[BitSet(2646)],
                                                         &[BitSet(266), BitSet(2656)],
                                                         &[BitSet(269), BitSet(2656)],
                                                         &[BitSet(2641), BitSet(266)],
                                                         &[BitSet(2908)],
                                                         &[BitSet(632)],
                                                         &[BitSet(7), BitSet(632)],
                                                         &[BitSet(585)],
                                                         &[BitSet(590)],
                                                         &[BitSet(882)],
                                                         &[BitSet(885)],
                                                         &[BitSet(835)],
                                                         &[BitSet(836)],
                                                         &[BitSet(2724)],
                                                         &[BitSet(2723)],
                                                         &[BitSet(2709)],
                                                         &[BitSet(2706)],
                                                         &[BitSet(266), BitSet(2724)],
                                                         &[BitSet(2985)],
                                                         &[BitSet(2709), BitSet(266)],
                                                         &[BitSet(2968)],
                                                         &[BitSet(700)],
                                                         &[BitSet(699)],
                                                         &[BitSet(653)],
                                                         &[BitSet(650)],
                                                         &[BitSet(950)],
                                                         &[BitSet(49), BitSet(896)],
                                                         &[BitSet(903)],
                                                         &[BitSet(896)],
                                                         &[BitSet(3456)],
                                                         &[BitSet(7), BitSet(3456)],
                                                         &[BitSet(49), BitSet(3456)],
                                                         &[BitSet(54), BitSet(3456)],
                                                         &[BitSet(3210)],
                                                         &[BitSet(3213)],
                                                         &[BitSet(49), BitSet(3210)],
                                                         &[BitSet(3260)],
                                                         &[BitSet(1432)],
                                                         &[BitSet(7), BitSet(1432)],
                                                         &[BitSet(1449)],
                                                         &[BitSet(1454)],
                                                         &[BitSet(1170)],
                                                         &[BitSet(1173)],
                                                         &[BitSet(1187)],
                                                         &[BitSet(1188)],
                                                         &[BitSet(3396)],
                                                         &[BitSet(3395)],
                                                         &[BitSet(49), BitSet(3396)],
                                                         &[BitSet(3442)],
                                                         &[BitSet(3150)],
                                                         &[BitSet(3145)],
                                                         &[BitSet(49), BitSet(3150)],
                                                         &[BitSet(3192)],
                                                         &[BitSet(1372)],
                                                         &[BitSet(1371)],
                                                         &[BitSet(1389)],
                                                         &[BitSet(266), BitSet(1120)],
                                                         &[BitSet(1110)],
                                                         &[BitSet(1105)],
                                                         &[BitSet(1127)],
                                                         &[BitSet(1120)],
                                                         &[BitSet(2528)],
                                                         &[BitSet(7), BitSet(2528)],
                                                         &[BitSet(2513)],
                                                         &[BitSet(2518)],
                                                         &[BitSet(2282)],
                                                         &[BitSet(2285)],
                                                         &[BitSet(2267)],
                                                         &[BitSet(2072), BitSet(196)],
                                                         &[BitSet(504)],
                                                         &[BitSet(7), BitSet(504)],
                                                         &[BitSet(457)],
                                                         &[BitSet(462)],
                                                         &[BitSet(242)],
                                                         &[BitSet(245)],
                                                         &[BitSet(195)],
                                                         &[BitSet(196)],
                                                         &[BitSet(2340)],
                                                         &[BitSet(2339)],
                                                         &[BitSet(2325)],
                                                         &[BitSet(2322)],
                                                         &[BitSet(2094)],
                                                         &[BitSet(2089)],
                                                         &[BitSet(2079)],
                                                         &[BitSet(2072)],
                                                         &[BitSet(316)],
                                                         &[BitSet(315)],
                                                         &[BitSet(269)],
                                                         &[BitSet(266)],
                                                         &[BitSet(54)],
                                                         &[BitSet(49)],
                                                         &[BitSet(7)],
                                                         &[]];

// Following is the code, used to generate this table.


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
// #[derive(Clone, Copy)]
// pub enum Corner {
//     A = 0,
//     B = 1,
//     C = 2,
//     D = 3,
//     E = 4,
//     F = 5,
//     G = 6,
//     H = 7,
// }
// // Corner connections
// pub const CORNER_CONNS: [[Corner; 3]; 8] = [[Corner::B, Corner::C, Corner::E],
//                                             [Corner::A, Corner::D, Corner::F],
//                                             [Corner::A, Corner::D, Corner::G],
//                                             [Corner::B, Corner::C, Corner::H],
//                                             [Corner::A, Corner::F, Corner::G],
//                                             [Corner::B, Corner::E, Corner::H],
//                                             [Corner::C, Corner::E, Corner::H],
//                                             [Corner::D, Corner::F, Corner::G]];
//
// // Which corners does a edge connect:
// pub const EDGE_DEF: [(Corner, Corner); 12] = [(Corner::A, Corner::B),
//                                               (Corner::A, Corner::C),
//                                               (Corner::A, Corner::E),
//                                               (Corner::C, Corner::D),
//                                               (Corner::B, Corner::D),
//                                               (Corner::B, Corner::F),
//                                               (Corner::E, Corner::F),
//                                               (Corner::E, Corner::G),
//                                               (Corner::C, Corner::G),
//                                               (Corner::G, Corner::H),
//                                               (Corner::F, Corner::H),
//                                               (Corner::D, Corner::H)];
//
// use dual_marching_cubes::Edge;
//
// fn get_dmc_cell_configs() -> Vec<Vec<BitSet>> {
//     let mut configs = Vec::with_capacity(256);
//     for cell_corners in 0..256 {
//         let edge_sets = get_edges_for_cell_config(cell_corners as u8);
//         configs.push(edge_sets);
//     }
//     println!("{:?}", configs);
//     configs
// }
//
//
// // Return a list of a set of edges for a cell config. E.g. which edges are connected to
// // each other for that cell config.
// fn get_edges_for_cell_config(corners: u8) -> Vec<BitSet> {
//     let cell = BitSet::new(corners as u32);
//     // Handle special case
//     if let Some(special) = get_connected_edges_for_diagonal_case(cell) {
//         return special;
//     }
//     let mut result = Vec::new();
//     let mut visited_corners = BitSet::zero();
//     for corner in cell.clone().into_iter() {
//         let connected_corners = visit_all_corners(corner, cell, &mut visited_corners);
//         if !connected_corners.empty() {
//             result.push(connected_corners);
//         }
//     }
//     result
// }
//
// fn get_edge_from_corners(a: usize, b: usize) -> Edge {
//     for (edge, &(x, y)) in EDGE_DEF.iter().enumerate() {
//         if (a == x as usize && b == y as usize) || (a == y as usize && b == x as usize) {
//             return Edge::from_usize(edge);
//         }
//     }
//     panic!("could not find edge for {:?} - {:?}", a, b);
// }
//
//
// fn visit_all_corners(corner: usize, cell: BitSet, visited_corners: &mut BitSet) -> BitSet {
//     if visited_corners.get(corner) {
//         // We already visited the current corner
//         return BitSet::zero();
//     }
//     // Mark the current corner as visited.
//     visited_corners.set(corner);
//     let mut result = BitSet::zero();
//     for adjacent_corner_ref in CORNER_CONNS[corner].into_iter() {
//         let adjacent_corner = *adjacent_corner_ref as usize;
//         if cell.get(adjacent_corner) {
//             result = result.merge(visit_all_corners(adjacent_corner, cell, visited_corners));
//         } else {
//             let edge = get_edge_from_corners(corner, adjacent_corner);
//             result.set(edge as usize)
//         }
//     }
//     result
// }
//
// fn bitset_from_edges(edges: [Edge; 3]) -> BitSet {
//     let mut bs = BitSet::zero();
//     for edge in edges.iter() {
//         bs.set(*edge as usize);
//     }
//     bs
// }
//
// fn get_connected_edges_for_diagonal_case(cell: BitSet) -> Option<Vec<BitSet>> {
//     if cell.size() == 6 {
//         let inv = cell.neg();
//         let lowest = inv.lowest().unwrap();
//         if inv.get(7 - lowest) {
//             return Some(match lowest {
//                 0 => {
//                     vec![bitset_from_edges([Edge::A, Edge::B, Edge::C]),
//                          bitset_from_edges([Edge::J, Edge::K, Edge::L])]
//                 }
//                 1 => {
//                     vec![bitset_from_edges([Edge::A, Edge::E, Edge::F]),
//                          bitset_from_edges([Edge::H, Edge::I, Edge::J])]
//                 }
//                 2 => {
//                     vec![bitset_from_edges([Edge::B, Edge::D, Edge::I]),
//                          bitset_from_edges([Edge::F, Edge::G, Edge::K])]
//                 }
//                 3 => {
//                     vec![bitset_from_edges([Edge::D, Edge::E, Edge::L]),
//                          bitset_from_edges([Edge::C, Edge::G, Edge::H])]
//                 }
//                 x => panic!("diagonal case {:?} with lowest corner {:?}", cell, x),
//             });
//         }
//     }
//     None
// }
