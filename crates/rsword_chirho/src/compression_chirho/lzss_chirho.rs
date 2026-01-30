// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! LZSS compression/decompression for legacy SWORD modules.
//!
//! LZSS (Lempel-Ziv-Storer-Szymanski) is a dictionary-based compression
//! algorithm used by older SWORD modules. This implementation is compatible
//! with the SWORD library's LZSS format.

use crate::compression_chirho::CompressorChirho;
use crate::error_chirho::ResultChirho;

/// Size of the ring buffer (must be power of 2)
const RING_BUFFER_SIZE_CHIRHO: usize = 4096;

/// Maximum match length
const MAX_MATCH_LENGTH_CHIRHO: usize = 18;

/// Minimum match length (threshold for encoding as reference)
const MIN_MATCH_LENGTH_CHIRHO: usize = 3;

/// Index mask for ring buffer
const RING_BUFFER_MASK_CHIRHO: usize = RING_BUFFER_SIZE_CHIRHO - 1;

/// LZSS compressor implementing the SWORD-compatible LZSS algorithm.
#[derive(Debug, Clone, Default)]
pub struct LzssCompressorChirho;

impl LzssCompressorChirho {
    /// Create a new LZSS compressor.
    pub fn new_chirho() -> Self {
        Self
    }

    /// Decompress LZSS-encoded data.
    ///
    /// The LZSS format uses flag bytes to indicate whether subsequent bytes
    /// are literal characters or back-references to previously seen data.
    fn decompress_internal_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        if data_chirho.is_empty() {
            return Ok(Vec::new());
        }

        let mut output_chirho = Vec::with_capacity(data_chirho.len() * 2);
        let mut ring_buffer_chirho = vec![b' '; RING_BUFFER_SIZE_CHIRHO];
        let mut ring_pos_chirho = RING_BUFFER_SIZE_CHIRHO - MAX_MATCH_LENGTH_CHIRHO;
        let mut input_pos_chirho = 0;

        while input_pos_chirho < data_chirho.len() {
            // Read flag byte - each bit indicates literal (1) or reference (0)
            let flags_chirho = data_chirho[input_pos_chirho];
            input_pos_chirho += 1;

            for bit_chirho in 0..8 {
                if input_pos_chirho >= data_chirho.len() {
                    break;
                }

                if (flags_chirho >> bit_chirho) & 1 != 0 {
                    // Literal byte
                    let byte_chirho = data_chirho[input_pos_chirho];
                    input_pos_chirho += 1;
                    output_chirho.push(byte_chirho);
                    ring_buffer_chirho[ring_pos_chirho] = byte_chirho;
                    ring_pos_chirho = (ring_pos_chirho + 1) & RING_BUFFER_MASK_CHIRHO;
                } else {
                    // Back-reference: 2 bytes encode position and length
                    if input_pos_chirho + 1 >= data_chirho.len() {
                        break;
                    }

                    let byte1_chirho = data_chirho[input_pos_chirho] as usize;
                    let byte2_chirho = data_chirho[input_pos_chirho + 1] as usize;
                    input_pos_chirho += 2;

                    // Position is 12 bits, length is 4 bits
                    let position_chirho = byte1_chirho | ((byte2_chirho & 0xF0) << 4);
                    let length_chirho = (byte2_chirho & 0x0F) + MIN_MATCH_LENGTH_CHIRHO;

                    // Copy from ring buffer
                    for i_chirho in 0..length_chirho {
                        let src_pos_chirho = (position_chirho + i_chirho) & RING_BUFFER_MASK_CHIRHO;
                        let byte_chirho = ring_buffer_chirho[src_pos_chirho];
                        output_chirho.push(byte_chirho);
                        ring_buffer_chirho[ring_pos_chirho] = byte_chirho;
                        ring_pos_chirho = (ring_pos_chirho + 1) & RING_BUFFER_MASK_CHIRHO;
                    }
                }
            }
        }

        Ok(output_chirho)
    }

    /// Compress data using LZSS algorithm.
    fn compress_internal_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        if data_chirho.is_empty() {
            return Ok(Vec::new());
        }

        let mut output_chirho = Vec::with_capacity(data_chirho.len());
        let mut ring_buffer_chirho = vec![b' '; RING_BUFFER_SIZE_CHIRHO];
        let mut ring_pos_chirho = RING_BUFFER_SIZE_CHIRHO - MAX_MATCH_LENGTH_CHIRHO;
        let mut input_pos_chirho = 0;

        // Temporary buffer for 8 items (controlled by flag byte)
        let mut code_buf_chirho: Vec<u8> = Vec::with_capacity(17); // 1 flag + up to 16 data bytes
        let mut mask_chirho: u8 = 1;
        let mut flags_chirho: u8 = 0;

        code_buf_chirho.push(0); // Placeholder for flags

        while input_pos_chirho < data_chirho.len() {
            // Find longest match in ring buffer
            let (match_pos_chirho, match_len_chirho) =
                self.find_longest_match_chirho(&ring_buffer_chirho, ring_pos_chirho, data_chirho, input_pos_chirho);

            if match_len_chirho >= MIN_MATCH_LENGTH_CHIRHO {
                // Encode as back-reference
                let byte1_chirho = (match_pos_chirho & 0xFF) as u8;
                let byte2_chirho = (((match_pos_chirho >> 4) & 0xF0) | ((match_len_chirho - MIN_MATCH_LENGTH_CHIRHO) & 0x0F)) as u8;
                code_buf_chirho.push(byte1_chirho);
                code_buf_chirho.push(byte2_chirho);

                // Add to ring buffer
                for i_chirho in 0..match_len_chirho {
                    ring_buffer_chirho[ring_pos_chirho] = data_chirho[input_pos_chirho + i_chirho];
                    ring_pos_chirho = (ring_pos_chirho + 1) & RING_BUFFER_MASK_CHIRHO;
                }
                input_pos_chirho += match_len_chirho;
            } else {
                // Encode as literal
                flags_chirho |= mask_chirho;
                let byte_chirho = data_chirho[input_pos_chirho];
                code_buf_chirho.push(byte_chirho);
                ring_buffer_chirho[ring_pos_chirho] = byte_chirho;
                ring_pos_chirho = (ring_pos_chirho + 1) & RING_BUFFER_MASK_CHIRHO;
                input_pos_chirho += 1;
            }

            mask_chirho <<= 1;

            // When we've processed 8 items, flush the buffer
            if mask_chirho == 0 {
                code_buf_chirho[0] = flags_chirho;
                output_chirho.extend_from_slice(&code_buf_chirho);
                code_buf_chirho.clear();
                code_buf_chirho.push(0);
                flags_chirho = 0;
                mask_chirho = 1;
            }
        }

        // Flush remaining data
        if code_buf_chirho.len() > 1 {
            code_buf_chirho[0] = flags_chirho;
            output_chirho.extend_from_slice(&code_buf_chirho);
        }

        Ok(output_chirho)
    }

    /// Find the longest match in the ring buffer for the current position.
    fn find_longest_match_chirho(
        &self,
        ring_buffer_chirho: &[u8],
        ring_pos_chirho: usize,
        data_chirho: &[u8],
        input_pos_chirho: usize,
    ) -> (usize, usize) {
        let mut best_pos_chirho = 0;
        let mut best_len_chirho = 0;

        let remaining_chirho = data_chirho.len() - input_pos_chirho;
        let max_len_chirho = remaining_chirho.min(MAX_MATCH_LENGTH_CHIRHO);

        if max_len_chirho < MIN_MATCH_LENGTH_CHIRHO {
            return (0, 0);
        }

        // Search the ring buffer for matches
        for offset_chirho in 1..RING_BUFFER_SIZE_CHIRHO {
            let search_pos_chirho = (ring_pos_chirho + RING_BUFFER_SIZE_CHIRHO - offset_chirho) & RING_BUFFER_MASK_CHIRHO;
            let mut match_len_chirho = 0;

            while match_len_chirho < max_len_chirho {
                let ring_idx_chirho = (search_pos_chirho + match_len_chirho) & RING_BUFFER_MASK_CHIRHO;
                if ring_buffer_chirho[ring_idx_chirho] != data_chirho[input_pos_chirho + match_len_chirho] {
                    break;
                }
                match_len_chirho += 1;
            }

            if match_len_chirho > best_len_chirho {
                best_pos_chirho = search_pos_chirho;
                best_len_chirho = match_len_chirho;
                if best_len_chirho == max_len_chirho {
                    break;
                }
            }
        }

        (best_pos_chirho, best_len_chirho)
    }
}

impl CompressorChirho for LzssCompressorChirho {
    fn compress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        self.compress_internal_chirho(data_chirho)
    }

    fn decompress_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        self.decompress_internal_chirho(data_chirho)
    }

    fn name_chirho(&self) -> &str {
        "LZSS"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_lzss_empty_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        let compressed_chirho = compressor_chirho.compress_chirho(&[]).unwrap();
        assert!(compressed_chirho.is_empty());

        let decompressed_chirho = compressor_chirho.decompress_chirho(&[]).unwrap();
        assert!(decompressed_chirho.is_empty());
    }

    #[test]
    fn test_lzss_roundtrip_simple_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        let original_chirho = b"Hello, World!";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_lzss_roundtrip_repetitive_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        // Repetitive data should compress well
        let original_chirho = b"ABCABCABCABCABCABCABCABC";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(decompressed_chirho, original_chirho);
        // Repetitive data should compress
        assert!(compressed_chirho.len() < original_chirho.len());
    }

    #[test]
    fn test_lzss_roundtrip_bible_text_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        let original_chirho = b"In the beginning God created the heaven and the earth. \
            And the earth was without form, and void; and darkness was upon the face of the deep. \
            And the Spirit of God moved upon the face of the waters.";

        let compressed_chirho = compressor_chirho.compress_chirho(original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_lzss_roundtrip_random_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        // Pseudo-random data (less compressible)
        let original_chirho: Vec<u8> = (0..256).map(|i| ((i * 17 + 31) % 256) as u8).collect();

        let compressed_chirho = compressor_chirho.compress_chirho(&original_chirho).unwrap();
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho).unwrap();

        assert_eq!(decompressed_chirho, original_chirho);
    }

    #[test]
    fn test_lzss_name_chirho() {
        let compressor_chirho = LzssCompressorChirho::new_chirho();
        assert_eq!(compressor_chirho.name_chirho(), "LZSS");
    }
}
