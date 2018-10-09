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
//! Write STL file:
//!
//! ```rust,no_run
//! use std::fs::OpenOptions;
//! let mesh = [stl_io::Triangle { normal: [1.0, 0.0, 0.0],
//!                                vertices: [[0.0, -1.0, 0.0],
//!                                           [0.0, 1.0, 0.0],
//!                                           [0.0, 0.0, 0.5]]}];
//! let mut file = OpenOptions::new().write(true).create_new(true).open("mesh.stl").unwrap();
//! stl_io::write_stl(&mut file, mesh.iter()).unwrap();
//! ```

#![warn(missing_docs)]

extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{BufRead, BufReader, BufWriter};
use std::io::{Read, Result, Write};
use std::iter::Iterator;

/// STL vertex - a corner of a Triangle in a 3D Mesh.
pub type Vertex = [f32; 3];
/// STL Normal - a vector perpendicular to a Triangle in a 3D Mesh.
pub type Normal = [f32; 3];

/// STL Triangle, consisting of a normal and three vertices.
/// This is the format Triangles are usually stored in STL files.
#[derive(Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
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
        for (fi, face) in self.faces.iter().enumerate() {
            for i in 0..3 {
                // Verify that all vertices are different.
                if self.vertices[face.vertices[i]] == self.vertices[face.vertices[(i + 1) % 3]] {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "face #{} has identical vertices #v{} and #v{}",
                            fi,
                            i,
                            (i + 1) % 3
                        ),
                    ));
                }
                let mut found_edge = false;
                for (fi2, face2) in self.faces.iter().enumerate() {
                    if fi == fi2 {
                        // Don't look for matching edge in this face.
                        continue;
                    }
                    for i2 in 0..3 {
                        if (self.vertices[face.vertices[i]]
                            == self.vertices[face2.vertices[(i2 + 1) % 3]])
                            && (self.vertices[face.vertices[(i + 1) % 3]]
                                == self.vertices[face2.vertices[i2]])
                        {
                            found_edge = true;
                            break;
                        }
                    }
                    if found_edge {
                        break;
                    }
                }
                if !found_edge {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "did not find facing edge for face #{}, edge #v{} -> #v{}",
                            fi,
                            i,
                            (i + 1) % 3
                        ),
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Write to std::io::Write as documented in
/// [Wikipedia](https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL).
///
/// ```
/// let mesh = [stl_io::Triangle { normal: [1.0, 0.0, 0.0],
///                                vertices: [[0.0, -1.0, 0.0],
///                                           [0.0, 1.0, 0.0],
///                                           [0.0, 0.0, 0.5]]}];
/// let mut binary_stl = Vec::<u8>::new();
/// stl_io::write_stl(&mut binary_stl, mesh.iter()).unwrap();
/// ```
pub fn write_stl<'a, W, I>(writer: &mut W, mesh: I) -> Result<()>
where
    W: ::std::io::Write,
    I: ::std::iter::ExactSizeIterator<Item = &'a Triangle>,
{
    let mut writer = BufWriter::new(writer);

    // Write 80 byte header
    writer.write_all(&[0u8; 80])?;
    writer.write_u32::<LittleEndian>(mesh.len() as u32)?;
    for t in mesh {
        for f in &t.normal {
            writer.write_f32::<LittleEndian>(*f as f32)?;
        }
        for &p in &t.vertices {
            for c in &p {
                writer.write_f32::<LittleEndian>(*c as f32)?;
            }
        }
        // Attribute byte count
        writer.write_u16::<LittleEndian>(0)?;
    }
    writer.flush()
}

/// Attempts to read either ascci or binary STL from std::io::Read.
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
    create_stl_reader(read)?.to_indexed_triangles()
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
) -> Result<Box<TriangleIterator<Item = Result<Triangle>> + 'a>>
where
    R: ::std::io::Read + ::std::io::Seek,
{
    match AsciiStlReader::probe(read) {
        Ok(()) => AsciiStlReader::create_triangle_iterator(read),
        Err(_) => BinaryStlReader::create_triangle_iterator(read),
    }
}

/// Struct for binary STL reader.
pub struct BinaryStlReader<'a> {
    reader: Box<::std::io::Read + 'a>,
    index: usize,
    size: usize,
}

impl<'a> BinaryStlReader<'a> {
    /// Factory to create a new BinaryStlReader from read.
    pub fn create_triangle_iterator(
        read: &'a mut ::std::io::Read,
    ) -> Result<Box<TriangleIterator<Item = Result<Triangle>> + 'a>> {
        let mut reader = Box::new(BufReader::new(read));
        reader.read_exact(&mut [0u8; 80])?;
        let num_faces = reader.read_u32::<LittleEndian>()? as usize;
        Ok(Box::new(BinaryStlReader {
            reader,
            index: 0,
            size: num_faces,
        }) as Box<TriangleIterator<Item = Result<Triangle>>>)
    }

    fn next_face(&mut self) -> Result<Triangle> {
        let mut normal = [0.; 3];
        for f in &mut normal {
            *f = self.reader.read_f32::<LittleEndian>()?;
        }
        let mut face = [[0.; 3]; 3];
        for vertex in &mut face {
            for c in vertex.iter_mut() {
                *c = self.reader.read_f32::<LittleEndian>()?;
            }
        }
        self.reader.read_u16::<LittleEndian>()?;
        Ok(Triangle {
            normal,
            vertices: face,
        })
    }
}

impl<'a> ::std::iter::Iterator for BinaryStlReader<'a> {
    type Item = Result<Triangle>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            self.index += 1;
            return Some(self.next_face());
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size - self.index, Some(self.size - self.index))
    }
}

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
    /// let indexed_mesh = stl.to_indexed_triangles().unwrap();
    /// ```
    fn to_indexed_triangles(&mut self) -> Result<IndexedMesh> {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut vertex_to_index = ::std::collections::HashMap::new();
        // Do not reserve memory in those structures based on size_hint, because we might have just
        // read bogus data.
        let mut vertex_indices = [0; 3];
        for t in self {
            let t = t?;
            for (i, vertex) in t.vertices.iter().enumerate() {
                // This is ugly, but f32 has no Eq and no Hash.
                let bitpattern = unsafe { std::mem::transmute::<[f32; 3], [u32; 3]>(*vertex) };
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

/// Struct for ascii STL reader.
pub struct AsciiStlReader<'a> {
    lines: Box<::std::iter::Iterator<Item = Result<Vec<String>>> + 'a>,
}

impl<'a> TriangleIterator for BinaryStlReader<'a> {}
impl<'a> TriangleIterator for AsciiStlReader<'a> {}

impl<'a> ::std::iter::Iterator for AsciiStlReader<'a> {
    type Item = Result<Triangle>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_face() {
            Ok(None) => None,
            Ok(Some(t)) => Some(Ok(t)),
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a> AsciiStlReader<'a> {
    /// Test whether or not read is an ascii STL file.
    pub fn probe<F: ::std::io::Read + ::std::io::Seek>(read: &mut F) -> Result<()> {
        let mut header = String::new();
        let maybe_read_error = BufReader::new(&mut *read).read_line(&mut header);
        // Try to seek back to start before evaluating potential read errors.
        read.seek(::std::io::SeekFrom::Start(0))?;
        maybe_read_error?;
        if !header.starts_with("solid ") {
            Err(::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                "ascii STL does not start with \"solid \"",
            ))
        } else {
            Ok(())
        }
    }
    /// Factory to create a new ascii STL Reader from read.
    pub fn create_triangle_iterator(
        read: &'a mut ::std::io::Read,
    ) -> Result<Box<TriangleIterator<Item = Result<Triangle>> + 'a>> {
        let mut lines = BufReader::new(read).lines();
        match lines.next() {
            Some(Err(e)) => return Err(e),
            Some(Ok(ref line)) if !line.starts_with("solid ") => {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    "ascii STL does not start with \"solid \"",
                ))
            }
            None => {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::UnexpectedEof,
                    "empty file?",
                ))
            }
            _ => {}
        }
        let lines = lines
            .map(|result| {
                     result.map(|l| {
                                    // Make lines into iterator over vectors of tokens
                                    l.split_whitespace()
                                        .map(|t| t.to_string())
                                        .collect::<Vec<_>>()
                                })
                 })
             // filter empty lines.
            .filter(|result| result.is_err() || (!result.as_ref().unwrap().is_empty()));
        Ok(Box::new(AsciiStlReader {
            lines: Box::new(lines),
        }) as Box<TriangleIterator<Item = Result<Triangle>>>)
    }
    // Tries to read a triangle.
    fn next_face(&mut self) -> Result<Option<Triangle>> {
        let face_header: Option<Result<Vec<String>>> = self.lines.next();
        if face_header.is_none() {
            return Err(::std::io::Error::new(
                ::std::io::ErrorKind::UnexpectedEof,
                "EOF while expecting facet or endsolid.",
            ));
        }
        let face_header = try!(face_header.unwrap());
        if !face_header.is_empty() && face_header[0] == "endsolid" {
            return Ok(None);
        }
        if face_header.len() != 5 || face_header[0] != "facet" || face_header[1] != "normal" {
            return Err(::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                format!("invalid facet header: {:?}", face_header),
            ));
        }
        let mut result_normal = [0.; 3];
        try!(AsciiStlReader::tokens_to_f32(
            &face_header[2..5],
            &mut result_normal[0..3]
        ));
        try!(self.expect_static(&["outer", "loop"]));
        let mut result_vertices = [[0.; 3]; 3];
        for vertex_result in &mut result_vertices {
            if let Some(line) = self.lines.next() {
                let line = try!(line);
                if line.len() != 4 || line[0] != "vertex" {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("vertex f32 f32 f32, got {:?}", line),
                    ));
                }
                try!(AsciiStlReader::tokens_to_f32(
                    &line[1..4],
                    &mut vertex_result[0..3]
                ));
            } else {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::UnexpectedEof,
                    "EOF while expecting vertex",
                ));
            }
        }
        try!(self.expect_static(&["endloop"]));
        try!(self.expect_static(&["endfacet"]));
        Ok(Some(Triangle {
            normal: result_normal,
            vertices: result_vertices,
        }))
    }
    fn tokens_to_f32(tokens: &[String], output: &mut [f32]) -> Result<()> {
        assert_eq!(tokens.len(), output.len());
        for i in 0..tokens.len() {
            let f = try!(tokens[i].parse::<f32>().map_err(|e| ::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                e.to_string()
            )));
            if !f.is_finite() {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    format!("expected finite f32, got {} which is {:?}", f, f.classify()),
                ));
            }
            output[i] = f;
        }
        Ok(())
    }
    fn expect_static(&mut self, expectation: &[&str]) -> Result<()> {
        if let Some(line) = self.lines.next() {
            let line = try!(line);
            if line != expectation {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::InvalidData,
                    format!("expected {:?}, got {:?}", expectation, line),
                ));
            }
        } else {
            return Err(::std::io::Error::new(
                ::std::io::ErrorKind::UnexpectedEof,
                format!("EOF while expecting {:?}", expectation),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
            AsciiStlReader::create_triangle_iterator(&mut reader)
                .unwrap()
                .to_indexed_triangles()
                .unwrap(),
            super::IndexedMesh {
                vertices: vec![[1., 2., 3.], [4., 5., 6e-15], [7., 8., 9.876_543]],
                faces: vec![IndexedTriangle {
                    normal: [0.1, 0.2, 0.3],
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
            AsciiStlReader::create_triangle_iterator(&mut reader)
                .unwrap()
                .to_indexed_triangles()
                .unwrap(),
            super::IndexedMesh {
                vertices: vec![[1., 2., 3.], [4., 5., 6e-15], [7., 8., 9.876_543]],
                faces: vec![IndexedTriangle {
                    normal: [0.1, 0.2, 0.3],
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles()
            .unwrap();
        assert_eq!(
            sort_vertices(stl),
            super::IndexedMesh {
                vertices: vec![[4., 5., 6.], [7., 8., 9.]],
                faces: vec![IndexedTriangle {
                    normal: [27., 28., 29.],
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader);
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles();
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles();
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles();
        assert!(stl.is_ok(), "{:?}", stl);
        assert_eq!(stl.unwrap().faces.len(), 99);
    }

    #[test]
    fn read_ascii_stl_bunny_and_write_binary_stl() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let bunny_mesh = AsciiStlReader::create_triangle_iterator(&mut reader);
        let bunny_mesh = bunny_mesh.unwrap().map(|t| t.unwrap()).collect::<Vec<_>>();
        let mut binary_bunny_stl = Vec::<u8>::new();
        let write_result = super::write_stl(&mut binary_bunny_stl, bunny_mesh.iter());
        assert!(write_result.is_ok(), "{:?}", write_result);
        assert_eq!(BUNNY_99.to_vec(), binary_bunny_stl);
    }

    #[test]
    fn read_binary_stl_bunny() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99);
        let stl = BinaryStlReader::create_triangle_iterator(&mut reader);
        assert_eq!(stl.unwrap().to_indexed_triangles().unwrap().faces.len(), 99);
    }

    #[test]
    fn read_ascii_and_binary_stl_bunny() {
        let mut binary_reader = ::std::io::Cursor::new(BUNNY_99);
        let binary_mesh = create_stl_reader(&mut binary_reader)
            .unwrap()
            .to_indexed_triangles()
            .unwrap();
        let mut ascii_reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let ascii_mesh = create_stl_reader(&mut ascii_reader)
            .unwrap()
            .to_indexed_triangles()
            .unwrap();
        let ascii_mesh = sort_vertices(ascii_mesh);
        let binary_mesh = sort_vertices(binary_mesh);
        assert_eq!(ascii_mesh, binary_mesh);
    }

    #[test]
    fn validate_bunny() {
        let mut reader = ::std::io::Cursor::new(BUNNY_99_ASCII);
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles()
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
        let stl = AsciiStlReader::create_triangle_iterator(&mut reader)
            .unwrap()
            .to_indexed_triangles();
        assert!(stl.is_ok(), "{:?}", stl);
    }
}
