// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Network transport for module downloads.
//!
//! Provides HTTP and FTP transport for downloading SWORD modules
//! from remote repositories.

use std::path::Path;

#[cfg(feature = "http-transport")]
use std::fs::File;
#[cfg(feature = "http-transport")]
use std::io::Write;

use crate::error_chirho::{ErrorChirho, ResultChirho};

/// Transport trait for downloading files.
pub trait TransportChirho {
    /// Download a file from a URL to a local path.
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()>;

    /// Fetch content from a URL.
    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>>;

    /// Fetch content as string.
    fn fetch_text_chirho(&self, url_chirho: &str) -> ResultChirho<String> {
        let bytes_chirho = self.fetch_chirho(url_chirho)?;
        String::from_utf8(bytes_chirho)
            .map_err(|e_chirho| ErrorChirho::generic_chirho(format!("Invalid UTF-8: {}", e_chirho)))
    }

    /// Check if a URL is accessible.
    fn is_accessible_chirho(&self, url_chirho: &str) -> bool;
}

/// HTTP transport using reqwest (blocking).
#[cfg(feature = "http-transport")]
pub struct HttpTransportChirho {
    client_chirho: reqwest::blocking::Client,
}

#[cfg(feature = "http-transport")]
impl HttpTransportChirho {
    /// Create a new HTTP transport.
    pub fn new_chirho() -> ResultChirho<Self> {
        let client_chirho = reqwest::blocking::Client::builder()
            .user_agent("rsword-chirho/0.1 (SWORD module library)")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to create HTTP client: {}", e_chirho)))?;

        Ok(Self { client_chirho })
    }
}

#[cfg(feature = "http-transport")]
impl TransportChirho for HttpTransportChirho {
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        let response_chirho = self.client_chirho.get(url_chirho)
            .send()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Download failed: {}", e_chirho)))?;

        if !response_chirho.status().is_success() {
            return Err(ErrorChirho::network_chirho(format!(
                "HTTP error: {}",
                response_chirho.status()
            )));
        }

        let bytes_chirho = response_chirho.bytes()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))?;

        // Create parent directories if needed
        if let Some(parent_chirho) = dest_chirho.parent() {
            std::fs::create_dir_all(parent_chirho)?;
        }

        let mut file_chirho = File::create(dest_chirho)?;
        file_chirho.write_all(&bytes_chirho)?;

        Ok(())
    }

    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        let response_chirho = self.client_chirho.get(url_chirho)
            .send()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Fetch failed: {}", e_chirho)))?;

        if !response_chirho.status().is_success() {
            return Err(ErrorChirho::network_chirho(format!(
                "HTTP error: {}",
                response_chirho.status()
            )));
        }

        response_chirho.bytes()
            .map(|b_chirho| b_chirho.to_vec())
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))
    }

    fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        self.client_chirho.head(url_chirho)
            .send()
            .map(|r_chirho| r_chirho.status().is_success())
            .unwrap_or(false)
    }
}

/// HTTP transport stub (when http-transport feature is disabled).
#[cfg(not(feature = "http-transport"))]
pub struct HttpTransportChirho;

#[cfg(not(feature = "http-transport"))]
impl HttpTransportChirho {
    /// Create a new HTTP transport.
    pub fn new_chirho() -> Self {
        Self
    }
}

#[cfg(not(feature = "http-transport"))]
impl TransportChirho for HttpTransportChirho {
    fn download_chirho(&self, _url_chirho: &str, _dest_chirho: &Path) -> ResultChirho<()> {
        Err(ErrorChirho::network_chirho("HTTP transport not compiled (enable http-transport feature)"))
    }

    fn fetch_chirho(&self, _url_chirho: &str) -> ResultChirho<Vec<u8>> {
        Err(ErrorChirho::network_chirho("HTTP transport not compiled (enable http-transport feature)"))
    }

    fn is_accessible_chirho(&self, _url_chirho: &str) -> bool {
        false
    }
}

/// Lightweight HTTP transport using ureq (no async runtime needed).
///
/// This provides a pure Rust HTTP client without requiring external tools
/// like curl or wget. It's simpler and more portable than reqwest.
pub struct UreqTransportChirho {
    agent_chirho: ureq::Agent,
}

impl UreqTransportChirho {
    /// Create a new ureq transport.
    pub fn new_chirho() -> Self {
        let agent_chirho = ureq::Agent::new_with_defaults();
        Self { agent_chirho }
    }
}

impl TransportChirho for UreqTransportChirho {
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        // Create parent directories
        if let Some(parent_chirho) = dest_chirho.parent() {
            std::fs::create_dir_all(parent_chirho)?;
        }

        let response_chirho = self.agent_chirho.get(url_chirho)
            .call()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("HTTP request failed: {}", e_chirho)))?;

        let status_chirho = response_chirho.status().as_u16();
        if status_chirho < 200 || status_chirho >= 300 {
            return Err(ErrorChirho::network_chirho(format!("HTTP error: {}", status_chirho)));
        }

        let bytes_chirho = response_chirho.into_body()
            .read_to_vec()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))?;

        std::fs::write(dest_chirho, &bytes_chirho)?;
        Ok(())
    }

    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        let response_chirho = self.agent_chirho.get(url_chirho)
            .call()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("HTTP request failed: {}", e_chirho)))?;

        let status_chirho = response_chirho.status().as_u16();
        if status_chirho < 200 || status_chirho >= 300 {
            return Err(ErrorChirho::network_chirho(format!("HTTP error: {}", status_chirho)));
        }

        let bytes_chirho = response_chirho.into_body()
            .read_to_vec()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))?;

        Ok(bytes_chirho)
    }

    fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        self.agent_chirho.head(url_chirho)
            .call()
            .map(|r_chirho| {
                let status_chirho = r_chirho.status().as_u16();
                status_chirho >= 200 && status_chirho < 300
            })
            .unwrap_or(false)
    }
}

/// Type alias for backwards compatibility.
pub type SystemTransportChirho = UreqTransportChirho;

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_system_transport_new_chirho() {
        let _transport_chirho = SystemTransportChirho::new_chirho();
    }

    #[cfg(not(feature = "http-transport"))]
    #[test]
    fn test_http_transport_stub_chirho() {
        let transport_chirho = HttpTransportChirho::new_chirho();
        assert!(!transport_chirho.is_accessible_chirho("https://example.com"));
    }
}
