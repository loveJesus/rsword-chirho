// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Compression support for SWORD modules.

mod lzss_chirho;

use std::io::Read;
use flate2::read::{ZlibDecoder, ZlibEncoder};
use flate2::Compression;

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::CompressionTypeChirho;

pub use lzss_chirho::LzssCompressorChirho;

/// Trait for compression/decompression.
pub trait CompressorChirho: Send + Sync {
    /// Compress data.
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>>;

    /// Decompress data.
    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>>;

    /// Get the compressor name.
    fn name_chirho(&self) -> &str;
}

/// No compression (passthrough).
pub struct NoCompressorChirho;

impl CompressorChirho for NoCompressorChirho {
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        Ok(data_chirho.to_vec())
    }

    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        Ok(data_chirho.to_vec())
    }

    fn name_chirho(&self) -> &str {
        "None"
    }
}

/// zlib/ZIP compression.
pub struct ZipCompressorChirho {
    level_chirho: Compression,
}

impl ZipCompressorChirho {
    /// Create a new ZIP compressor with default compression level.
    pub fn new_chirho() -> Self {
        Self {
            level_chirho: Compression::default(),
        }
    }

    /// Create a new ZIP compressor with specified compression level (0-9).
    pub fn with_level_chirho(level_chirho: u32) -> Self {
        Self {
            level_chirho: Compression::new(level_chirho),
        }
    }
}

impl Default for ZipCompressorChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl CompressorChirho for ZipCompressorChirho {
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut encoder_chirho = ZlibEncoder::new(data_chirho, self.level_chirho);
        let mut compressed_chirho = Vec::new();
        encoder_chirho.read_to_end(&mut compressed_chirho)
            .map_err(|e| ErrorChirho::CompressionChirho { message_chirho: e.to_string() })?;
        Ok(compressed_chirho)
    }

    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut decoder_chirho = ZlibDecoder::new(data_chirho);
        let mut decompressed_chirho = Vec::new();
        decoder_chirho.read_to_end(&mut decompressed_chirho)
            .map_err(|e| ErrorChirho::DecompressionChirho { message_chirho: e.to_string() })?;
        Ok(decompressed_chirho)
    }

    fn name_chirho(&self) -> &str {
        "ZIP"
    }
}

/// bzip2 compression.
pub struct Bzip2CompressorChirho {
    level_chirho: bzip2::Compression,
}

impl Bzip2CompressorChirho {
    /// Create a new bzip2 compressor with default compression level.
    pub fn new_chirho() -> Self {
        Self {
            level_chirho: bzip2::Compression::default(),
        }
    }

    /// Create a new bzip2 compressor with specified compression level (1-9).
    pub fn with_level_chirho(level_chirho: u32) -> Self {
        Self {
            level_chirho: bzip2::Compression::new(level_chirho),
        }
    }
}

impl Default for Bzip2CompressorChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl CompressorChirho for Bzip2CompressorChirho {
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut encoder_chirho = bzip2::read::BzEncoder::new(data_chirho, self.level_chirho);
        let mut compressed_chirho = Vec::new();
        encoder_chirho.read_to_end(&mut compressed_chirho)
            .map_err(|e| ErrorChirho::CompressionChirho { message_chirho: e.to_string() })?;
        Ok(compressed_chirho)
    }

    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut decoder_chirho = bzip2::read::BzDecoder::new(data_chirho);
        let mut decompressed_chirho = Vec::new();
        decoder_chirho.read_to_end(&mut decompressed_chirho)
            .map_err(|e| ErrorChirho::DecompressionChirho { message_chirho: e.to_string() })?;
        Ok(decompressed_chirho)
    }

    fn name_chirho(&self) -> &str {
        "BZIP2"
    }
}

/// XZ/LZMA compression.
pub struct XzCompressorChirho {
    level_chirho: u32,
}

impl XzCompressorChirho {
    /// Create a new XZ compressor with default compression level.
    pub fn new_chirho() -> Self {
        Self { level_chirho: 6 }
    }

    /// Create a new XZ compressor with specified compression level (0-9).
    pub fn with_level_chirho(level_chirho: u32) -> Self {
        Self { level_chirho }
    }
}

impl Default for XzCompressorChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl CompressorChirho for XzCompressorChirho {
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut encoder_chirho = xz2::read::XzEncoder::new(data_chirho, self.level_chirho);
        let mut compressed_chirho = Vec::new();
        encoder_chirho.read_to_end(&mut compressed_chirho)
            .map_err(|e| ErrorChirho::CompressionChirho { message_chirho: e.to_string() })?;
        Ok(compressed_chirho)
    }

    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        let mut decoder_chirho = xz2::read::XzDecoder::new(data_chirho);
        let mut decompressed_chirho = Vec::new();
        decoder_chirho.read_to_end(&mut decompressed_chirho)
            .map_err(|e| ErrorChirho::DecompressionChirho { message_chirho: e.to_string() })?;
        Ok(decompressed_chirho)
    }

    fn name_chirho(&self) -> &str {
        "XZ"
    }
}

/// Create a compressor from compression type.
pub fn create_compressor_chirho(comp_type_chirho: CompressionTypeChirho) -> Box<dyn CompressorChirho> {
    match comp_type_chirho {
        CompressionTypeChirho::NoneChirho => Box::new(NoCompressorChirho),
        CompressionTypeChirho::ZipChirho => Box::new(ZipCompressorChirho::new_chirho()),
        CompressionTypeChirho::Bzip2Chirho => Box::new(Bzip2CompressorChirho::new_chirho()),
        CompressionTypeChirho::XzChirho => Box::new(XzCompressorChirho::new_chirho()),
        CompressionTypeChirho::LzssChirho => Box::new(LzssCompressorChirho::new_chirho()),
    }
}

/// Parse compression type from config string.
pub fn parse_compression_type_chirho(s: &str) -> CompressionTypeChirho {
    match s.to_uppercase().as_str() {
        "ZIP" | "ZLIB" => CompressionTypeChirho::ZipChirho,
        "BZIP2" | "BZ2" => CompressionTypeChirho::Bzip2Chirho,
        "XZ" | "LZMA" => CompressionTypeChirho::XzChirho,
        "LZSS" => CompressionTypeChirho::LzssChirho,
        _ => CompressionTypeChirho::NoneChirho,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_zip_roundtrip_chirho() {
        let compressor_chirho = ZipCompressorChirho::new_chirho();
        let original_chirho = b"For God so loved the world...";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(&decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_bzip2_roundtrip_chirho() {
        let compressor_chirho = Bzip2CompressorChirho::new_chirho();
        let original_chirho = b"For God so loved the world...";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(&decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_xz_roundtrip_chirho() {
        let compressor_chirho = XzCompressorChirho::new_chirho();
        let original_chirho = b"For God so loved the world...";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(&decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_no_compressor_chirho() {
        let compressor_chirho = NoCompressorChirho;
        let original_chirho = b"For God so loved the world...";

        let result_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        assert_eq!(&result_chirho, original_chirho);
    }

    #[test]
    fn test_parse_compression_type_chirho() {
        assert_eq!(parse_compression_type_chirho("ZIP"), CompressionTypeChirho::ZipChirho);
        assert_eq!(parse_compression_type_chirho("BZIP2"), CompressionTypeChirho::Bzip2Chirho);
        assert_eq!(parse_compression_type_chirho("XZ"), CompressionTypeChirho::XzChirho);
        assert_eq!(parse_compression_type_chirho(""), CompressionTypeChirho::NoneChirho);
    }
}
