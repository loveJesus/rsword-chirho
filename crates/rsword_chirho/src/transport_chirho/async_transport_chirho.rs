// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Async HTTP transport for module downloads.
//!
//! Provides asynchronous HTTP transport using tokio and reqwest for
//! efficient downloading of SWORD modules with progress reporting.
//!
//! # Features
//!
//! - Async/await based API using tokio runtime
//! - Progress callbacks during downloads
//! - Concurrent downloads of multiple files
//! - Download cancellation support
//! - Retry on transient failures
//!
//! # Usage
//!
//! ```rust,ignore
//! use rsword_chirho::transport_chirho::{AsyncTransportChirho, DownloadProgressChirho};
//! use std::path::Path;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let transport_chirho = AsyncTransportChirho::new_chirho()?;
//!
//!     // Download with progress callback
//!     let progress_chirho = Arc::new(|progress_chirho: DownloadProgressChirho| {
//!         println!("Downloaded {}%", progress_chirho.percent_chirho());
//!     });
//!
//!     transport_chirho.download_with_progress_chirho(
//!         "https://example.com/module.zip",
//!         Path::new("./module.zip"),
//!         Some(progress_chirho),
//!     ).await?;
//!
//!     Ok(())
//! }
//! ```

use std::path::Path;
use std::sync::Arc;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::watch;

use crate::error_chirho::{ErrorChirho, ResultChirho};

/// Download progress information.
#[derive(Debug, Clone)]
pub struct DownloadProgressChirho {
    /// URL being downloaded.
    pub url_chirho: String,
    /// Bytes downloaded so far.
    pub downloaded_chirho: u64,
    /// Total bytes (if known).
    pub total_chirho: Option<u64>,
}

impl DownloadProgressChirho {
    /// Get download percentage (0-100).
    pub fn percent_chirho(&self) -> u8 {
        match self.total_chirho {
            Some(total_chirho) if total_chirho > 0 => {
                ((self.downloaded_chirho as f64 / total_chirho as f64) * 100.0) as u8
            }
            _ => 0,
        }
    }

    /// Check if download is complete.
    pub fn is_complete_chirho(&self) -> bool {
        match self.total_chirho {
            Some(total_chirho) => self.downloaded_chirho >= total_chirho,
            None => false,
        }
    }
}

/// Progress callback type.
pub type ProgressCallbackChirho = Arc<dyn Fn(DownloadProgressChirho) + Send + Sync>;

/// Cancellation token for downloads.
#[derive(Clone)]
pub struct CancellationTokenChirho {
    /// Sender to signal cancellation.
    sender_chirho: watch::Sender<bool>,
    /// Receiver to check if cancelled.
    receiver_chirho: watch::Receiver<bool>,
}

impl CancellationTokenChirho {
    /// Create a new cancellation token.
    pub fn new_chirho() -> Self {
        let (sender_chirho, receiver_chirho) = watch::channel(false);
        Self {
            sender_chirho,
            receiver_chirho,
        }
    }

    /// Cancel the operation.
    pub fn cancel_chirho(&self) {
        let _ = self.sender_chirho.send(true);
    }

    /// Check if cancelled.
    pub fn is_cancelled_chirho(&self) -> bool {
        *self.receiver_chirho.borrow()
    }

    /// Clone the receiver for checking cancellation.
    pub fn subscribe_chirho(&self) -> watch::Receiver<bool> {
        self.receiver_chirho.clone()
    }
}

impl Default for CancellationTokenChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Async HTTP transport using reqwest.
pub struct AsyncTransportChirho {
    /// HTTP client.
    client_chirho: reqwest::Client,
    /// Default timeout in seconds.
    timeout_chirho: u64,
    /// Number of retries on transient failures.
    retries_chirho: u32,
}

impl AsyncTransportChirho {
    /// Create a new async transport.
    pub fn new_chirho() -> ResultChirho<Self> {
        let client_chirho = reqwest::Client::builder()
            .user_agent("rsword-chirho/0.1 (SWORD module library)")
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to create HTTP client: {}", e_chirho)))?;

        Ok(Self {
            client_chirho,
            timeout_chirho: 300,
            retries_chirho: 3,
        })
    }

    /// Create with custom configuration.
    pub fn with_config_chirho(timeout_chirho: u64, retries_chirho: u32) -> ResultChirho<Self> {
        let client_chirho = reqwest::Client::builder()
            .user_agent("rsword-chirho/0.1 (SWORD module library)")
            .timeout(std::time::Duration::from_secs(timeout_chirho))
            .build()
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to create HTTP client: {}", e_chirho)))?;

        Ok(Self {
            client_chirho,
            timeout_chirho,
            retries_chirho,
        })
    }

    /// Download a file asynchronously.
    pub async fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        self.download_with_progress_chirho(url_chirho, dest_chirho, None, None).await
    }

    /// Download a file with progress callback.
    pub async fn download_with_progress_chirho(
        &self,
        url_chirho: &str,
        dest_chirho: &Path,
        progress_chirho: Option<ProgressCallbackChirho>,
        cancel_token_chirho: Option<CancellationTokenChirho>,
    ) -> ResultChirho<()> {
        // Create parent directories
        if let Some(parent_chirho) = dest_chirho.parent() {
            tokio::fs::create_dir_all(parent_chirho).await?;
        }

        let mut last_error_chirho: Option<ErrorChirho> = None;

        for attempt_chirho in 0..=self.retries_chirho {
            if let Some(ref token_chirho) = cancel_token_chirho {
                if token_chirho.is_cancelled_chirho() {
                    return Err(ErrorChirho::network_chirho("Download cancelled"));
                }
            }

            match self.download_attempt_chirho(url_chirho, dest_chirho, &progress_chirho, &cancel_token_chirho).await {
                Ok(()) => return Ok(()),
                Err(e_chirho) => {
                    last_error_chirho = Some(e_chirho);
                    if attempt_chirho < self.retries_chirho {
                        // Exponential backoff
                        let delay_chirho = std::time::Duration::from_millis(100 * 2u64.pow(attempt_chirho));
                        tokio::time::sleep(delay_chirho).await;
                    }
                }
            }
        }

        Err(last_error_chirho.unwrap_or_else(|| ErrorChirho::network_chirho("Download failed")))
    }

    /// Single download attempt.
    async fn download_attempt_chirho(
        &self,
        url_chirho: &str,
        dest_chirho: &Path,
        progress_chirho: &Option<ProgressCallbackChirho>,
        cancel_token_chirho: &Option<CancellationTokenChirho>,
    ) -> ResultChirho<()> {
        use futures_util::StreamExt;

        let response_chirho = self.client_chirho.get(url_chirho)
            .send()
            .await
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Request failed: {}", e_chirho)))?;

        if !response_chirho.status().is_success() {
            return Err(ErrorChirho::network_chirho(format!(
                "HTTP error: {}",
                response_chirho.status()
            )));
        }

        let total_chirho = response_chirho.content_length();
        let mut downloaded_chirho: u64 = 0;

        let mut file_chirho = File::create(dest_chirho).await?;
        let mut stream_chirho = response_chirho.bytes_stream();

        while let Some(chunk_result_chirho) = stream_chirho.next().await {
            // Check for cancellation
            if let Some(ref token_chirho) = cancel_token_chirho {
                if token_chirho.is_cancelled_chirho() {
                    // Clean up partial file
                    let _ = tokio::fs::remove_file(dest_chirho).await;
                    return Err(ErrorChirho::network_chirho("Download cancelled"));
                }
            }

            let chunk_chirho = chunk_result_chirho
                .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Stream error: {}", e_chirho)))?;

            file_chirho.write_all(&chunk_chirho).await?;
            downloaded_chirho += chunk_chirho.len() as u64;

            // Report progress
            if let Some(ref callback_chirho) = progress_chirho {
                callback_chirho(DownloadProgressChirho {
                    url_chirho: url_chirho.to_string(),
                    downloaded_chirho,
                    total_chirho,
                });
            }
        }

        file_chirho.flush().await?;
        Ok(())
    }

    /// Fetch content from a URL.
    pub async fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        let response_chirho = self.client_chirho.get(url_chirho)
            .send()
            .await
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Fetch failed: {}", e_chirho)))?;

        if !response_chirho.status().is_success() {
            return Err(ErrorChirho::network_chirho(format!(
                "HTTP error: {}",
                response_chirho.status()
            )));
        }

        response_chirho.bytes()
            .await
            .map(|b_chirho| b_chirho.to_vec())
            .map_err(|e_chirho| ErrorChirho::network_chirho(format!("Failed to read response: {}", e_chirho)))
    }

    /// Fetch content as string.
    pub async fn fetch_text_chirho(&self, url_chirho: &str) -> ResultChirho<String> {
        let bytes_chirho = self.fetch_chirho(url_chirho).await?;
        String::from_utf8(bytes_chirho)
            .map_err(|e_chirho| ErrorChirho::generic_chirho(format!("Invalid UTF-8: {}", e_chirho)))
    }

    /// Check if a URL is accessible.
    pub async fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        self.client_chirho.head(url_chirho)
            .send()
            .await
            .map(|r_chirho| r_chirho.status().is_success())
            .unwrap_or(false)
    }

    /// Download multiple files concurrently.
    pub async fn download_multiple_chirho(
        &self,
        downloads_chirho: Vec<(String, std::path::PathBuf)>,
        progress_chirho: Option<ProgressCallbackChirho>,
        cancel_token_chirho: Option<CancellationTokenChirho>,
    ) -> Vec<ResultChirho<()>> {
        use futures_util::future::join_all;

        let futures_chirho: Vec<_> = downloads_chirho
            .into_iter()
            .map(|(url_chirho, path_chirho)| {
                let client_chirho = self.client_chirho.clone();
                let progress_chirho = progress_chirho.clone();
                let cancel_token_chirho = cancel_token_chirho.clone();
                let timeout_chirho = self.timeout_chirho;
                let retries_chirho = self.retries_chirho;

                async move {
                    let transport_chirho = Self {
                        client_chirho,
                        timeout_chirho,
                        retries_chirho,
                    };
                    transport_chirho.download_with_progress_chirho(
                        &url_chirho,
                        &path_chirho,
                        progress_chirho,
                        cancel_token_chirho,
                    ).await
                }
            })
            .collect();

        join_all(futures_chirho).await
    }
}

/// Async transport trait for extensibility.
#[async_trait::async_trait]
pub trait AsyncTransportTraitChirho: Send + Sync {
    /// Download a file asynchronously.
    async fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()>;

    /// Fetch content from a URL.
    async fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>>;

    /// Check if a URL is accessible.
    async fn is_accessible_chirho(&self, url_chirho: &str) -> bool;
}

#[async_trait::async_trait]
impl AsyncTransportTraitChirho for AsyncTransportChirho {
    async fn download_chirho(&self, url_chirho: &str, dest_chirho: &Path) -> ResultChirho<()> {
        AsyncTransportChirho::download_chirho(self, url_chirho, dest_chirho).await
    }

    async fn fetch_chirho(&self, url_chirho: &str) -> ResultChirho<Vec<u8>> {
        AsyncTransportChirho::fetch_chirho(self, url_chirho).await
    }

    async fn is_accessible_chirho(&self, url_chirho: &str) -> bool {
        AsyncTransportChirho::is_accessible_chirho(self, url_chirho).await
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_download_progress_chirho() {
        let progress_chirho = DownloadProgressChirho {
            url_chirho: "https://example.com/file".to_string(),
            downloaded_chirho: 50,
            total_chirho: Some(100),
        };

        assert_eq!(progress_chirho.percent_chirho(), 50);
        assert!(!progress_chirho.is_complete_chirho());

        let complete_chirho = DownloadProgressChirho {
            url_chirho: "https://example.com/file".to_string(),
            downloaded_chirho: 100,
            total_chirho: Some(100),
        };

        assert_eq!(complete_chirho.percent_chirho(), 100);
        assert!(complete_chirho.is_complete_chirho());
    }

    #[test]
    fn test_cancellation_token_chirho() {
        let token_chirho = CancellationTokenChirho::new_chirho();
        assert!(!token_chirho.is_cancelled_chirho());

        token_chirho.cancel_chirho();
        assert!(token_chirho.is_cancelled_chirho());
    }

    #[tokio::test]
    async fn test_async_transport_creation_chirho() {
        let transport_chirho = AsyncTransportChirho::new_chirho();
        assert!(transport_chirho.is_ok());
    }

    #[tokio::test]
    async fn test_async_transport_with_config_chirho() {
        let transport_chirho = AsyncTransportChirho::with_config_chirho(60, 5);
        assert!(transport_chirho.is_ok());
    }
}
