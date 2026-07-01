// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Network transport for module downloads.
//!
//! Provides HTTP and FTP transport for downloading SWORD modules
//! from remote repositories.
//!
//! ## Transport Options
//!
//! | Transport | Feature | Description |
//! |-----------|---------|-------------|
//! | [`UreqTransportChirho`] | default | Lightweight sync HTTP (pure Rust) |
//! | [`HttpTransportChirho`] | `http-transport` | Blocking HTTP with reqwest |
//! | [`AsyncTransportChirho`] | `async-transport` | Async HTTP with tokio/reqwest |
//!
//! ## Async Transport
//!
//! Enable the `async-transport` feature for async downloads with progress:
//!
//! ```rust,ignore
//! use rsword_chirho::transport_chirho::{AsyncTransportChirho, DownloadProgressChirho};
//!
//! #[tokio::main]
//! async fn main() {
//!     let transport_chirho = AsyncTransportChirho::new_chirho().unwrap();
//!     transport_chirho.download_chirho("https://example.com/mod.zip", &path).await.unwrap();
//! }
//! ```

#[cfg(feature = "async-transport")]
pub mod async_transport_chirho;

#[cfg(feature = "async-transport")]
pub use async_transport_chirho::{
    AsyncTransportChirho, AsyncTransportTraitChirho,
    DownloadProgressChirho, ProgressCallbackChirho, CancellationTokenChirho,
};

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

/// Upper bound for an in-memory [`TransportChirho::fetch_chirho`] body.
///
/// `fetch_chirho` buffers the whole response in RAM, so it is used for small
/// metadata (remote catalogs, `.conf` files) — never module archives, which
/// stream to disk in [`UreqTransportChirho::download_chirho`]. ureq's
/// `read_to_vec()` defaults to a 10 MB cap; we raise it well past any real
/// catalog while still bounding memory so a hostile/misbehaving server cannot
/// exhaust the client's RAM.
#[cfg(feature = "native")]
const MAX_FETCH_BYTES_CHIRHO: u64 = 64 * 1024 * 1024;

/// Lightweight HTTP transport using ureq (no async runtime needed).
///
/// This provides a pure Rust HTTP client without requiring external tools
/// like curl or wget. It's simpler and more portable than reqwest.
///
/// Only available on native platforms (requires the `native` feature).
#[cfg(feature = "native")]
pub struct UreqTransportChirho {
    agent_chirho: ureq::Agent,
}

#[cfg(feature = "native")]
impl UreqTransportChirho {
    /// Create a new ureq transport.
    pub fn new_chirho() -> Self {
        let agent_chirho = ureq::Agent::new_with_defaults();
        Self { agent_chirho }
    }
}

#[cfg(feature = "native")]
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
        if !(200..300).contains(&status_chirho) {
            return Err(ErrorChirho::network_chirho(format!("HTTP error: {}", status_chirho)));
        }

        // Stream the body straight to disk. ureq's convenience `read_to_vec()`
        // caps the response at 10 MB (`MAX_BODY_SIZE`), which silently broke any
        // module larger than that (e.g. MHC ~30 MB → "the response body is larger
        // than request limit: 10485760"). `into_reader()` is unbounded, and
        // copying it in fixed-size chunks keeps peak memory flat regardless of
        // module size — important on mobile where a module may be hundreds of MB.
        let mut reader_chirho = response_chirho.into_body().into_reader();
        let mut file_chirho = std::fs::File::create(dest_chirho)?;
        std::io::copy(&mut reader_chirho, &mut file_chirho)
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))?;

        Ok(())
    }

    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        let response_chirho = self.agent_chirho.get(url_chirho)
            .call()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("HTTP request failed: {}", e_chirho)))?;

        let status_chirho = response_chirho.status().as_u16();
        if !(200..300).contains(&status_chirho) {
            return Err(ErrorChirho::network_chirho(format!("HTTP error: {}", status_chirho)));
        }

        // Metadata fetch buffers in RAM; raise ureq's default 10 MB `read_to_vec()`
        // cap to `MAX_FETCH_BYTES_CHIRHO` so a large catalog still loads while
        // memory stays bounded. (Module archives download via `download_chirho`.)
        let mut body_chirho = response_chirho.into_body();
        let bytes_chirho = body_chirho
            .with_config()
            .limit(MAX_FETCH_BYTES_CHIRHO)
            .read_to_vec()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))?;

        Ok(bytes_chirho)
    }

    fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        self.agent_chirho.head(url_chirho)
            .call()
            .map(|r_chirho| {
                let status_chirho = r_chirho.status().as_u16();
                (200..300).contains(&status_chirho)
            })
            .unwrap_or(false)
    }
}

/// Type alias for backwards compatibility (native only).
#[cfg(feature = "native")]
pub type SystemTransportChirho = UreqTransportChirho;

/// Stub transport for WASM (no network support).
#[cfg(not(feature = "native"))]
pub struct StubTransportChirho;

#[cfg(not(feature = "native"))]
impl TransportChirho for StubTransportChirho {
    fn download_chirho(&self, _url_chirho: &str, _dest_chirho: &Path) -> ResultChirho<()> {
        Err(ErrorChirho::network_chirho("Network transport not available in WASM"))
    }

    fn fetch_chirho(&self, _url_chirho: &str) -> ResultChirho<Vec<u8>> {
        Err(ErrorChirho::network_chirho("Network transport not available in WASM"))
    }

    fn is_accessible_chirho(&self, _url_chirho: &str) -> bool {
        false
    }
}

/// Type alias for backwards compatibility (WASM stub).
#[cfg(not(feature = "native"))]
pub type SystemTransportChirho = StubTransportChirho;

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    #[cfg(feature = "native")]
    fn test_system_transport_new_chirho() {
        let _transport_chirho = SystemTransportChirho::new_chirho();
    }

    #[cfg(all(feature = "native", not(feature = "http-transport")))]
    #[test]
    fn test_http_transport_stub_chirho() {
        let transport_chirho = HttpTransportChirho::new_chirho();
        assert!(!transport_chirho.is_accessible_chirho("https://example.com"));
    }
}
