// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Byte order utilities for reading SWORD module binary files.
//!
//! SWORD stores all binary data in little-endian format. This module provides
//! functions to read and write values in the SWORD format regardless of the
//! host machine's endianness.

use std::io::{self, Read, Write};

/// Read a little-endian u16 from SWORD format.
#[inline]
pub fn read_u16_sword_chirho(data_chirho: &[u8]) -> u16 {
    debug_assert!(data_chirho.len() >= 2);
    u16::from_le_bytes([data_chirho[0], data_chirho[1]])
}

/// Read a little-endian i16 from SWORD format.
#[inline]
pub fn read_i16_sword_chirho(data_chirho: &[u8]) -> i16 {
    debug_assert!(data_chirho.len() >= 2);
    i16::from_le_bytes([data_chirho[0], data_chirho[1]])
}

/// Read a little-endian u32 from SWORD format.
#[inline]
pub fn read_u32_sword_chirho(data_chirho: &[u8]) -> u32 {
    debug_assert!(data_chirho.len() >= 4);
    u32::from_le_bytes([data_chirho[0], data_chirho[1], data_chirho[2], data_chirho[3]])
}

/// Read a little-endian i32 from SWORD format.
#[inline]
pub fn read_i32_sword_chirho(data_chirho: &[u8]) -> i32 {
    debug_assert!(data_chirho.len() >= 4);
    i32::from_le_bytes([data_chirho[0], data_chirho[1], data_chirho[2], data_chirho[3]])
}

/// Read a little-endian u64 from SWORD format.
#[inline]
pub fn read_u64_sword_chirho(data_chirho: &[u8]) -> u64 {
    debug_assert!(data_chirho.len() >= 8);
    u64::from_le_bytes([
        data_chirho[0], data_chirho[1], data_chirho[2], data_chirho[3],
        data_chirho[4], data_chirho[5], data_chirho[6], data_chirho[7],
    ])
}

/// Read a little-endian i64 from SWORD format.
#[inline]
pub fn read_i64_sword_chirho(data_chirho: &[u8]) -> i64 {
    debug_assert!(data_chirho.len() >= 8);
    i64::from_le_bytes([
        data_chirho[0], data_chirho[1], data_chirho[2], data_chirho[3],
        data_chirho[4], data_chirho[5], data_chirho[6], data_chirho[7],
    ])
}

/// Write a u16 in little-endian SWORD format.
#[inline]
pub fn write_u16_sword_chirho(value_chirho: u16) -> [u8; 2] {
    value_chirho.to_le_bytes()
}

/// Write an i16 in little-endian SWORD format.
#[inline]
pub fn write_i16_sword_chirho(value_chirho: i16) -> [u8; 2] {
    value_chirho.to_le_bytes()
}

/// Write a u32 in little-endian SWORD format.
#[inline]
pub fn write_u32_sword_chirho(value_chirho: u32) -> [u8; 4] {
    value_chirho.to_le_bytes()
}

/// Write an i32 in little-endian SWORD format.
#[inline]
pub fn write_i32_sword_chirho(value_chirho: i32) -> [u8; 4] {
    value_chirho.to_le_bytes()
}

/// Write a u64 in little-endian SWORD format.
#[inline]
pub fn write_u64_sword_chirho(value_chirho: u64) -> [u8; 8] {
    value_chirho.to_le_bytes()
}

/// Write an i64 in little-endian SWORD format.
#[inline]
pub fn write_i64_sword_chirho(value_chirho: i64) -> [u8; 8] {
    value_chirho.to_le_bytes()
}

/// Extension trait for reading SWORD format values from a reader.
pub trait ReadSwordChirho: Read {
    /// Read a u16 in SWORD format.
    fn read_u16_sword_chirho(&mut self) -> io::Result<u16> {
        let mut buf_chirho = [0u8; 2];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_u16_sword_chirho(&buf_chirho))
    }

    /// Read an i16 in SWORD format.
    fn read_i16_sword_chirho(&mut self) -> io::Result<i16> {
        let mut buf_chirho = [0u8; 2];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_i16_sword_chirho(&buf_chirho))
    }

    /// Read a u32 in SWORD format.
    fn read_u32_sword_chirho(&mut self) -> io::Result<u32> {
        let mut buf_chirho = [0u8; 4];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_u32_sword_chirho(&buf_chirho))
    }

    /// Read an i32 in SWORD format.
    fn read_i32_sword_chirho(&mut self) -> io::Result<i32> {
        let mut buf_chirho = [0u8; 4];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_i32_sword_chirho(&buf_chirho))
    }

    /// Read a u64 in SWORD format.
    fn read_u64_sword_chirho(&mut self) -> io::Result<u64> {
        let mut buf_chirho = [0u8; 8];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_u64_sword_chirho(&buf_chirho))
    }

    /// Read an i64 in SWORD format.
    fn read_i64_sword_chirho(&mut self) -> io::Result<i64> {
        let mut buf_chirho = [0u8; 8];
        self.read_exact(&mut buf_chirho)?;
        Ok(read_i64_sword_chirho(&buf_chirho))
    }
}

impl<R: Read + ?Sized> ReadSwordChirho for R {}

/// Extension trait for writing SWORD format values to a writer.
pub trait WriteSwordChirho: Write {
    /// Write a u16 in SWORD format.
    fn write_u16_sword_chirho(&mut self, value_chirho: u16) -> io::Result<()> {
        self.write_all(&write_u16_sword_chirho(value_chirho))
    }

    /// Write an i16 in SWORD format.
    fn write_i16_sword_chirho(&mut self, value_chirho: i16) -> io::Result<()> {
        self.write_all(&write_i16_sword_chirho(value_chirho))
    }

    /// Write a u32 in SWORD format.
    fn write_u32_sword_chirho(&mut self, value_chirho: u32) -> io::Result<()> {
        self.write_all(&write_u32_sword_chirho(value_chirho))
    }

    /// Write an i32 in SWORD format.
    fn write_i32_sword_chirho(&mut self, value_chirho: i32) -> io::Result<()> {
        self.write_all(&write_i32_sword_chirho(value_chirho))
    }

    /// Write a u64 in SWORD format.
    fn write_u64_sword_chirho(&mut self, value_chirho: u64) -> io::Result<()> {
        self.write_all(&write_u64_sword_chirho(value_chirho))
    }

    /// Write an i64 in SWORD format.
    fn write_i64_sword_chirho(&mut self, value_chirho: i64) -> io::Result<()> {
        self.write_all(&write_i64_sword_chirho(value_chirho))
    }
}

impl<W: Write + ?Sized> WriteSwordChirho for W {}

/// Index entry for RawVerse format (6 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawVerseIndexChirho {
    /// Offset into the text file.
    pub offset_chirho: u32,
    /// Size of the verse text.
    pub size_chirho: u16,
}

impl RawVerseIndexChirho {
    /// Size of a RawVerse index entry in bytes.
    pub const SIZE_CHIRHO: usize = 6;

    /// Read from a byte slice.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        debug_assert!(data_chirho.len() >= Self::SIZE_CHIRHO);
        Self {
            offset_chirho: read_u32_sword_chirho(&data_chirho[0..4]),
            size_chirho: read_u16_sword_chirho(&data_chirho[4..6]),
        }
    }

    /// Write to a byte array.
    pub fn to_bytes_chirho(&self) -> [u8; 6] {
        let mut buf_chirho = [0u8; 6];
        buf_chirho[0..4].copy_from_slice(&write_u32_sword_chirho(self.offset_chirho));
        buf_chirho[4..6].copy_from_slice(&write_u16_sword_chirho(self.size_chirho));
        buf_chirho
    }

    /// Check if this entry is empty (no text).
    pub fn is_empty_chirho(&self) -> bool {
        self.size_chirho == 0
    }
}

/// Index entry for RawVerse4 format (8 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawVerse4IndexChirho {
    /// Offset into the text file.
    pub offset_chirho: u32,
    /// Size of the verse text.
    pub size_chirho: u32,
}

impl RawVerse4IndexChirho {
    /// Size of a RawVerse4 index entry in bytes.
    pub const SIZE_CHIRHO: usize = 8;

    /// Read from a byte slice.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        debug_assert!(data_chirho.len() >= Self::SIZE_CHIRHO);
        Self {
            offset_chirho: read_u32_sword_chirho(&data_chirho[0..4]),
            size_chirho: read_u32_sword_chirho(&data_chirho[4..8]),
        }
    }

    /// Write to a byte array.
    pub fn to_bytes_chirho(&self) -> [u8; 8] {
        let mut buf_chirho = [0u8; 8];
        buf_chirho[0..4].copy_from_slice(&write_u32_sword_chirho(self.offset_chirho));
        buf_chirho[4..8].copy_from_slice(&write_u32_sword_chirho(self.size_chirho));
        buf_chirho
    }

    /// Check if this entry is empty (no text).
    pub fn is_empty_chirho(&self) -> bool {
        self.size_chirho == 0
    }
}

/// Index entry for zText compressed verse format (10 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZVerseIndexChirho {
    /// Block number containing this verse.
    pub block_num_chirho: u32,
    /// Offset within the uncompressed block.
    pub offset_in_block_chirho: u32,
    /// Size of the verse in uncompressed form.
    pub size_chirho: u16,
}

impl ZVerseIndexChirho {
    /// Size of a zVerse index entry in bytes.
    pub const SIZE_CHIRHO: usize = 10;

    /// Read from a byte slice.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        debug_assert!(data_chirho.len() >= Self::SIZE_CHIRHO);
        Self {
            block_num_chirho: read_u32_sword_chirho(&data_chirho[0..4]),
            offset_in_block_chirho: read_u32_sword_chirho(&data_chirho[4..8]),
            size_chirho: read_u16_sword_chirho(&data_chirho[8..10]),
        }
    }

    /// Write to a byte array.
    pub fn to_bytes_chirho(&self) -> [u8; 10] {
        let mut buf_chirho = [0u8; 10];
        buf_chirho[0..4].copy_from_slice(&write_u32_sword_chirho(self.block_num_chirho));
        buf_chirho[4..8].copy_from_slice(&write_u32_sword_chirho(self.offset_in_block_chirho));
        buf_chirho[8..10].copy_from_slice(&write_u16_sword_chirho(self.size_chirho));
        buf_chirho
    }

    /// Check if this entry is empty (no text).
    pub fn is_empty_chirho(&self) -> bool {
        self.size_chirho == 0
    }
}

/// Block index entry for zText compressed format (12 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZBlockIndexChirho {
    /// Offset of compressed data in the .czz file.
    pub comp_offset_chirho: u32,
    /// Size of compressed data.
    pub comp_size_chirho: u32,
    /// Size of uncompressed data.
    pub uncomp_size_chirho: u32,
}

impl ZBlockIndexChirho {
    /// Size of a zBlock index entry in bytes.
    pub const SIZE_CHIRHO: usize = 12;

    /// Read from a byte slice.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        debug_assert!(data_chirho.len() >= Self::SIZE_CHIRHO);
        Self {
            comp_offset_chirho: read_u32_sword_chirho(&data_chirho[0..4]),
            comp_size_chirho: read_u32_sword_chirho(&data_chirho[4..8]),
            uncomp_size_chirho: read_u32_sword_chirho(&data_chirho[8..12]),
        }
    }

    /// Write to a byte array.
    pub fn to_bytes_chirho(&self) -> [u8; 12] {
        let mut buf_chirho = [0u8; 12];
        buf_chirho[0..4].copy_from_slice(&write_u32_sword_chirho(self.comp_offset_chirho));
        buf_chirho[4..8].copy_from_slice(&write_u32_sword_chirho(self.comp_size_chirho));
        buf_chirho[8..12].copy_from_slice(&write_u32_sword_chirho(self.uncomp_size_chirho));
        buf_chirho
    }

    /// Check if this block is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.comp_size_chirho == 0
    }
}

/// Index entry for zText4 compressed verse format (12 bytes).
/// Uses 4-byte size field instead of 2-byte to support larger entries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZVerse4IndexChirho {
    /// Block number containing this verse.
    pub block_num_chirho: u32,
    /// Offset within the uncompressed block.
    pub offset_in_block_chirho: u32,
    /// Size of the verse in uncompressed form (4-byte).
    pub size_chirho: u32,
}

impl ZVerse4IndexChirho {
    /// Size of a zVerse4 index entry in bytes.
    pub const SIZE_CHIRHO: usize = 12;

    /// Read from a byte slice.
    pub fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        debug_assert!(data_chirho.len() >= Self::SIZE_CHIRHO);
        Self {
            block_num_chirho: read_u32_sword_chirho(&data_chirho[0..4]),
            offset_in_block_chirho: read_u32_sword_chirho(&data_chirho[4..8]),
            size_chirho: read_u32_sword_chirho(&data_chirho[8..12]),
        }
    }

    /// Write to a byte array.
    pub fn to_bytes_chirho(&self) -> [u8; 12] {
        let mut buf_chirho = [0u8; 12];
        buf_chirho[0..4].copy_from_slice(&write_u32_sword_chirho(self.block_num_chirho));
        buf_chirho[4..8].copy_from_slice(&write_u32_sword_chirho(self.offset_in_block_chirho));
        buf_chirho[8..12].copy_from_slice(&write_u32_sword_chirho(self.size_chirho));
        buf_chirho
    }

    /// Check if this entry is empty (no text).
    pub fn is_empty_chirho(&self) -> bool {
        self.size_chirho == 0
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_u16_roundtrip_chirho() {
        let value_chirho = 0x1234u16;
        let bytes_chirho = write_u16_sword_chirho(value_chirho);
        assert_eq!(bytes_chirho, [0x34, 0x12]); // Little-endian
        assert_eq!(read_u16_sword_chirho(&bytes_chirho), value_chirho);
    }

    #[test]
    fn test_u32_roundtrip_chirho() {
        let value_chirho = 0x12345678u32;
        let bytes_chirho = write_u32_sword_chirho(value_chirho);
        assert_eq!(bytes_chirho, [0x78, 0x56, 0x34, 0x12]); // Little-endian
        assert_eq!(read_u32_sword_chirho(&bytes_chirho), value_chirho);
    }

    #[test]
    fn test_i32_roundtrip_chirho() {
        let value_chirho = -12345i32;
        let bytes_chirho = write_i32_sword_chirho(value_chirho);
        assert_eq!(read_i32_sword_chirho(&bytes_chirho), value_chirho);
    }

    #[test]
    fn test_raw_verse_index_chirho() {
        let index_chirho = RawVerseIndexChirho {
            offset_chirho: 0x1000,
            size_chirho: 0x0100,
        };
        let bytes_chirho = index_chirho.to_bytes_chirho();
        assert_eq!(bytes_chirho.len(), RawVerseIndexChirho::SIZE_CHIRHO);

        let parsed_chirho = RawVerseIndexChirho::from_bytes_chirho(&bytes_chirho);
        assert_eq!(parsed_chirho, index_chirho);
    }

    #[test]
    fn test_raw_verse4_index_chirho() {
        let index_chirho = RawVerse4IndexChirho {
            offset_chirho: 0x10000000,
            size_chirho: 0x00010000,
        };
        let bytes_chirho = index_chirho.to_bytes_chirho();
        assert_eq!(bytes_chirho.len(), RawVerse4IndexChirho::SIZE_CHIRHO);

        let parsed_chirho = RawVerse4IndexChirho::from_bytes_chirho(&bytes_chirho);
        assert_eq!(parsed_chirho, index_chirho);
    }

    #[test]
    fn test_z_verse_index_chirho() {
        let index_chirho = ZVerseIndexChirho {
            block_num_chirho: 42,
            offset_in_block_chirho: 1024,
            size_chirho: 256,
        };
        let bytes_chirho = index_chirho.to_bytes_chirho();
        assert_eq!(bytes_chirho.len(), ZVerseIndexChirho::SIZE_CHIRHO);

        let parsed_chirho = ZVerseIndexChirho::from_bytes_chirho(&bytes_chirho);
        assert_eq!(parsed_chirho, index_chirho);
    }

    #[test]
    fn test_z_block_index_chirho() {
        let index_chirho = ZBlockIndexChirho {
            comp_offset_chirho: 0x1000,
            comp_size_chirho: 0x0200,
            uncomp_size_chirho: 0x0800,
        };
        let bytes_chirho = index_chirho.to_bytes_chirho();
        assert_eq!(bytes_chirho.len(), ZBlockIndexChirho::SIZE_CHIRHO);

        let parsed_chirho = ZBlockIndexChirho::from_bytes_chirho(&bytes_chirho);
        assert_eq!(parsed_chirho, index_chirho);
    }

    #[test]
    fn test_reader_extension_chirho() {
        let data_chirho = vec![0x34, 0x12, 0x78, 0x56, 0x34, 0x12];
        let mut cursor_chirho = Cursor::new(data_chirho);

        assert_eq!(cursor_chirho.read_u16_sword_chirho().unwrap(), 0x1234);
        assert_eq!(cursor_chirho.read_u32_sword_chirho().unwrap(), 0x12345678);
    }

    #[test]
    fn test_writer_extension_chirho() {
        let mut buf_chirho = Vec::new();
        buf_chirho.write_u16_sword_chirho(0x1234).unwrap();
        buf_chirho.write_u32_sword_chirho(0x12345678).unwrap();

        assert_eq!(buf_chirho, vec![0x34, 0x12, 0x78, 0x56, 0x34, 0x12]);
    }
}
