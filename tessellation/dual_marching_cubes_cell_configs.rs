use BitSet;
use dual_marching_cubes::{CORNER_CONNS, EDGE_DEF, Edge};


// DualMarchingCubesCellConfigs stores the different cell configurations as suggested by Nielson.
// Each cell has 8 corners. Each corner can be either inside (true) or outside (false).
// This results in 256 different configurations. Due to symmetry those can be reduced to 23
// distinct configs.

// For each corner config, return all edges, that are connected to a single point. These egdes are
// stored in a BitSet. Since there might be more than 1 point, store  a vector of BitSets.

pub fn get_dmc_cell_configs() -> Vec<Vec<BitSet>> {
    let mut configs = Vec::with_capacity(256);
    for cell_corners in 0..256 {
        let edge_sets = get_edges_for_cell_config(cell_corners as u8);
        configs.push(edge_sets);
    }
    configs
}


// Return a list of a set of edges for a cell config. E.g. which edges are connected to
// each other for that cell config.
fn get_edges_for_cell_config(corners: u8) -> Vec<BitSet> {
    let cell = BitSet::new(corners as u32);
    // Handle special case
    if let Some(special) = get_connected_edges_for_diagonal_case(cell) {
        return special;
    }
    let mut result = Vec::new();
    let mut visited_corners = BitSet::new(0);
    for corner in cell.clone().into_iter() {
        let connected_corners = visit_all_corners(corner, cell, &mut visited_corners);
        if !connected_corners.empty() {
            result.push(connected_corners);
        }
    }
    result
}

fn get_edge_from_corners(a: usize, b: usize) -> Edge {
    for (edge, &(x, y)) in EDGE_DEF.iter().enumerate() {
        if (a == x as usize && b == y as usize) || (a == y as usize && b == x as usize) {
            return Edge::from_usize(edge);
        }
    }
    panic!("could not find edge for {:?} - {:?}", a, b);
}


fn visit_all_corners(corner: usize, cell: BitSet, visited_corners: &mut BitSet) -> BitSet {
    if visited_corners.get(corner) {
        // We already visited the current corner
        return BitSet::new(0);
    }
    // Mark the current corner as visited.
    visited_corners.set(corner);
    let mut result = BitSet::new(0);
    for adjacent_corner_ref in CORNER_CONNS[corner].into_iter() {
        let adjacent_corner = *adjacent_corner_ref as usize;
        if cell.get(adjacent_corner) {
            result = result.merge(visit_all_corners(adjacent_corner, cell, visited_corners));
        } else {
            let edge = get_edge_from_corners(corner, adjacent_corner);
            result.set(edge as usize)
        }
    }
    result
}

fn bitset_from_edges(edges: [Edge; 3]) -> BitSet {
    let mut bs = BitSet::new(0);
    for edge in edges.iter() {
        bs.set(*edge as usize);
    }
    bs
}

fn get_connected_edges_for_diagonal_case(cell: BitSet) -> Option<Vec<BitSet>> {
    if cell.size() == 6 {
        let inv = cell.neg();
        let lowest = inv.lowest().unwrap();
        if inv.get(7 - lowest) {
            return Some(match lowest {
                0 => {
                    vec![bitset_from_edges([Edge::A, Edge::B, Edge::C]),
                         bitset_from_edges([Edge::J, Edge::K, Edge::L])]
                }
                1 => {
                    vec![bitset_from_edges([Edge::A, Edge::E, Edge::F]),
                         bitset_from_edges([Edge::H, Edge::I, Edge::J])]
                }
                2 => {
                    vec![bitset_from_edges([Edge::B, Edge::D, Edge::I]),
                         bitset_from_edges([Edge::F, Edge::G, Edge::K])]
                }
                3 => {
                    vec![bitset_from_edges([Edge::D, Edge::E, Edge::L]),
                         bitset_from_edges([Edge::C, Edge::G, Edge::H])]
                }
                x => panic!("diagonal case {:?} with lowest corner {:?}", cell, x),
            });
        }
    }
    None
}
