use crate::types::{Normal, Triangle, Vertex};
use crate::TriangleIterator;
use std::io::{BufRead, BufReader, Result};

/// Struct for ascii STL reader.
pub struct AsciiStlReader<'a> {
    name: Option<String>,
    lines: Box<dyn ::std::iter::Iterator<Item = Result<Vec<String>>> + 'a>,
}

impl ::std::iter::Iterator for AsciiStlReader<'_> {
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
        read: &'a mut dyn (::std::io::Read),
    ) -> Result<Box<dyn TriangleIterator<Item = Result<Triangle>> + 'a>> {
        let mut lines = BufReader::new(read).lines();
        let mut name: Option<String> = None;
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
            Some(Ok(ref line)) => {
                if line.trim().len() > 5 {
                    name = Some((line["solid".len()..].trim()).to_string());
                }
            }
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
            name,
            lines: Box::new(lines),
        })
            as Box<dyn TriangleIterator<Item = Result<Triangle>>>)
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
        let face_header = face_header.unwrap()?;
        if !face_header.is_empty() && face_header[0] == "endsolid" {
            return Ok(None);
        }
        if face_header.len() != 5 || face_header[0] != "facet" || face_header[1] != "normal" {
            return Err(::std::io::Error::new(
                ::std::io::ErrorKind::InvalidData,
                format!("invalid facet header: {:?}", face_header),
            ));
        }
        let mut result_normal = Normal::default();
        AsciiStlReader::tokens_to_f32(&face_header[2..5], &mut result_normal.0[0..3])?;
        self.expect_static(&["outer", "loop"])?;
        let mut result_vertices = [Vertex::default(); 3];
        for vertex_result in &mut result_vertices {
            if let Some(line) = self.lines.next() {
                let line = line?;
                if line.len() != 4 || line[0] != "vertex" {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!("vertex f32 f32 f32, got {:?}", line),
                    ));
                }
                AsciiStlReader::tokens_to_f32(&line[1..4], &mut vertex_result.0[0..3])?;
            } else {
                return Err(::std::io::Error::new(
                    ::std::io::ErrorKind::UnexpectedEof,
                    "EOF while expecting vertex",
                ));
            }
        }
        self.expect_static(&["endloop"])?;
        self.expect_static(&["endfacet"])?;
        Ok(Some(Triangle {
            normal: result_normal,
            vertices: result_vertices,
        }))
    }
    fn tokens_to_f32(tokens: &[String], output: &mut [f32]) -> Result<()> {
        assert_eq!(tokens.len(), output.len());
        for i in 0..tokens.len() {
            let f = tokens[i].parse::<f32>().map_err(|e| {
                ::std::io::Error::new(::std::io::ErrorKind::InvalidData, e.to_string())
            })?;
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
            let line = line?;
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

impl TriangleIterator for AsciiStlReader<'_> {
    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}
