use crate::TriangleIterator;
use crate::types::{Normal, Triangle, Vertex};
use std::io::{BufReader, Read, Result};

/// Struct for binary STL reader.
pub struct BinaryStlReader<'a> {
    reader: Box<dyn Read + 'a>,
    index: usize,
    size: usize,
}

impl<'a> BinaryStlReader<'a> {
    /// Factory to create a new BinaryStlReader from read.
    pub fn create_triangle_iterator(
        read: &'a mut dyn Read,
    ) -> Result<Box<dyn TriangleIterator<Item = Result<Triangle>> + 'a>> {
        let mut reader = Box::new(BufReader::new(read));
        let mut header = [0u8; 80];
        reader.read_exact(&mut header)?;
        let mut count_buf = [0u8; 4];
        reader.read_exact(&mut count_buf)?;
        let num_faces = u32::from_le_bytes(count_buf) as usize;
        Ok(Box::new(BinaryStlReader {
            reader,
            index: 0,
            size: num_faces,
        })
            as Box<dyn TriangleIterator<Item = Result<Triangle>>>)
    }

    fn next_face(&mut self) -> Result<Triangle> {
        // Each triangle: 12 floats (normal + 3 vertices) * 4 bytes + 2 bytes attribute = 50 bytes
        let mut buf = [0u8; 50];
        self.reader.read_exact(&mut buf)?;

        let mut floats = buf[..48]
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()));
        let mut next_vec = || [(); 3].map(|_| floats.next().unwrap());

        let normal = Normal::new(next_vec());
        let vertices = [
            Vertex::new(next_vec()),
            Vertex::new(next_vec()),
            Vertex::new(next_vec()),
        ];
        // buf[48..50] is the attribute byte count, ignored

        Ok(Triangle { normal, vertices })
    }
}

impl<'a> Iterator for BinaryStlReader<'a> {
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
