use crate::types::Vertex;

/// Calculate the area of a triangle
pub fn tri_area(a: Vertex, b: Vertex, c: Vertex) -> f32 {
    fn cross(a: Vertex, b: Vertex) -> Vertex {
        let x = a[1] * b[2] - a[2] * b[1];
        let y = a[2] * b[0] - a[0] * b[2];
        let z = a[0] * b[1] - a[1] * b[0];
        Vertex::new([x, y, z])
    }
    fn sub(a: Vertex, b: Vertex) -> Vertex {
        let x = a[0] - b[0];
        let y = a[1] - b[1];
        let z = a[2] - b[2];
        Vertex::new([x, y, z])
    }
    fn length(v: Vertex) -> f32 {
        (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
    }

    length(cross(sub(c, b), sub(a, b))) * 0.5
}
