
use stl_parser::STL;
use std::io;


fn main() -> io::Result<()> {
    let _stl = STL::parse("data/teapot.stl")?;
    println!("first");
    let _large = STL::parse("data/wiki.stl")?;
    println!("second");

    Ok(())
}