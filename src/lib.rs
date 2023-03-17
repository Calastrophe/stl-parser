
// Reads the binary formatted STL format, planning to add ASCII, but its slower.

use std::io;
use std::io::{SeekFrom::*, Seek};
use std::fs;
use byteorder::{LittleEndian, ReadBytesExt};

// NOTE: May need to rename 'Vertices', bit confusing.
type Vertices = [(f32, f32, f32); 3];
type Vector3 = (f32, f32, f32);
const SIZE_OF_TRIANGLE: u32 = 50;


/// The STL struct which houses the vecotrs and vertices generated from parsing an STL file.
pub struct STL {
    vectors: Vec<Vector3>,
    vertices: Vec<Vertices>
}


/// A trait which is implemented atop Cursor to read vectors and vertices from a given cursor.
trait STLReader {
    fn read_vector(&mut self) -> io::Result<Vector3>;

    fn read_vertices(&mut self) -> io::Result<Vertices>;
}



impl STLReader for io::Cursor<Vec<u8>> {

    /// Reads a vector3 from the given cursor and returns the values in a Vector3 type.
    fn read_vector(&mut self) -> io::Result<Vector3> {
        let first = self.read_f32::<LittleEndian>()?;
        let second = self.read_f32::<LittleEndian>()?;
        let third = self.read_f32::<LittleEndian>()?;
        Ok((first, second, third))
    }

    /// Reads three vertices and returns them in a Vertices type.
    fn read_vertices(&mut self) -> io::Result<Vertices> {
        // Reuses read_vector since vertices are essentially just made up of three points.
        let v1 = self.read_vector()?;
        let v2 = self.read_vector()?;
        let v3 = self.read_vector()?;
        Ok([v1, v2, v3])
    }

}


impl STL {

    /// Parses a given filename and returns an STL file with vectors and vertices.
    pub fn parse(filename: &str) -> io::Result<STL> {
        let file = fs::read(filename)?;
        let mut cursor = io::Cursor::new(file);

        // Read the amount of triangles that we need to read in and allocate space.
        cursor.seek(Start(80))?;
        let size = cursor.read_u32::<LittleEndian>()?;

        // Allocate the vectors inside of the STL
        let mut stl = STL {vectors: Vec::with_capacity(size as _), vertices: Vec::with_capacity(size as _)};

        // Populate the vectors in the STL struct
        stl.populate_vecs(&mut cursor, 0, size)?;


        Ok(stl)
    }


    /// A function that is used to populate the underlying Vectors of the STL. Planned for future use in parallelism of reading files.
    fn populate_vecs(&mut self, cursor: &mut io::Cursor<Vec<u8>>, start: u32, end: u32) -> io::Result<()> {
        for _ in start..end {
            self.vectors.push(cursor.read_vector()?);
            self.vertices.push(cursor.read_vertices()?);
            // Skip over the attribute bytes...
            // NOTE: Could possibly be used in some files, may need to record them.
            cursor.read_u16::<LittleEndian>()?;
        }

        Ok(())
    }

}



#[cfg(test)]
mod tests {

    use std::mem::size_of;

    use super::*;

    #[test]
    fn cube_check() -> io::Result<()> {
        let stl = STL::parse("data/cube.stl")?;
        assert_eq!(12, stl.vectors.capacity());
        assert_eq!(12, stl.vectors.len());
        Ok(())
    }

    #[test]
    fn read_two_files() -> io::Result<()> {
        let _stl = STL::parse("data/teapot.stl")?;
        println!("first");
        let _large = STL::parse("data/NOADD.stl")?;
        println!("second");

        Ok(())
    }


}
