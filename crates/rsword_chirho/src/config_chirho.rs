// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Configuration file parser for SWORD modules.
//!
//! SWORD module configuration files use an INI-like format with sections
//! and key-value pairs. This module provides parsing and writing support.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

use crate::error_chirho::{ErrorChirho, ResultChirho};

/// A SWORD module configuration.
///
/// Configuration files have a main section (the module name) and key-value pairs.
/// Multi-line values are supported using continuation lines starting with whitespace.
#[derive(Debug, Clone, Default)]
pub struct ModuleConfigChirho {
    /// The module name (section header in the .conf file).
    pub name_chirho: String,
    /// Configuration entries (key-value pairs).
    pub entries_chirho: HashMap<String, String>,
}

impl ModuleConfigChirho {
    /// Create a new empty configuration.
    pub fn new_chirho(name_chirho: String) -> Self {
        Self {
            name_chirho,
            entries_chirho: HashMap::new(),
        }
    }

    /// Parse a configuration from a file.
    pub fn from_file_chirho<P: AsRef<Path>>(path_chirho: P) -> ResultChirho<Self> {
        let file_chirho = File::open(path_chirho)?;
        let reader_chirho = BufReader::new(file_chirho);
        Self::from_reader_chirho(reader_chirho)
    }

    /// Parse a configuration from a string.
    pub fn from_str_chirho(content_chirho: &str) -> ResultChirho<Self> {
        Self::from_reader_chirho(content_chirho.as_bytes())
    }

    /// Parse a configuration from a reader.
    pub fn from_reader_chirho<R: BufRead>(reader_chirho: R) -> ResultChirho<Self> {
        let mut config_chirho = Self::default();
        let mut current_key_chirho: Option<String> = None;
        let mut current_value_chirho = String::new();

        for line_result_chirho in reader_chirho.lines() {
            let line_chirho = line_result_chirho?;

            // Check for continuation line (starts with whitespace)
            if !line_chirho.is_empty() && line_chirho.starts_with(|c: char| c.is_whitespace()) {
                if current_key_chirho.is_some() {
                    // Append to current value, preserving the newline
                    current_value_chirho.push('\n');
                    current_value_chirho.push_str(line_chirho.trim());
                }
                continue;
            }

            // Save any pending key-value pair
            if let Some(key_chirho) = current_key_chirho.take() {
                config_chirho.entries_chirho.insert(key_chirho, current_value_chirho.clone());
                current_value_chirho.clear();
            }

            // Skip empty lines and comments
            let trimmed_chirho = line_chirho.trim();
            if trimmed_chirho.is_empty() || trimmed_chirho.starts_with('#') || trimmed_chirho.starts_with(';') {
                continue;
            }

            // Check for section header [ModuleName]
            if trimmed_chirho.starts_with('[') && trimmed_chirho.ends_with(']') {
                config_chirho.name_chirho = trimmed_chirho[1..trimmed_chirho.len()-1].to_string();
                continue;
            }

            // Parse key=value
            if let Some(eq_pos_chirho) = trimmed_chirho.find('=') {
                let key_chirho = trimmed_chirho[..eq_pos_chirho].trim().to_string();
                let value_chirho = trimmed_chirho[eq_pos_chirho + 1..].trim().to_string();
                current_key_chirho = Some(key_chirho);
                current_value_chirho = value_chirho;
            }
        }

        // Save final key-value pair if any
        if let Some(key_chirho) = current_key_chirho {
            config_chirho.entries_chirho.insert(key_chirho, current_value_chirho);
        }

        Ok(config_chirho)
    }

    /// Get a configuration value.
    pub fn get_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.entries_chirho.get(key_chirho).map(|s| s.as_str())
    }

    /// Get a configuration value, returning an error if not found.
    pub fn get_required_chirho(&self, key_chirho: &str) -> ResultChirho<&str> {
        self.get_chirho(key_chirho)
            .ok_or_else(|| ErrorChirho::missing_config_key_chirho(key_chirho))
    }

    /// Get a configuration value with a default.
    pub fn get_or_chirho<'a>(&'a self, key_chirho: &str, default_chirho: &'a str) -> &'a str {
        self.get_chirho(key_chirho).unwrap_or(default_chirho)
    }

    /// Set a configuration value.
    pub fn set_chirho(&mut self, key_chirho: impl Into<String>, value_chirho: impl Into<String>) {
        self.entries_chirho.insert(key_chirho.into(), value_chirho.into());
    }

    /// Remove a configuration value.
    pub fn remove_chirho(&mut self, key_chirho: &str) -> Option<String> {
        self.entries_chirho.remove(key_chirho)
    }

    /// Check if a key exists.
    pub fn contains_key_chirho(&self, key_chirho: &str) -> bool {
        self.entries_chirho.contains_key(key_chirho)
    }

    /// Get the data path for this module.
    pub fn data_path_chirho(&self) -> Option<&str> {
        self.get_chirho("DataPath")
    }

    /// Get the module type (e.g., "RawText", "zText", "RawCom", etc.).
    pub fn module_driver_chirho(&self) -> Option<&str> {
        self.get_chirho("ModDrv")
    }

    /// Get the versification system.
    pub fn versification_chirho(&self) -> &str {
        self.get_or_chirho("Versification", "KJV")
    }

    /// Get the source markup format.
    pub fn source_type_chirho(&self) -> Option<&str> {
        self.get_chirho("SourceType")
    }

    /// Get the text encoding.
    pub fn encoding_chirho(&self) -> &str {
        self.get_or_chirho("Encoding", "UTF-8")
    }

    /// Get the compression type.
    pub fn compress_type_chirho(&self) -> Option<&str> {
        self.get_chirho("CompressType")
    }

    /// Get the block type for compressed modules.
    pub fn block_type_chirho(&self) -> Option<&str> {
        self.get_chirho("BlockType")
    }

    /// Get the cipher key (for encrypted modules).
    pub fn cipher_key_chirho(&self) -> Option<&str> {
        self.get_chirho("CipherKey")
    }

    /// Check if the module is encrypted.
    pub fn is_encrypted_chirho(&self) -> bool {
        self.cipher_key_chirho().is_some() || self.contains_key_chirho("Unlocked")
    }

    /// Get the module description.
    pub fn description_chirho(&self) -> Option<&str> {
        self.get_chirho("Description")
    }

    /// Get the module language.
    pub fn language_chirho(&self) -> Option<&str> {
        self.get_chirho("Lang")
    }

    /// Get the text direction.
    pub fn direction_chirho(&self) -> &str {
        self.get_or_chirho("Direction", "LtoR")
    }

    /// Check if this is a Bible module.
    pub fn is_bible_chirho(&self) -> bool {
        matches!(
            self.module_driver_chirho(),
            Some("RawText") | Some("RawText4") | Some("zText") | Some("zText4")
        )
    }

    /// Check if this is a commentary module.
    pub fn is_commentary_chirho(&self) -> bool {
        matches!(
            self.module_driver_chirho(),
            Some("RawCom") | Some("RawCom4") | Some("zCom") | Some("zCom4")
        )
    }

    /// Check if this is a lexicon/dictionary module.
    pub fn is_lexicon_chirho(&self) -> bool {
        matches!(
            self.module_driver_chirho(),
            Some("RawLD") | Some("RawLD4") | Some("zLD")
        )
    }

    /// Check if this is a general book module.
    pub fn is_genbook_chirho(&self) -> bool {
        matches!(self.module_driver_chirho(), Some("RawGenBook"))
    }

    /// Write the configuration to a writer.
    pub fn write_to_chirho<W: Write>(&self, writer_chirho: &mut W) -> ResultChirho<()> {
        writeln!(writer_chirho, "[{}]", self.name_chirho)?;

        // Write entries in a deterministic order
        let mut keys_chirho: Vec<_> = self.entries_chirho.keys().collect();
        keys_chirho.sort();

        for key_chirho in keys_chirho {
            if let Some(value_chirho) = self.entries_chirho.get(key_chirho) {
                // Handle multi-line values
                let lines_chirho: Vec<&str> = value_chirho.lines().collect();
                if lines_chirho.len() > 1 {
                    writeln!(writer_chirho, "{}={}", key_chirho, lines_chirho[0])?;
                    for line_chirho in &lines_chirho[1..] {
                        writeln!(writer_chirho, "\t{}", line_chirho)?;
                    }
                } else {
                    writeln!(writer_chirho, "{}={}", key_chirho, value_chirho)?;
                }
            }
        }

        Ok(())
    }

    /// Write the configuration to a file.
    pub fn write_to_file_chirho<P: AsRef<Path>>(&self, path_chirho: P) -> ResultChirho<()> {
        let mut file_chirho = File::create(path_chirho)?;
        self.write_to_chirho(&mut file_chirho)
    }
}

/// Manager for install sources configuration (installmgr.conf).
#[derive(Debug, Clone, Default)]
pub struct InstallSourcesConfigChirho {
    /// FTP sources.
    pub ftp_sources_chirho: Vec<InstallSourceChirho>,
    /// HTTP sources.
    pub http_sources_chirho: Vec<InstallSourceChirho>,
    /// HTTPS sources.
    pub https_sources_chirho: Vec<InstallSourceChirho>,
}

/// An install source for SWORD modules.
#[derive(Debug, Clone)]
pub struct InstallSourceChirho {
    /// Source type (FTPSource, HTTPSource, HTTPSSource).
    pub source_type_chirho: String,
    /// Caption/name of the source.
    pub caption_chirho: String,
    /// Source host.
    pub source_chirho: String,
    /// Directory path on the server.
    pub directory_chirho: String,
}

impl InstallSourcesConfigChirho {
    /// Parse install sources from a file.
    pub fn from_file_chirho<P: AsRef<Path>>(path_chirho: P) -> ResultChirho<Self> {
        let content_chirho = std::fs::read_to_string(path_chirho)?;
        Self::from_str_chirho(&content_chirho)
    }

    /// Parse install sources from a string.
    pub fn from_str_chirho(content_chirho: &str) -> ResultChirho<Self> {
        let mut config_chirho = Self::default();

        for line_chirho in content_chirho.lines() {
            let trimmed_chirho = line_chirho.trim();
            if trimmed_chirho.is_empty() || trimmed_chirho.starts_with('#') {
                continue;
            }

            // Parse source entries like:
            // FTPSource=CrossWire|ftp.crosswire.org|/pub/sword/raw
            if let Some(eq_pos_chirho) = trimmed_chirho.find('=') {
                let source_type_chirho = trimmed_chirho[..eq_pos_chirho].trim();
                let value_chirho = trimmed_chirho[eq_pos_chirho + 1..].trim();

                if matches!(source_type_chirho, "FTPSource" | "HTTPSource" | "HTTPSSource") {
                    let parts_chirho: Vec<&str> = value_chirho.split('|').collect();
                    if parts_chirho.len() >= 3 {
                        let source_chirho = InstallSourceChirho {
                            source_type_chirho: source_type_chirho.to_string(),
                            caption_chirho: parts_chirho[0].to_string(),
                            source_chirho: parts_chirho[1].to_string(),
                            directory_chirho: parts_chirho[2].to_string(),
                        };

                        match source_type_chirho {
                            "FTPSource" => config_chirho.ftp_sources_chirho.push(source_chirho),
                            "HTTPSource" => config_chirho.http_sources_chirho.push(source_chirho),
                            "HTTPSSource" => config_chirho.https_sources_chirho.push(source_chirho),
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok(config_chirho)
    }

    /// Get all sources.
    pub fn all_sources_chirho(&self) -> Vec<&InstallSourceChirho> {
        let mut sources_chirho = Vec::new();
        sources_chirho.extend(self.https_sources_chirho.iter());
        sources_chirho.extend(self.http_sources_chirho.iter());
        sources_chirho.extend(self.ftp_sources_chirho.iter());
        sources_chirho
    }

    /// Find a source by caption.
    pub fn find_by_caption_chirho(&self, caption_chirho: &str) -> Option<&InstallSourceChirho> {
        self.all_sources_chirho()
            .into_iter()
            .find(|s| s.caption_chirho == caption_chirho)
    }

    /// Add a source.
    pub fn add_source_chirho(&mut self, source_chirho: InstallSourceChirho) {
        match source_chirho.source_type_chirho.as_str() {
            "FTPSource" => self.ftp_sources_chirho.push(source_chirho),
            "HTTPSource" => self.http_sources_chirho.push(source_chirho),
            "HTTPSSource" => self.https_sources_chirho.push(source_chirho),
            _ => {}
        }
    }

    /// Write to a writer.
    pub fn write_to_chirho<W: Write>(&self, writer_chirho: &mut W) -> ResultChirho<()> {
        writeln!(writer_chirho, "[Install]")?;

        for source_chirho in &self.ftp_sources_chirho {
            writeln!(
                writer_chirho,
                "FTPSource={}|{}|{}",
                source_chirho.caption_chirho,
                source_chirho.source_chirho,
                source_chirho.directory_chirho
            )?;
        }

        for source_chirho in &self.http_sources_chirho {
            writeln!(
                writer_chirho,
                "HTTPSource={}|{}|{}",
                source_chirho.caption_chirho,
                source_chirho.source_chirho,
                source_chirho.directory_chirho
            )?;
        }

        for source_chirho in &self.https_sources_chirho {
            writeln!(
                writer_chirho,
                "HTTPSSource={}|{}|{}",
                source_chirho.caption_chirho,
                source_chirho.source_chirho,
                source_chirho.directory_chirho
            )?;
        }

        Ok(())
    }
}

impl InstallSourceChirho {
    /// Get the URL for this source.
    pub fn url_chirho(&self) -> String {
        let protocol_chirho = match self.source_type_chirho.as_str() {
            "FTPSource" => "ftp",
            "HTTPSource" => "http",
            "HTTPSSource" => "https",
            _ => "https",
        };
        format!("{}://{}{}", protocol_chirho, self.source_chirho, self.directory_chirho)
    }
}

/// Default CrossWire install source.
pub fn default_crosswire_source_chirho() -> InstallSourceChirho {
    InstallSourceChirho {
        source_type_chirho: "HTTPSSource".to_string(),
        caption_chirho: "CrossWire".to_string(),
        source_chirho: "crosswire.org".to_string(),
        directory_chirho: "/ftpmirror/pub/sword/raw".to_string(),
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_simple_config_chirho() {
        let content_chirho = r#"
[KJV]
DataPath=./modules/texts/rawtext/kjv/
ModDrv=RawText
SourceType=OSIS
Encoding=UTF-8
Lang=en
Description=King James Version
        "#;

        let config_chirho = ModuleConfigChirho::from_str_chirho(content_chirho).unwrap();
        assert_eq!(config_chirho.name_chirho, "KJV");
        assert_eq!(config_chirho.get_chirho("ModDrv"), Some("RawText"));
        assert_eq!(config_chirho.get_chirho("Lang"), Some("en"));
        assert_eq!(config_chirho.encoding_chirho(), "UTF-8");
    }

    #[test]
    fn test_parse_multiline_config_chirho() {
        let content_chirho = r#"
[TestMod]
Description=This is a
	multi-line
	description
Lang=en
        "#;

        let config_chirho = ModuleConfigChirho::from_str_chirho(content_chirho).unwrap();
        assert_eq!(config_chirho.name_chirho, "TestMod");

        let desc_chirho = config_chirho.get_chirho("Description").unwrap();
        assert!(desc_chirho.contains("multi-line"));
        assert!(desc_chirho.contains("description"));
    }

    #[test]
    fn test_config_module_type_detection_chirho() {
        let mut config_chirho = ModuleConfigChirho::new_chirho("Test".to_string());

        config_chirho.set_chirho("ModDrv", "RawText");
        assert!(config_chirho.is_bible_chirho());
        assert!(!config_chirho.is_commentary_chirho());

        config_chirho.set_chirho("ModDrv", "RawCom");
        assert!(!config_chirho.is_bible_chirho());
        assert!(config_chirho.is_commentary_chirho());

        config_chirho.set_chirho("ModDrv", "RawLD");
        assert!(config_chirho.is_lexicon_chirho());

        config_chirho.set_chirho("ModDrv", "RawGenBook");
        assert!(config_chirho.is_genbook_chirho());
    }

    #[test]
    fn test_parse_install_sources_chirho() {
        let content_chirho = r#"
[Install]
FTPSource=CrossWire|ftp.crosswire.org|/pub/sword/raw
HTTPSSource=Secure|secure.crosswire.org|/sword
        "#;

        let config_chirho = InstallSourcesConfigChirho::from_str_chirho(content_chirho).unwrap();
        assert_eq!(config_chirho.ftp_sources_chirho.len(), 1);
        assert_eq!(config_chirho.https_sources_chirho.len(), 1);

        let ftp_source_chirho = &config_chirho.ftp_sources_chirho[0];
        assert_eq!(ftp_source_chirho.caption_chirho, "CrossWire");
        assert_eq!(ftp_source_chirho.source_chirho, "ftp.crosswire.org");
    }

    #[test]
    fn test_source_url_chirho() {
        let source_chirho = InstallSourceChirho {
            source_type_chirho: "HTTPSSource".to_string(),
            caption_chirho: "Test".to_string(),
            source_chirho: "example.com".to_string(),
            directory_chirho: "/path/to/modules".to_string(),
        };

        assert_eq!(source_chirho.url_chirho(), "https://example.com/path/to/modules");
    }

    #[test]
    fn test_config_write_roundtrip_chirho() {
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestMod".to_string());
        config_chirho.set_chirho("DataPath", "./modules/test/");
        config_chirho.set_chirho("ModDrv", "RawText");
        config_chirho.set_chirho("Lang", "en");

        let mut output_chirho = Vec::new();
        config_chirho.write_to_chirho(&mut output_chirho).unwrap();

        let written_chirho = String::from_utf8(output_chirho).unwrap();
        let parsed_chirho = ModuleConfigChirho::from_str_chirho(&written_chirho).unwrap();

        assert_eq!(parsed_chirho.name_chirho, config_chirho.name_chirho);
        assert_eq!(parsed_chirho.get_chirho("ModDrv"), config_chirho.get_chirho("ModDrv"));
    }
}
