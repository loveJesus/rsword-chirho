// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD cipher implementations.
//!
//! The primary cipher used by SWORD is the Sapphire stream cipher,
//! a simple but effective cipher for protecting copyrighted content.

use crate::error_chirho::ResultChirho;
use super::CipherChirho;

/// Cipher type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherTypeChirho {
    /// No encryption.
    #[default]
    NoneChirho,
    /// Sapphire stream cipher (most common).
    SapphireChirho,
    /// Simple XOR cipher (legacy).
    XorChirho,
}

impl CipherTypeChirho {
    /// Parse cipher type from string.
    pub fn from_str_chirho(s_chirho: &str) -> Self {
        match s_chirho.to_lowercase().as_str() {
            "sapphire" | "sapphire2" => Self::SapphireChirho,
            "xor" => Self::XorChirho,
            _ => Self::NoneChirho,
        }
    }
}

/// SWORD cipher wrapper.
#[derive(Debug, Clone)]
pub struct SwCipherChirho {
    /// Cipher type.
    cipher_type_chirho: CipherTypeChirho,
    /// Encryption key.
    key_chirho: String,
}

impl SwCipherChirho {
    /// Create a new SWORD cipher.
    pub fn new_chirho(cipher_type_chirho: CipherTypeChirho, key_chirho: &str) -> Self {
        Self {
            cipher_type_chirho,
            key_chirho: key_chirho.to_string(),
        }
    }

    /// Get the cipher type.
    pub fn cipher_type_chirho(&self) -> CipherTypeChirho {
        self.cipher_type_chirho
    }

    /// Set the encryption key.
    pub fn set_key_chirho(&mut self, key_chirho: &str) {
        self.key_chirho = key_chirho.to_string();
    }

    /// Get the encryption key.
    pub fn key_chirho(&self) -> &str {
        &self.key_chirho
    }
}

/// Sapphire stream cipher implementation.
///
/// The Sapphire cipher is a simple stream cipher used by SWORD.
/// It uses a 256-byte state array shuffled based on the key.
///
/// Reference: SWORD source code src/modules/common/sapphire.cpp
#[derive(Debug, Clone)]
pub struct SapphireCipherChirho {
    /// Cipher key.
    key_chirho: String,
}

impl SapphireCipherChirho {
    /// Create a new Sapphire cipher with the given key.
    pub fn new_chirho(key_chirho: &str) -> Self {
        Self {
            key_chirho: key_chirho.to_string(),
        }
    }

    /// Initialize the cipher state from the key.
    fn init_state_chirho(&self) -> SapphireStateChirho {
        SapphireStateChirho::new_chirho(&self.key_chirho)
    }
}

impl CipherChirho for SapphireCipherChirho {
    fn encrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        if self.key_chirho.is_empty() {
            return Ok(data_chirho.to_vec());
        }

        let mut state_chirho = self.init_state_chirho();
        let mut result_chirho = Vec::with_capacity(data_chirho.len());

        for &byte_chirho in data_chirho {
            result_chirho.push(byte_chirho ^ state_chirho.next_chirho());
        }

        Ok(result_chirho)
    }

    fn decrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        // Sapphire is symmetric - encryption and decryption are the same
        self.encrypt_chirho(data_chirho)
    }

    fn name_chirho(&self) -> &str {
        "Sapphire"
    }

    fn has_key_chirho(&self) -> bool {
        !self.key_chirho.is_empty()
    }
}

/// Sapphire cipher internal state.
#[derive(Debug, Clone)]
struct SapphireStateChirho {
    /// 256-byte permutation array.
    cards_chirho: [u8; 256],
    /// Rotor position.
    rotor_chirho: u8,
    /// Ratchet position.
    ratchet_chirho: u8,
    /// Avalanche counter.
    avalanche_chirho: u8,
    /// Last plain byte (for stream).
    last_plain_chirho: u8,
    /// Last cipher byte (for stream).
    last_cipher_chirho: u8,
}

impl SapphireStateChirho {
    /// Create a new state initialized with the given key.
    fn new_chirho(key_chirho: &str) -> Self {
        let mut state_chirho = Self {
            cards_chirho: [0; 256],
            rotor_chirho: 1,
            ratchet_chirho: 3,
            avalanche_chirho: 5,
            last_plain_chirho: 7,
            last_cipher_chirho: 11,
        };

        // Initialize cards to identity permutation
        for i_chirho in 0..256 {
            state_chirho.cards_chirho[i_chirho] = i_chirho as u8;
        }

        // Shuffle based on key
        if !key_chirho.is_empty() {
            let key_bytes_chirho = key_chirho.as_bytes();
            let key_len_chirho = key_bytes_chirho.len();

            for i_chirho in 0..256 {
                let key_byte_chirho = key_bytes_chirho[i_chirho % key_len_chirho];
                let swap_idx_chirho = ((key_byte_chirho as usize) + i_chirho) % 256;

                state_chirho.cards_chirho.swap(i_chirho, swap_idx_chirho);
            }

            // Additional mixing
            for _ in 0..256 {
                state_chirho.next_chirho();
            }
        }

        state_chirho
    }

    /// Generate the next keystream byte.
    fn next_chirho(&mut self) -> u8 {
        // Update rotor
        self.rotor_chirho = self.rotor_chirho.wrapping_add(1);

        // Update ratchet
        self.ratchet_chirho = self.ratchet_chirho.wrapping_add(
            self.cards_chirho[self.rotor_chirho as usize]
        );

        // Swap cards
        self.cards_chirho.swap(
            self.rotor_chirho as usize,
            self.ratchet_chirho as usize
        );

        // Update avalanche
        self.avalanche_chirho = self.avalanche_chirho.wrapping_add(
            self.cards_chirho[self.ratchet_chirho as usize]
        );

        // Update last values
        self.last_plain_chirho = self.cards_chirho[
            self.avalanche_chirho.wrapping_add(
                self.cards_chirho[
                    self.rotor_chirho.wrapping_add(
                        self.cards_chirho[self.last_plain_chirho as usize]
                    ) as usize
                ]
            ) as usize
        ];

        self.last_cipher_chirho = self.cards_chirho[
            self.last_cipher_chirho.wrapping_add(
                self.cards_chirho[
                    self.ratchet_chirho.wrapping_add(
                        self.cards_chirho[self.last_plain_chirho as usize]
                    ) as usize
                ]
            ) as usize
        ];

        self.last_cipher_chirho
    }
}

/// Simple XOR cipher implementation.
///
/// A basic XOR cipher that cycles through the key bytes.
/// Used in some legacy SWORD modules.
#[derive(Debug, Clone)]
pub struct XorCipherChirho {
    /// Cipher key.
    key_chirho: String,
}

impl XorCipherChirho {
    /// Create a new XOR cipher with the given key.
    pub fn new_chirho(key_chirho: &str) -> Self {
        Self {
            key_chirho: key_chirho.to_string(),
        }
    }
}

impl CipherChirho for XorCipherChirho {
    fn encrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        if self.key_chirho.is_empty() {
            return Ok(data_chirho.to_vec());
        }

        let key_bytes_chirho = self.key_chirho.as_bytes();
        let key_len_chirho = key_bytes_chirho.len();
        let mut result_chirho = Vec::with_capacity(data_chirho.len());

        for (i_chirho, &byte_chirho) in data_chirho.iter().enumerate() {
            result_chirho.push(byte_chirho ^ key_bytes_chirho[i_chirho % key_len_chirho]);
        }

        Ok(result_chirho)
    }

    fn decrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        // XOR is symmetric
        self.encrypt_chirho(data_chirho)
    }

    fn name_chirho(&self) -> &str {
        "XOR"
    }

    fn has_key_chirho(&self) -> bool {
        !self.key_chirho.is_empty()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_sapphire_roundtrip_chirho() {
        let cipher_chirho = SapphireCipherChirho::new_chirho("test_key");
        let plaintext_chirho = b"In the beginning God created the heaven and the earth.";

        let encrypted_chirho = cipher_chirho.encrypt_chirho(plaintext_chirho).unwrap();
        assert_ne!(encrypted_chirho, plaintext_chirho);

        let decrypted_chirho = cipher_chirho.decrypt_chirho(&encrypted_chirho).unwrap();
        assert_eq!(decrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_sapphire_empty_key_chirho() {
        let cipher_chirho = SapphireCipherChirho::new_chirho("");
        let plaintext_chirho = b"Test data";

        let encrypted_chirho = cipher_chirho.encrypt_chirho(plaintext_chirho).unwrap();
        assert_eq!(encrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_sapphire_different_keys_chirho() {
        let cipher1_chirho = SapphireCipherChirho::new_chirho("key1");
        let cipher2_chirho = SapphireCipherChirho::new_chirho("key2");
        let plaintext_chirho = b"Test data";

        let encrypted1_chirho = cipher1_chirho.encrypt_chirho(plaintext_chirho).unwrap();
        let encrypted2_chirho = cipher2_chirho.encrypt_chirho(plaintext_chirho).unwrap();

        assert_ne!(encrypted1_chirho, encrypted2_chirho);
    }

    #[test]
    fn test_xor_roundtrip_chirho() {
        let cipher_chirho = XorCipherChirho::new_chirho("test_key");
        let plaintext_chirho = b"In the beginning God created the heaven and the earth.";

        let encrypted_chirho = cipher_chirho.encrypt_chirho(plaintext_chirho).unwrap();
        assert_ne!(encrypted_chirho, plaintext_chirho);

        let decrypted_chirho = cipher_chirho.decrypt_chirho(&encrypted_chirho).unwrap();
        assert_eq!(decrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_xor_empty_key_chirho() {
        let cipher_chirho = XorCipherChirho::new_chirho("");
        let plaintext_chirho = b"Test data";

        let encrypted_chirho = cipher_chirho.encrypt_chirho(plaintext_chirho).unwrap();
        assert_eq!(encrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_cipher_type_from_str_chirho() {
        assert_eq!(CipherTypeChirho::from_str_chirho("sapphire"), CipherTypeChirho::SapphireChirho);
        assert_eq!(CipherTypeChirho::from_str_chirho("SAPPHIRE"), CipherTypeChirho::SapphireChirho);
        assert_eq!(CipherTypeChirho::from_str_chirho("sapphire2"), CipherTypeChirho::SapphireChirho);
        assert_eq!(CipherTypeChirho::from_str_chirho("xor"), CipherTypeChirho::XorChirho);
        assert_eq!(CipherTypeChirho::from_str_chirho("unknown"), CipherTypeChirho::NoneChirho);
    }

    #[test]
    fn test_sw_cipher_chirho() {
        let mut cipher_chirho = SwCipherChirho::new_chirho(CipherTypeChirho::SapphireChirho, "key1");

        assert_eq!(cipher_chirho.cipher_type_chirho(), CipherTypeChirho::SapphireChirho);
        assert_eq!(cipher_chirho.key_chirho(), "key1");

        cipher_chirho.set_key_chirho("key2");
        assert_eq!(cipher_chirho.key_chirho(), "key2");
    }

    #[test]
    fn test_sapphire_long_data_chirho() {
        let cipher_chirho = SapphireCipherChirho::new_chirho("long_key_for_testing");
        let plaintext_chirho: Vec<u8> = (0..1000).map(|i| (i % 256) as u8).collect();

        let encrypted_chirho = cipher_chirho.encrypt_chirho(&plaintext_chirho).unwrap();
        assert_eq!(encrypted_chirho.len(), plaintext_chirho.len());
        assert_ne!(encrypted_chirho, plaintext_chirho);

        let decrypted_chirho = cipher_chirho.decrypt_chirho(&encrypted_chirho).unwrap();
        assert_eq!(decrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_has_key_chirho() {
        let cipher_with_key_chirho = SapphireCipherChirho::new_chirho("key");
        let cipher_no_key_chirho = SapphireCipherChirho::new_chirho("");

        assert!(cipher_with_key_chirho.has_key_chirho());
        assert!(!cipher_no_key_chirho.has_key_chirho());
    }
}
