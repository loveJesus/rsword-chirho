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

/// Simple URL-based file downloader using the system curl/wget if available.
pub struct SystemTransportChirho;

impl SystemTransportChirho {
    /// Create a new system transport.
    pub fn new_chirho() -> Self {
        Self
    }
}

impl TransportChirho for SystemTransportChirho {
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        // Create parent directories
        if let Some(parent_chirho) = dest_chirho.parent() {
            std::fs::create_dir_all(parent_chirho)?;
        }

        // Try curl first, then wget
        let dest_str_chirho = dest_chirho.to_string_lossy();

        let status_chirho = std::process::Command::new("curl")
            .args(["-fsSL", "-o", &dest_str_chirho, url_chirho])
            .status();

        if let Ok(status_chirho) = status_chirho {
            if status_chirho.success() {
                return Ok(());
            }
        }

        // Try wget
        let status_chirho = std::process::Command::new("wget")
            .args(["-q", "-O", &dest_str_chirho, url_chirho])
            .status();

        if let Ok(status_chirho) = status_chirho {
            if status_chirho.success() {
                return Ok(());
            }
        }

        Err(ErrorChirho::network_chirho("Neither curl nor wget available for download"))
    }

    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        // Try curl first
        let output_chirho = std::process::Command::new("curl")
            .args(["-fsSL", url_chirho])
            .output();

        if let Ok(output_chirho) = output_chirho {
            if output_chirho.status.success() {
                return Ok(output_chirho.stdout);
            }
        }

        // Try wget
        let output_chirho = std::process::Command::new("wget")
            .args(["-q", "-O", "-", url_chirho])
            .output();

        if let Ok(output_chirho) = output_chirho {
            if output_chirho.status.success() {
                return Ok(output_chirho.stdout);
            }
        }

        Err(ErrorChirho::network_chirho("Neither curl nor wget available for fetch"))
    }

    fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        std::process::Command::new("curl")
            .args(["-fsSI", url_chirho])
            .status()
            .map(|s_chirho| s_chirho.success())
            .unwrap_or(false)
    }
}

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
