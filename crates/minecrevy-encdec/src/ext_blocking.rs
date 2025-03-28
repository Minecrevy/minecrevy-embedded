use embedded_byteorder::{BigEndian, Read, ReadBytesExt, Write, WriteBytesExt};
use heapless::{String, Vec};
use uuid::Uuid;

use crate::{RawPacket, ReadMinecraftError, var_i32_size};

/// Extends [`Read`] with methods for reading Minecraft-specific data types.
pub trait ReadMinecraftExt: Read {
    fn read_var_i32(&mut self) -> Result<i32, ReadMinecraftError<Self::Error>> {
        const CONTINUE_BIT: u8 = 0x80;
        const SEGMENT_MASK: u8 = 0x7F;

        let mut value = 0;
        let mut position = 0;
        let mut byte;

        loop {
            byte = self.read_u8()?;
            value |= ((byte & SEGMENT_MASK) as i32) << position;

            if (byte & CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                return Err(ReadMinecraftError::VarIntTooBig);
            }
        }
        Ok(value)
    }

    fn read_string<const MAX: usize>(
        &mut self,
    ) -> Result<String<MAX>, ReadMinecraftError<Self::Error>> {
        let len_i32 = self.read_var_i32()?;
        let len_usize = usize::try_from(len_i32).map_err(|_| ReadMinecraftError::LengthExceeded)?;

        let mut buf = Vec::new();
        buf.resize(len_usize, 0)
            .map_err(|_| ReadMinecraftError::LengthExceeded)?;
        self.read_exact(&mut buf)?;

        String::from_utf8(buf).map_err(|_| ReadMinecraftError::InvalidUtf8)
    }

    fn read_uuid(&mut self) -> Result<Uuid, ReadMinecraftError<Self::Error>> {
        let msb = self.read_u64::<BigEndian>()?;
        let lsb = self.read_u64::<BigEndian>()?;
        Ok(Uuid::from_u64_pair(msb, lsb))
    }

    fn read_raw_packet<const MAX: usize>(
        &mut self,
    ) -> Result<RawPacket<MAX>, ReadMinecraftError<Self::Error>> {
        let len_i32 = self.read_var_i32()?;
        let len_usize = usize::try_from(len_i32).map_err(|_| ReadMinecraftError::LengthExceeded)?;

        let id = self.read_var_i32()?;
        let mut data = Vec::new();
        data.resize(len_usize - var_i32_size(id), 0)
            .map_err(|_| ReadMinecraftError::LengthExceeded)?;
        self.read_exact(&mut data)?;

        Ok(RawPacket { id, data })
    }
}

impl<R: Read + ?Sized> ReadMinecraftExt for R {}

/// Extends [`AsyncWrite`] with methods for writing Minecraft-specific data types.
pub trait WriteMinecraftExt: Write {
    fn write_var_i32(&mut self, value: i32) -> Result<(), Self::Error> {
        const CONTINUE_BIT: u32 = 0x80;
        const SEGMENT_MASK: u32 = 0x7F;

        let value = value as u32;
        if (value & (0xFF_FF_FF_FF << 7)) == 0 {
            self.write_u8(value as u8)?;
        } else if (value & (0xFF_FF_FF_FF << 14)) == 0 {
            let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 8) | (value >> 7);
            self.write_u16::<BigEndian>(w as u16)?;
        } else if (value & (0xFF_FF_FF_FF << 21)) == 0 {
            let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 16)
                | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 8)
                | (value >> 14);
            // write u24
            self.write_u24::<BigEndian>(w)?;
        } else if (value & (0xFF_FF_FF_FF << 28)) == 0 {
            let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 24)
                | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 16)
                | (((value >> 14) & SEGMENT_MASK | CONTINUE_BIT) << 8)
                | (value >> 21);
            self.write_u32::<BigEndian>(w)?;
        } else {
            let w = ((value & SEGMENT_MASK | CONTINUE_BIT) << 24)
                | (((value >> 7) & SEGMENT_MASK | CONTINUE_BIT) << 16)
                | (((value >> 14) & SEGMENT_MASK | CONTINUE_BIT) << 8)
                | ((value >> 21) & SEGMENT_MASK | CONTINUE_BIT);
            self.write_u32::<BigEndian>(w)?;
            self.write_u8((value >> 28) as u8)?;
        }
        Ok(())
    }

    fn write_string<const MAX: usize>(&mut self, value: &str) -> Result<(), Self::Error> {
        let len_i32 = i32::try_from(value.len()).unwrap();
        self.write_var_i32(len_i32)?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }
}

impl<W: Write + ?Sized> WriteMinecraftExt for W {}
