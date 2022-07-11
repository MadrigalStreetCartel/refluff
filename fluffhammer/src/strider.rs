use binary_reader::{BinaryReader, Endian};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to read from binary")]
    BinaryReadError,
}

#[derive(Debug)]
pub struct Offset {
    pub offset: usize,
    pub width_bits: u8,
    pub endian: Endian,
}

#[derive(Debug)]
pub struct Strider<'a> {
    data: &'a [u8],
    reader: BinaryReader,
    needle_u8: Option<u8>,
    needle_u16: Option<u16>,
    needle_u32: u32,
}

impl <'a> Strider<'a> {
    pub fn new(needle: u32, data: &'a [u8]) -> Self {
        let reader = BinaryReader::from_u8(data);
        Self {
            data,
            reader,
            needle_u8: u8::try_from(needle).ok(),
            needle_u16: u16::try_from(needle).ok(),
            needle_u32: needle,
        }
    }

    pub fn run(&mut self) -> Result<Vec<Offset>, Error> {
        let mut results: Vec<Offset> = Vec::new();

        macro_rules! read {
            ($offset:expr) => {{
                self.reader.jmp($offset);
                let len = self.reader.length;
                match $offset {
                    pos if pos + 4 < len => {
                        self.reader.jmp(pos);
                        let val32 = self.reader.read_u32().map_err(|_| Error::BinaryReadError)?;
                        self.reader.jmp(pos);
                        let val16 = self.reader.read_u16().map_err(|_| Error::BinaryReadError)?;
                        self.reader.jmp(pos);
                        let val8 = self.reader.read_u8().map_err(|_| Error::BinaryReadError)?;
                        (val8, val16, val32)
                    }
                    pos if pos + 2 < len => {
                        let val16 = self.reader.read_u16().map_err(|_| Error::BinaryReadError)?;
                        self.reader.jmp(pos);
                        let val8 = self.reader.read_u8().map_err(|_| Error::BinaryReadError)?;
                        (val8, val16, 0_u32)
                    }
                    pos => {
                        self.reader.jmp(pos);
                        let val8 = self.reader.read_u8().map_err(|_| Error::BinaryReadError)?;
                        (val8, 0_u16, 0_u32)
                    }
                }
            }};
            (__all, $endianness:expr) => {{
                self.reader.jmp(0);
                let endian: Endian = $endianness;
                self.reader.set_endian(endian);
                for offset in 0..self.data.len() {
                    let (val8, val16, val32) = read!(offset);
                    if let Some(needle8) = self.needle_u8 {
                        if val8 == needle8 {
                            results.push(Offset { offset, width_bits: 8, endian })
                        }
                    }
                    if let Some(needle16) = self.needle_u16 {
                        if val16 == needle16 {
                            results.push(Offset { offset, width_bits: 16, endian })
                        }
                    }
                    if val32 == self.needle_u32 {
                        results.push(Offset { offset, width_bits: 32, endian })
                    }
                }
            }};
        }

        read!(__all, Endian::Little);
        read!(__all, Endian::Big);

        results.sort_by(|a, b| a.offset.cmp(&b.offset));

        Ok(results)
    }
}
