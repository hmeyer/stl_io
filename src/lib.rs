//! ```stl_io``` is a crate for reading and writing [STL (STereoLithography)](https://en.wikipedia.org/wiki/STL_(file_format)) files.
//! It can read both, binary and ascii STL in a safe manner.
//! Writing is limited to binary STL, which is more compact anyway.
//! # Examples
//!
//! Read STL file:
//!
//! ```rust,no_run
//! use std::fs::OpenOptions;
//! let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
//! let stl = stl_io::read_stl(&mut file).unwrap();
//! ```
//!
//! Read number of triangles in a STL file:
//!
//! ```rust,no_run
//! use std::fs::OpenOptions;
//! let mut file = OpenOptions::new().read(true).open("mesh.stl").unwrap();
//! let size_hint = stl_io::create_stl_reader(&mut file).unwrap().size_hint();
//! ```
//!
//! Write STL file:
//!
//! ```rust,no_run
//! use std::fs::OpenOptions;
//! use stl_io::{Normal, Vertex};
//! let mesh = [stl_io::Triangle { normal: Normal::new([1.0, 0.0, 0.0]),
//!                                vertices: [Vertex::new([0.0, -1.0, 0.0]),
//!                                           Vertex::new([0.0, 1.0, 0.0]),
//!                                           Vertex::new([0.0, 0.0, 0.5])]}];
//! let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
//! stl_io::write_stl(&mut file, mesh.iter()).unwrap();
//! ```

#![warn(missing_docs)]

mod ascii_reader;
mod binary_reader;
mod types;
mod utils;
mod writer;

use std::io::{Result};
use std::iter::Iterator;

pub use types::{IndexedMesh, IndexedTriangle, Normal, Triangle, Vector, Vertex};
pub use writer::write_stl;

/// Iterates over all Triangles in a STL.
pub trait TriangleIterator: ::std::iter::Iterator<Item = Result<Triangle>> {
    /// Consumes this iterator and generates an [indexed Mesh](struct.IndexedMesh.html).
    ///
    /// ```
    /// let mut reader = ::std::io::Cursor::new(b"solid foobar
    /// facet normal 1 2 3
    ///     outer loop
    ///         vertex 7 8 9
    ///         vertex 4 5 6
    ///         vertex 7 8 9
    ///     endloop
    /// endfacet
    /// endsolid foobar".to_vec());
    /// let mut stl = stl_io::create_stl_reader(&mut reader).unwrap();
    /// let indexed_mesh = stl.as_indexed_triangles().unwrap();
    /// ```
    fn as_indexed_triangles(&mut self) -> Result<IndexedMesh> {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut vertex_to_index = ::std::collections::HashMap::new();
        // Do not reserve memory in those structures based on size_hint, because we might have just
        // read bogus data.
        let mut vertex_indices = [0; 3];
        for t in self {
            let t = t?;
            for (i, vertex) in t.vertices.iter().enumerate() {
                // f32 has no Eq and no Hash, but comparing the bits will do.
                // This has the effect that if any coordinate is NaN (which does not make sense
                // anyway), its NaN payload bits will be used as the identity of the vertex.
                let bitpattern = vertex.0.map(f32::to_bits);
                let index = *vertex_to_index
                    .entry(bitpattern)
                    .or_insert_with(|| vertices.len());
                if index == vertices.len() {
                    vertices.push(*vertex);
                }
                vertex_indices[i] = index;
            }
            triangles.push(IndexedTriangle {
                normal: t.normal,
                vertices: vertex_indices,
            });
        }
        vertices.shrink_to_fit();
        triangles.shrink_to_fit();
        Ok(IndexedMesh {
            vertices,
            faces: triangles,
        })
    }
}

/// Attempts to read either ascii or binary STL from std::io::Read.
///
/// ```
/// let mut reader = ::std::io::Cursor::new(
///     b"solid foobar
///       facet normal 0.1 0.2 0.3
///           outer loop
///               vertex 1 2 3
///               vertex 4 5 6e-15
///               vertex 7 8 9.87654321
///           endloop
///       endfacet
///       endsolid foobar".to_vec());
/// let mesh = stl_io::read_stl(&mut reader).unwrap();
/// ```
pub fn read_stl<R>(read: &mut R) -> Result<IndexedMesh>
where
    R: ::std::io::Read + ::std::io::Seek,
{
    create_stl_reader(read)?.as_indexed_triangles()
}

/// Attempts to create a [TriangleIterator](trait.TriangleIterator.html) for either ascii or binary
/// STL from std::io::Read.
///
/// ```
/// let mut reader = ::std::io::Cursor::new(b"solid foobar
/// facet normal 1 2 3
///     outer loop
///         vertex 7 8 9
///         vertex 4 5 6
///         vertex 7 8 9
///     endloop
/// endfacet
/// endsolid foobar".to_vec());
/// let stl = stl_io::create_stl_reader(&mut reader).unwrap();
/// ```
pub fn create_stl_reader<'a, R>(
    read: &'a mut R,
) -> Result<Box<dyn TriangleIterator<Item = Result<Triangle>> + 'a>>
where
    R: ::std::io::Read + ::std::io::Seek,
{
    match ascii_reader::AsciiStlReader::probe(read) {
        Ok(()) => ascii_reader::AsciiStlReader::create_triangle_iterator(read),
        Err(_) => binary_reader::BinaryStlReader::create_triangle_iterator(read),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use float_cmp::F32Margin;

    const BUNNY_99: &[u8] = include_bytes!("testdata/bunny_99.stl");
    const BUNNY_99_ASCII: &[u8] = include_bytes!("testdata/bunny_99_ascii.stl");

    // Will sort the vertices of the Mesh and fix the indices in the faces.
    fn sort_vertices(mut mesh: super::IndexedMesh) -> super::IndexedMesh {
        let mut index_map = (0..mesh.vertices.len()).collect::<Vec<_>>();
        index_map.sort_by(|a, b| mesh.vertices[*a].partial_cmp(&mesh.vertices[*b]).unwrap());
        let new_vertices = index_map
            .iter()
            .map(|i| mesh.vertices[*i])
            .collect::<Vec<_>>();
        mesh.vertices = new_vertices;
        for t in &mut mesh.faces {
            for i in &mut t.vertices {
                *i = index_map[*i];
            }
        }
        mesh
    }

    #[test]
    fn read_ascii_stl_simple_success() {
        let mut reader = ::std::io::Cursor::new(
            b"solid foobar
        facet normal 0.1 0.2 0.3
            outer loop
                vertex 1 2 3
                vertex 4 5 6e-15
                vertex 7 8 9.87654321
            endloop
        endfacet
        endsolid foobar"
                .to_vec(),
        );
        assert_eq!(
            ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
                .unwrap()
                .as_indexed_triangles()
                .unwrap(),
            super::IndexedMesh {
                vertices: vec![
                    Vertex::new([1., 2., 3.]),
                    Vertex::new([4., 5., 6e-15]),
                    Vertex::new([7., 8., 9.876_543])
                ],
                faces: vec![IndexedTriangle {
                    normal: Normal::new([0.1, 0.2, 0.3]),
                    vertices: [0, 1, 2],
                }],
            }
        );
    }

    #[test]
    fn read_ascii_stl_name_with_spaces_success() {
        let mut reader = ::std::io::Cursor::new(
            b"solid foo bar
        facet normal 0.1 0.2 0.3
            outer loop
                vertex 1 2 3
                vertex 4 5 6e-15
                vertex 7 8 9.87654321
            endloop
        endfacet
        endsolid foo bar"
                .to_vec(),
        );
        assert_eq!(
            ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
                .unwrap()
                .as_indexed_triangles()
                .unwrap(),
            super::IndexedMesh {
                vertices: vec![
                    Vertex::new([1., 2., 3.]),
                    Vertex::new([4., 5., 6e-15]),
                    Vertex::new([7., 8., 9.876_543])
                ],
                faces: vec![IndexedTriangle {
                    normal: Normal::new([0.1, 0.2, 0.3]),
                    vertices: [0, 1, 2],
                }],
            }
        );
    }

    #[test]
    fn read_ascii_stl_sort_and_depup_vertices() {
        let mut reader = ::std::io::Cursor::new(
            b"solid foobar
        facet normal 27 28 29
            outer loop
                vertex 7 8 9
                vertex 4 5 6
                vertex 7 8 9
            endloop
        endfacet
        endsolid foobar"
                .to_vec(),
        );
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles()
            .unwrap();
        assert_eq!(
            sort_vertices(stl),
            super::IndexedMesh {
                vertices: vec![Vertex::new([4., 5., 6.]), Vertex::new([7., 8., 9.])],
                faces: vec![IndexedTriangle {
                    normal: Normal::new([27., 28., 29.]),
                    vertices: [1, 0, 1],
                }],
            }
        );
    }

    #[test]
    fn read_ascii_stl_no_header() {
        let mut reader = ::std::io::Cursor::new(
            b"non-solid foobar
        facet normal 1 2 3
            outer loop
                vertex 7 8 9
                vertex 4 5 6
                vertex 7 8 9
            endloop
        endfacet
        endsolid foobar"
                .to_vec(),
        );
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader);
        assert_eq!(
            stl.as_ref().err().unwrap().kind(),
            ::std::io::ErrorKind::InvalidData,
            "{:?}",
            stl.err()
        );
    }

    #[test]
    fn read_ascii_stl_wrong_number() {
        let mut reader = ::std::io::Cursor::new(
            b"solid foobar
        facet normal 1 2 3
            outer loop
                vertex 7 8 9,
                vertex 4 5 6
                vertex 7 8 9
            endloop
        endfacet
        endsolid foobar"
                .to_vec(),
        );
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles();
        assert_eq!(
            stl.as_ref().err().unwrap().kind(),
            ::std::io::ErrorKind::InvalidData,
            "{:?}",
            stl.err()
        );
    }

    #[test]
    fn read_ascii_stl_missing_vertex() {
        let mut reader = ::std::io::Cursor::new(
            b"solid foobar
        facet normal 1 2 3
            outer loop
                vertex 7 8 9,
                vertex 4 5 6
            endloop
        endfacet
        endsolid foobar"
                .to_vec(),
        );
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles();
        assert_eq!(
            stl.as_ref().err().unwrap().kind(),
            ::std::io::ErrorKind::InvalidData,
            "{:?}",
            stl
        );
    }

    #[test]
    fn read_ascii_stl_bunny() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles();
        assert!(stl.is_ok(), "{:?}", stl);
        assert_eq!(stl.unwrap().faces.len(), 99);
    }

    #[test]
    fn read_ascii_stl_bunny_and_write_binary_stl() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let bunny_mesh = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader);
        let bunny_mesh = bunny_mesh.unwrap().map(|t| t.unwrap()).collect::<Vec<_>>();
        let mut binary_bunny_stl = Vec::<u8>::new();
        let write_result = super::write_stl(&mut binary_bunny_stl, bunny_mesh.iter());
        assert!(write_result.is_ok(), "{:?}", write_result);
        assert_eq!(BUNNY_99.to_vec(), binary_bunny_stl);
    }

    #[test]
    fn read_binary_stl_bunny() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99);
        let stl = binary_reader::BinaryStlReader::create_triangle_iterator(&mut reader);
        assert_eq!(stl.unwrap().as_indexed_triangles().unwrap().faces.len(), 99);
    }

    #[test]
    fn read_ascii_and_binary_stl_bunny() {
        let mut binary_reader = ::std::io::Cursor::new(BUNNY_99);
        let binary_mesh = create_stl_reader(&mut binary_reader)
            .unwrap()
            .as_indexed_triangles()
            .unwrap();
        let mut ascii_reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let ascii_mesh = create_stl_reader(&mut ascii_reader)
            .unwrap()
            .as_indexed_triangles()
            .unwrap();
        let ascii_mesh = sort_vertices(ascii_mesh);
        let binary_mesh = sort_vertices(binary_mesh);
        assert_eq!(ascii_mesh, binary_mesh);
    }

    #[test]
    fn validate_bunny() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles()
            .unwrap();
        assert_eq!(
            stl.validate().err().unwrap().kind(),
            ::std::io::ErrorKind::InvalidData,
            "{:?}",
            stl.validate().err()
        );
    }

    #[test]
    fn read_ascii_stl_tiny_numbers() {
        let mut reader = ::std::io::Cursor::new(
            b"solid ASCII
                  facet normal 8.491608e-001 1.950388e-001 -4.908011e-001
                    outer loop
                    vertex   -8.222098e-001 2.326105e+001 5.724931e-046
                    vertex   -8.811435e-001 2.351764e+001 1.135191e-045
                    vertex   3.688022e+000 2.340444e+001 7.860367e+000
                    endloop
                endfacet
            endsolid"
                .to_vec(),
        );
        let stl = ascii_reader::AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles();
        assert!(stl.is_ok(), "{:?}", stl);
    }

    #[test]
    fn simple_tri_area() {
        let a = Vector::new([-1.0, 1.0, 0.0]);
        let b = Vector::new([1.0, -1.0, 0.0]);
        let c = Vector::new([-1.0, -1.0, 0.0]);
        let area = utils::tri_area(a, b, c);
        assert_eq!(area, 2.0);
    }

    #[test]
    fn bunny_tri_area() {
        use float_cmp::ApproxEq;
        
        let mut reader = ::std::io::Cursor::new(BUNNY_99);
        let stl = binary_reader::BinaryStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .as_indexed_triangles()
            .unwrap();

        let mut total_area = 0.0;
        for face in stl.faces.iter() {
            let a = stl.vertices[face.vertices[0]];
            let b = stl.vertices[face.vertices[1]];
            let c = stl.vertices[face.vertices[2]];
            total_area = total_area + utils::tri_area(a, b, c);
        }

        // area of bunny model according to blender
        let blender_area: f32 = 0.04998364;

        assert!(total_area.approx_eq(blender_area, F32Margin::default()));
    }
}