use binary_reader::{BinaryReader, Endian};
use thiserror::Error;

const HEADER_MAGIC_LE: u32 = 0x534a4325; // %CJS
const HEADER_MAGIC_BE: u32 = 0x25434a53;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid header")]
    InvalidHeader,
}

/// World Header.
///
/// Things that _could_ be in here:
/// - Magic number (confirmed)
/// - Version (unsure)
/// - Size (unsure)
/// - ID (unsure, likely u16; example: 1018 for Flaris)
/// - Town (unsure, bool, most likely 0x00 or 0x01)
/// - Polygons / Coords (unsure, maybe pairs of u16 for x and z coords)
///
/// The FlyffU API describes continents using polygons, but since the world here
/// actually has to be renderered I'd expect them to include more info. Maybe (x, y, z, h)?
///
/// The old Flyff lnd format has a fixed chunkset of 129x129 and only stores height as i16 for each chunk.
/// Landscape flags such as [NoDie, NoMove, NoFly, NoWalk] are inferred from the height value.
#[derive(Debug, Default)]
pub struct WorldHeader {
    /// Magic number
    magic: u32,
    /// Version
    version: u8,
    __unknown0: u16,
    __unknown1: u16,
    size: u16,
    town: bool,
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
        printhex!(__print_table);

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

            // Maybe 0x0005 - 0x0006 should be read as a u16?
            // Seems kinda random, no idea what this is.. Huge range of numbers.
            let __unknown0 = reader.read_u16()?;
            printhex!(0x0005: u16 = __unknown0);

            // Maybe 0x0007 - 0x0008 should be read as a u16?
            // 0x0007 is always between 0x01 and 0x09
            // 0x0008 is always 0x00
            // BE: Always a multiple of 256 (0x0100 - 0x0900)
            // LE: Always between 0x1 and 0x9 (0x0001 - 0x0009)
            let __unknown1 = reader.read_u16()?;
            printhex!(0x0007: u16 = __unknown1);

            // 0x0009 - 0x000a
            // Mostly different, let's read it as u16 for now
            let size = reader.read_u16()?; // Size?
            printhex!(0x0009: u16 = size; "Size?");

            // 0x000b | Always 0x00 or 0x01
            let town = printhex!(0x000b: u8 = reader.read_u8()?; "Town?") == 1;

            // 0x000c | Always 0x00
            printhex!(0x000c: u8 = reader.read_u8()?);

            printhex!(0x000d: u8 = reader.read_u8()?);
            printhex!(0x000e: u16 = reader.read_u16()?);

            WorldHeader {
                magic,
                version,
                __unknown0,
                __unknown1,
                size,
                town,
            }
        };

        Ok(World { header })
    }
}
