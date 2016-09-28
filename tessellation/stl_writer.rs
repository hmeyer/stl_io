use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::{Result, Write};
use Mesh;
use byteorder::{LittleEndian, WriteBytesExt};
use xplicit_types::{Point, Vector};
use cgmath::InnerSpace;

// Write as documented in https://en.wikipedia.org/wiki/STL_(file_format)#Binary_STL
pub fn write_stl(filename: &str, mesh: &Mesh) -> Result<()> {
    let file = try!(OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(filename));
    let mut writer = BufWriter::new(file);

    // Write 80 byte header
    try!(writer.write(&[0u8; 80]));
    try!(writer.write_u32::<LittleEndian>(mesh.faces.len() as u32));
    for face in &mesh.faces {
        let n = get_normal(mesh, face);
        for i in 0..3 {
            try!(writer.write_f32::<LittleEndian>(n[i] as f32));
        }
        for &p in face {
            for c in &mesh.vertices[p] {
                try!(writer.write_f32::<LittleEndian>(*c as f32));
            }
        }
        // Attribute byte count
        try!(writer.write_u16::<LittleEndian>(0));
    }
    writer.flush()
}

fn get_normal(mesh: &Mesh, face: &[usize; 3]) -> Vector {
    let verts = &mesh.vertices;
    let v: Vec<Point> = face.iter()
                            .map(|&i| Point::new(verts[i][0], verts[i][1], verts[i][2]))
                            .collect();
    (v[1] - v[0]).cross(v[2] - v[0]).normalize()
}
