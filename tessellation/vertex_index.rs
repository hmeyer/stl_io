use bitset::BitSet;


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

// Face order X0, X1, Y0, Y1, Z0, Z1
const EDGES_ON_FACE: [BitSet; 6] = [BitSet::from_4bits(1, 2, 7, 8),
                                    BitSet::from_4bits(4, 5, 10, 11),
                                    BitSet::from_4bits(0, 2, 5, 6),
                                    BitSet::from_4bits(3, 8, 9, 11),
                                    BitSet::from_4bits(0, 1, 3, 4),
                                    BitSet::from_4bits(6, 7, 9, 10)];

fn egdes_on_neighbor(neighbor_index: usize, edges: BitSet) -> BitSet {
    let bits = edges.intersect(EDGES_ON_FACE[neighbor_index]).as_u32();
    match neighbor_index {
        0 => BitSet::new(bits << 3),
        1 => BitSet::new(bits >> 3),
        2 => BitSet::new((bits & 0b1000001) << 3 | (bits & 0b100100) << 6),
        3 => BitSet::new((bits & 0b1000001000) >> 3 | (bits & 0b100100000000) >> 6),
        4 => BitSet::new(bits << 6),
        5 => BitSet::new(bits >> 6),
        x => panic!("Invalid neighbor index: {}", x),
    }
}

pub type Index = [usize; 3];

pub fn offset(idx: Index, offset: Index) -> Index {
    [idx[0] + offset[0], idx[1] + offset[1], idx[2] + offset[2]]
}

pub fn neg_offset(idx: Index, offset: Index) -> Index {
    [idx[0] - offset[0], idx[1] - offset[1], idx[2] - offset[2]]
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct VertexIndex {
    pub edges: BitSet,
    pub index: Index,
}

impl VertexIndex {
    pub fn neighbor(&self, mut face: usize) -> Option<VertexIndex> {
        let neighbor_edge_set = egdes_on_neighbor(face, self.edges);
        if neighbor_edge_set.empty() {
            return None;
        }
        let mut off = [0, 0, 0];
        off[face / 2] = 1;
        let neighbor_index;
        if (face & 1) == 1 {
            neighbor_index = offset(self.index, off);
        } else {
            neighbor_index = neg_offset(self.index, off);
        }
        Some(VertexIndex {
            edges: neighbor_edge_set,
            index: neighbor_index,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]

pub enum VarIndex {
    VertexIndex(VertexIndex),
    Index(usize),
}

#[cfg(test)]
mod tests {
    use super::{EDGES_ON_FACE, VertexIndex};
    use super::super::bitset::BitSet;

    #[test]
    fn neighbor() {
        let v = VertexIndex {
            edges: BitSet::new(0xfff),
            index: [17, 35, 8],
        };
        for i in (0..6).step_by(2) {
            let mut expected_index = v.index;
            expected_index[i / 2] -= 1;
            assert_eq!(v.neighbor(i).unwrap().index, expected_index);
            assert_eq!(v.neighbor(i).unwrap().edges, EDGES_ON_FACE[i + 1]);
        }
        for i in (1..6).step_by(2) {
            let mut expected_index = v.index;
            expected_index[i / 2] += 1;
            assert_eq!(v.neighbor(i).unwrap().index, expected_index);
            assert_eq!(v.neighbor(i).unwrap().edges, EDGES_ON_FACE[i - 1]);
        }
    }
}
