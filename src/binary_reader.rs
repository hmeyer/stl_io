use crate::types::{Normal, Triangle, Vertex};
use crate::TriangleIterator;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{BufReader, Result, Read};

/// Struct for binary STL reader.
pub struct BinaryStlReader<'a> {
    reader: Box<dyn ::std::io::Read + 'a>,
    index: usize,
    size: usize,
}

impl<'a> BinaryStlReader<'a> {
    /// Factory to create a new BinaryStlReader from read.
    pub fn create_triangle_iterator(
        read: &'a mut dyn (::std::io::Read),
    ) -> Result<Box<dyn TriangleIterator<Item = Result<Triangle>> + 'a>> {
        let mut reader = Box::new(BufReader::new(read));
        let mut header = [0u8; 80];
        reader.read_exact(&mut header)?;
        let num_faces = reader.read_u32::<LittleEndian>()? as usize;
        Ok(Box::new(BinaryStlReader {
            reader,
            index: 0,
            size: num_faces,
        })
            as Box<dyn TriangleIterator<Item = Result<Triangle>>>)
    }

    fn next_face(&mut self) -> Result<Triangle> {
        let mut normal = Normal::default();
        for f in &mut normal.0 {
            *f = self.reader.read_f32::<LittleEndian>()?;
        }
        let mut face = [Vertex::default(); 3];
        for vertex in &mut face {
            for c in vertex.0.iter_mut() {
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

impl<'a> TriangleIterator for BinaryStlReader<'a> {}