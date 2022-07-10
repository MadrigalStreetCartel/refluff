use binary_reader::{BinaryReader, Endian};
use thiserror::Error;

const HEADER_MAGIC_LE: u32 = 0x534a4325; // %CJS
const HEADER_MAGIC_BE: u32 = 0x25434a53;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid header")]
    InvalidHeader,
}

#[derive(Debug, Default)]
pub struct WorldHeader {
    magic: u32,
    version: u8,
    size: u16,
}

#[derive(Debug, Default)]
pub struct World {
    header: WorldHeader,
}

impl World {
    pub fn parse(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader = BinaryReader::from_u8(data);

        // I don't even know the endianess at the moment.
        // I would guess it's LittleEndian, but I'm not sure
        reader.set_endian(Endian::Little);

        // Parse header
        let header = {

            // 0x0000 - 0x0003 | Magic number?
            let magic = reader.read_u32()?;
            printhex!(0x0000: u32 = magic; "Magic Number");
            require!(magic => HEADER_MAGIC_LE; else Error::InvalidHeader);

            // 0x0004 | Always 0x01 -> Version?
            let version = reader.read_u8()?;
            printhex!(0x0004: u8 = version; "Version?");
            require!(version => 0x01; else Error::InvalidHeader);

            // 0x0005 - 0x0006 | These two seem to be different in most cases
            let _ = reader.read_u8()?; // ?
            let _ = reader.read_u8()?; // ?

            // Maybe 0x0005 - 0x0006 should be read as a u16?
            // Seems kinda random, no idea what this is.. Huge range of numbers.
            reader.jmp(0x0005);
            let b = reader.read_u16()?;
            printhex!(0x0005: u16 = b);

            // 0x0007 | Seems to always be between 0x01 and 0x09
            let _ = reader.read_u8()?;

            // 0x0008 | Always 0x00
            let b = reader.read_u8()?;
            require!(b => 0x00; else Error::InvalidHeader);

            // Maybe 0x0007 - 0x0008 should be read as a u16?
            // BE: Always a multiple of 256
            // LE: Always between 0x1 and 0x9
            reader.jmp(0x0007);
            let b = reader.read_u16()?;
            printhex!(0x0007: u16 = b); // 0x0100 - 0x0900 (256 - 2304) in BE

            // 0x0009 - 0x000a
            // Mostly different, let's read it as u16 for now
            let size = reader.read_u16()?; // Size?
            printhex!(0x0009: u16 = size; "Size?");

            // 0x000b | Always 0x00 or 0x01
            let b = reader.read_u8()?;
            printhex!(0x000b: u8 = b);

            // 0x000c | Always 0x00
            let b = reader.read_u8()?;
            require!(b => 0x00; else Error::InvalidHeader);

            WorldHeader { magic, version, size }
        };

        Ok(World { header })
    }
}
