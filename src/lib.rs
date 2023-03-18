
// Reads the binary formatted STL format, planning to add ASCII, but its slower.

use std::io;
use std::io::{BufReader, Seek, SeekFrom};
use std::fs::{File};
use byteorder::{LittleEndian, ReadBytesExt};

type Vertices = [(f32, f32, f32); 3];
type Vector3 = (f32, f32, f32);
const TRIANGLE_SIZE: usize = 50;


/// The STL struct which houses the vecotrs and vertices generated from parsing an STL file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct STL {
    pub vectors: Vec<Vector3>,
    pub vertices: Vec<Vertices>
}


/// A trait which is extended onto all std::io::Read to be able to extract vectors and groups of vertices.
pub trait STLReaderExt {
    fn read_vector(&mut self) -> io::Result<Vector3>;

    fn read_vertices(&mut self) -> io::Result<Vertices>;
}



impl<T> STLReaderExt for T where
    T: std::io::Read {

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
        let mut file = File::open(filename)?;
        // Seek over the header of the STL file
        file.seek(SeekFrom::Current(80))?;
        // Grab the amount of triangles expected
        let size = file.read_u32::<LittleEndian>()? as usize;
        // Buffer our reader with the amount of triangles we expect to read
        let mut reader = BufReader::with_capacity(size * TRIANGLE_SIZE, file);

        // Allocate the vectors inside of the STL
        let mut stl = STL {vectors: Vec::with_capacity(size as _), vertices: Vec::with_capacity(size as _)};

        // Populate the STL file with our vectors and vertices, skipping over attribute bytes.
        for _ in 0..size {
            stl.vectors.push(reader.read_vector()?);
            stl.vertices.push(reader.read_vertices()?);
            // Skip over the attribute bytes...
            // NOTE: Could possibly be used in some files, may need to record them.
            reader.read_u16::<LittleEndian>()?;
        }


        Ok(stl)
    }

}



#[cfg(test)]
mod tests {
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

    #[test]
    fn speed_test() -> io::Result<()> {
        for i in 0..100 {
            let _ = STL::parse("data/teapot.stl")?;
            println!("{i} iteration");
        }

        Ok(())
    }

}
