// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module encryption and decryption support.
//!
//! SWORD modules can be encrypted to protect copyrighted content.
//! This module provides support for:
//! - Sapphire cipher (SWORD's custom stream cipher, most common)
//! - Basic XOR cipher (legacy)
//!
//! Encrypted modules require an unlock key to access content.

mod sw_cipher_chirho;

pub use sw_cipher_chirho::{
    SwCipherChirho, CipherTypeChirho, SapphireCipherChirho, XorCipherChirho,
};

use crate::error_chirho::ResultChirho;

/// Trait for cipher implementations.
pub trait CipherChirho: Send + Sync + std::fmt::Debug {
    /// Encrypt data using the cipher.
    fn encrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>>;

    /// Decrypt data using the cipher.
    fn decrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>>;

    /// Get the cipher name.
    fn name_chirho(&self) -> &str;

    /// Check if a key is set.
    fn has_key_chirho(&self) -> bool;
}

/// Create a cipher from type and key.
pub fn create_cipher_chirho(cipher_type_chirho: CipherTypeChirho, key_chirho: &str) -> Box<dyn CipherChirho> {
    match cipher_type_chirho {
        CipherTypeChirho::NoneChirho => Box::new(NullCipherChirho),
        CipherTypeChirho::SapphireChirho => Box::new(SapphireCipherChirho::new_chirho(key_chirho)),
        CipherTypeChirho::XorChirho => Box::new(XorCipherChirho::new_chirho(key_chirho)),
    }
}

/// Null cipher (no encryption).
#[derive(Debug, Clone, Default)]
pub struct NullCipherChirho;

impl CipherChirho for NullCipherChirho {
    fn encrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        Ok(data_chirho.to_vec())
    }

    fn decrypt_chirho(&self, data_chirho: &[u8]) -> ResultChirho<Vec<u8>> {
        Ok(data_chirho.to_vec())
    }

    fn name_chirho(&self) -> &str {
        "None"
    }

    fn has_key_chirho(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_null_cipher_chirho() {
        let cipher_chirho = NullCipherChirho;
        let data_chirho = b"Hello, World!";

        let encrypted_chirho = cipher_chirho.encrypt_chirho(data_chirho).unwrap();
        assert_eq!(encrypted_chirho, data_chirho);

        let decrypted_chirho = cipher_chirho.decrypt_chirho(&encrypted_chirho).unwrap();
        assert_eq!(decrypted_chirho, data_chirho);
    }

    #[test]
    fn test_create_cipher_chirho() {
        let cipher_chirho = create_cipher_chirho(CipherTypeChirho::NoneChirho, "");
        assert_eq!(cipher_chirho.name_chirho(), "None");

        let cipher_chirho = create_cipher_chirho(CipherTypeChirho::SapphireChirho, "test");
        assert_eq!(cipher_chirho.name_chirho(), "Sapphire");

        let cipher_chirho = create_cipher_chirho(CipherTypeChirho::XorChirho, "test");
        assert_eq!(cipher_chirho.name_chirho(), "XOR");
    }
}
