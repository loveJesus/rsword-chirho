// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Cipher filter for decrypting encrypted module content.
//!
//! This filter integrates with the cipher module to decrypt content
//! from encrypted SWORD modules.

use crate::cipher_chirho::{CipherChirho, CipherTypeChirho, create_cipher_chirho};
use crate::error_chirho::ResultChirho;
use crate::filters_chirho::FilterChirho;

/// Filter that decrypts content using a SWORD cipher.
#[derive(Debug)]
pub struct CipherFilterChirho {
    /// The cipher to use for decryption.
    cipher_chirho: Box<dyn CipherChirho>,
}

impl CipherFilterChirho {
    /// Create a new cipher filter with the given cipher type and key.
    pub fn new_chirho(cipher_type_chirho: CipherTypeChirho, key_chirho: &str) -> Self {
        Self {
            cipher_chirho: create_cipher_chirho(cipher_type_chirho, key_chirho),
        }
    }

    /// Create a cipher filter from a cipher instance.
    pub fn from_cipher_chirho(cipher_chirho: Box<dyn CipherChirho>) -> Self {
        Self { cipher_chirho }
    }

    /// Check if the filter has a valid key set.
    pub fn has_key_chirho(&self) -> bool {
        self.cipher_chirho.has_key_chirho()
    }

    /// Get the cipher name.
    pub fn cipher_name_chirho(&self) -> &str {
        self.cipher_chirho.name_chirho()
    }
}

impl FilterChirho for CipherFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        // If no key is set, return as-is
        if !self.cipher_chirho.has_key_chirho() {
            return Ok(text_chirho.to_string());
        }

        // Decrypt the bytes
        match self.cipher_chirho.decrypt_chirho(text_chirho.as_bytes()) {
            Ok(decrypted_chirho) => {
                // Try to convert to UTF-8, fall back to lossy conversion
                Ok(String::from_utf8(decrypted_chirho)
                    .unwrap_or_else(|e| String::from_utf8_lossy(e.as_bytes()).into_owned()))
            }
            Err(_) => Ok(text_chirho.to_string()),
        }
    }

    fn name_chirho(&self) -> &str {
        "CipherFilter"
    }
}

/// Cipher filter that works on raw bytes (for binary content).
#[derive(Debug)]
pub struct CipherBytesFilterChirho {
    /// The cipher to use for decryption.
    cipher_chirho: Box<dyn CipherChirho>,
}

impl CipherBytesFilterChirho {
    /// Create a new cipher bytes filter with the given cipher type and key.
    pub fn new_chirho(cipher_type_chirho: CipherTypeChirho, key_chirho: &str) -> Self {
        Self {
            cipher_chirho: create_cipher_chirho(cipher_type_chirho, key_chirho),
        }
    }

    /// Decrypt raw bytes.
    pub fn decrypt_bytes_chirho(&self, data_chirho: &[u8]) -> Vec<u8> {
        if !self.cipher_chirho.has_key_chirho() {
            return data_chirho.to_vec();
        }

        self.cipher_chirho.decrypt_chirho(data_chirho)
            .unwrap_or_else(|_| data_chirho.to_vec())
    }

    /// Encrypt raw bytes.
    pub fn encrypt_bytes_chirho(&self, data_chirho: &[u8]) -> Vec<u8> {
        if !self.cipher_chirho.has_key_chirho() {
            return data_chirho.to_vec();
        }

        self.cipher_chirho.encrypt_chirho(data_chirho)
            .unwrap_or_else(|_| data_chirho.to_vec())
    }

    /// Check if the filter has a valid key set.
    pub fn has_key_chirho(&self) -> bool {
        self.cipher_chirho.has_key_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_cipher_filter_basic_chirho() {
        let filter_chirho = CipherFilterChirho::new_chirho(CipherTypeChirho::XorChirho, "key");
        assert!(filter_chirho.has_key_chirho());
        assert_eq!(filter_chirho.cipher_name_chirho(), "XOR");
    }

    #[test]
    fn test_cipher_filter_no_key_chirho() {
        let filter_chirho = CipherFilterChirho::new_chirho(CipherTypeChirho::SapphireChirho, "");
        assert!(!filter_chirho.has_key_chirho());

        let result_chirho = filter_chirho.process_chirho("test").unwrap();
        assert_eq!(result_chirho, "test");
    }

    #[test]
    fn test_cipher_bytes_filter_chirho() {
        let filter_chirho = CipherBytesFilterChirho::new_chirho(CipherTypeChirho::SapphireChirho, "test_key");
        let plaintext_chirho = b"For God so loved the world";

        let encrypted_chirho = filter_chirho.encrypt_bytes_chirho(plaintext_chirho);
        assert_ne!(encrypted_chirho, plaintext_chirho);

        let decrypted_chirho = filter_chirho.decrypt_bytes_chirho(&encrypted_chirho);
        assert_eq!(decrypted_chirho, plaintext_chirho);
    }

    #[test]
    fn test_cipher_filter_name_chirho() {
        let filter_chirho = CipherFilterChirho::new_chirho(CipherTypeChirho::NoneChirho, "");
        assert_eq!(filter_chirho.name_chirho(), "CipherFilter");
    }
}
