use std::io::{BufReader, Read};

#[macro_use]
mod macros;
mod world;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    if let Some(filename) = args.next() {
        let data = {
            let file = std::fs::File::open(filename)?;
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            buf
        };
        let world = world::World::parse(&data[..])?;
        println!("{:?}", world);
    }
    Ok(())
}
