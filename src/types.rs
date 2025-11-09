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

impl<'a, M: Copy + Default + float_cmp::FloatMargin, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Vector<F> {
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

        if let Option::Some((fi, i1, i2)) = unconnected_edges.values().into_iter().next() {
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

    /// Transforms STL Mesh into STL Triangles in order to save them.
    ///
    /// ```
    /// let mut reader = ::std::io::Cursor::new(
    ///     b"solid ASCII
    ///             facet normal 8.491608e-001 1.950388e-001 -4.908011e-001
    ///             outer loop
    ///             vertex   -8.222098e-001 2.326105e+001 5.724931e-046
    ///             vertex   -8.811435e-001 2.351764e+001 1.135191e-045
    ///             vertex   3.688022e+000 2.340444e+001 7.860367e+000
    ///             endloop
    ///         endfacet
    ///     endsolid"
    ///         .to_vec(),
    /// );
    ///
    /// let triangle_reference_vector =
    ///     stl_io::create_stl_reader(&mut reader.clone())
    ///         .unwrap()
    ///         .collect::<Result<Vec<_>,_>>()
    ///         .unwrap();
    ///
    /// let stl = stl_io::create_stl_reader(&mut reader)
    ///         .unwrap()
    ///         .as_indexed_triangles().unwrap();
    ///
    /// assert_eq!(stl.into_triangle_vec(), triangle_reference_vector);
    /// ```
    pub fn into_triangle_vec(self) -> Vec<Triangle> {
        self.faces
            .iter()
            .map(|a| Triangle {
                normal: a.normal,
                vertices: [
                    self.vertices[a.vertices[0]],
                    self.vertices[a.vertices[1]],
                    self.vertices[a.vertices[2]],
                ],
            })
            .collect()
    }
}
