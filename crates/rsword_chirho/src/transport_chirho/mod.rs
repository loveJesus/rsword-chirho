// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Network transport for module downloads.

use std::path::Path;

use crate::error_chirho::ResultChirho;

/// Transport trait for downloading files.
pub trait TransportChirho {
    /// Download a file from a URL to a local path.
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()>;

    /// Fetch content from a URL.
    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>>;

    /// Check if a URL is accessible.
    fn is_accessible_chirho(&self, url_chirho: &str) -> bool;
}

/// HTTP transport using reqwest.
pub struct HttpTransportChirho;

impl HttpTransportChirho {
    /// Create a new HTTP transport.
    pub fn new_chirho() -> Self {
        Self
    }
}

impl TransportChirho for HttpTransportChirho {
    fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        // TODO: Implement using reqwest
        Err(crate::error_chirho::ErrorChirho::network_chirho("HTTP transport not implemented"))
    }

    fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        // TODO: Implement using reqwest
        Err(crate::error_chirho::ErrorChirho::network_chirho("HTTP transport not implemented"))
    }

    fn is_accessible_chirho(&self, _url_chirho: &str) -> bool {
        false
    }
}

// TODO: Implement FTP transport if needed

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_http_transport_new_chirho() {
        let _transport_chirho = HttpTransportChirho::new_chirho();
    }
}
