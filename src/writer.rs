use crate::types::Triangle;
use std::io::{BufWriter, Result, Write};

/// Write to std::io::Write as documented in
/// [Wikipedia](https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL).
///
/// ```
/// use stl_io::{Vertex, Normal};
/// let mesh = [stl_io::Triangle { normal: Normal::new([1.0, 0.0, 0.0]),
///                                vertices: [Vertex::new([0.0, -1.0, 0.0]),
///                                           Vertex::new([0.0, 1.0, 0.0]),
///                                           Vertex::new([0.0, 0.0, 0.5])]}];
/// let mut binary_stl = Vec::<u8>::new();
/// stl_io::write_stl(&mut binary_stl, mesh.iter()).unwrap();
/// ```
pub fn write_stl<T, W, I>(writer: &mut W, mesh: I) -> Result<()>
where
    W: Write,
    I: ExactSizeIterator<Item = T>,
    T: std::borrow::Borrow<Triangle>,
{
    let mut writer = BufWriter::new(writer);

    // Write 80 byte header
    writer.write_all(&[0u8; 80])?;
    writer.write_all(&(mesh.len() as u32).to_le_bytes())?;
    for t in mesh {
        let t = t.borrow();
        for f in &t.normal.0 {
            writer.write_all(&f.to_le_bytes())?;
        }
        for p in &t.vertices {
            for c in &p.0 {
                writer.write_all(&c.to_le_bytes())?;
            }
        }
        // Attribute byte count
        writer.write_all(&0u16.to_le_bytes())?;
    }
    writer.flush()
}
