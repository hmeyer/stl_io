use float_cmp::ApproxEq;
use std::collections::HashMap;
use std::io::Result;

/// Float Vector with approx_eq.
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector<F>(pub [F; 3]);

impl<F> Vector<F> {
    /// Constructor from array.
    pub const fn new(v: [F; 3]) -> Self {
        Self(v)
    }
}

impl<F> From<Vector<F>> for [F; 3] {
    fn from(v: Vector<F>) -> Self {
        v.0
    }
}

impl<F> std::ops::Index<usize> for Vector<F> {
    type Output = F;
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < 3);
        &self.0[i]
    }
}

impl<M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &Vector<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self[0].approx_eq(other[0], margin)
            && self[1].approx_eq(other[1], margin)
            && self[2].approx_eq(other[2], margin)
    }
}

/// STL vertex - a corner of a Triangle in a 3D Mesh.
pub type Vertex = Vector<f32>;
/// STL Normal - a vector perpendicular to a Triangle in a 3D Mesh.
pub type Normal = Vector<f32>;

/// STL Triangle, consisting of a normal and three vertices.
/// This is the format Triangles are usually stored in STL files.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    /// Normal vector of the Triangle.
    pub normal: Normal,
    /// The three vertices of the Triangle.
    pub vertices: [Vertex; 3],
}

/// STL Triangle in indexed form, consisting of a normal and three indices to vertices in the
/// vertex list.
/// This format is more compact, since in real world Meshes Triangles usually share vertices with
/// other Triangles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IndexedTriangle {
    /// Normal vector of the Triangle.
    pub normal: Normal,
    /// The indexed to the three vertices of the Triangle, when this is used in an
    /// [IndexedMesh](struct.IndexedMesh.html).
    pub vertices: [usize; 3],
}

/// STL Mesh in indexed form, consisting of a list of [Vertices](type.Vertex.html) and a list of
/// [indexed Triangles](struct.IndexedTriangle.html).
#[derive(Clone, Debug, PartialEq)]
pub struct IndexedMesh {
    /// Optional name of the mesh (ASCII only).
    pub name: Option<String>,
    /// List of vertices.
    pub vertices: Vec<Vertex>,
    /// List of triangles..
    pub faces: Vec<IndexedTriangle>,
}

impl IndexedMesh {
    /// Checks that the Mesh has no holes and no zero-area faces.
    /// Also makes sure that all triangles are faced in the same direction.
    pub fn validate(&self) -> Result<()> {
        let mut unconnected_edges: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();

        for (fi, face) in self.faces.iter().enumerate() {
            {
                let a = self.vertices[face.vertices[0]];
                let b = self.vertices[face.vertices[1]];
                let c = self.vertices[face.vertices[2]];

                let area = super::utils::tri_area(a, b, c);

                if area < f32::EPSILON {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("face #{} has a zero-area face", fi),
                    ));
                }
            }

            for i in 0..3 {
                let u = face.vertices[i];
                let v = face.vertices[(i + 1) % 3];

                if unconnected_edges.contains_key(&(v, u)) {
                    unconnected_edges.remove(&(v, u));
                } else {
                    unconnected_edges.insert((u, v), (fi, i, (i + 1) % 3));
                }
            }
        }

        if let Option::Some((fi, i1, i2)) = unconnected_edges.values().next() {
            Err(::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                format!(
                    "did not find facing edge for face #{}, edge #v{} -> #v{}",
                    fi, i1, i2
                ),
            ))
        } else {
            Ok(())
        }
    }
}
