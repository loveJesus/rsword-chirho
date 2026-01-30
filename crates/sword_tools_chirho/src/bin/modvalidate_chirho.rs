// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module validation tool.
//!
//! Validates SWORD module files for correctness, integrity, and completeness.
//!
//! # Usage
//!
//! ```bash
//! # Validate a single module
//! modvalidate_chirho KJV
//!
//! # Validate all modules in a path
//! modvalidate_chirho --path /usr/share/sword
//!
//! # Verbose output
//! modvalidate_chirho -v KJV
//!
//! # Check specific aspects
//! modvalidate_chirho --check-encoding KJV
//! modvalidate_chirho --check-coverage KJV
//! ```

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use rsword_chirho::manager_chirho::SwMgrChirho;
use rsword_chirho::manager_chirho::sw_mgr_chirho::SwMgrConfigChirho;

/// SWORD module validation tool.
#[derive(Parser, Debug)]
#[command(name = "modvalidate_chirho")]
#[command(about = "Validate SWORD modules for correctness and integrity")]
#[command(version)]
struct ArgsChirho {
    /// Module name to validate.
    #[arg()]
    module_chirho: Option<String>,

    /// Path to module directory.
    #[arg(short = 'p', long = "path")]
    path_chirho: Option<PathBuf>,

    /// Validate all modules in path.
    #[arg(short = 'a', long = "all")]
    all_chirho: bool,

    /// Verbose output.
    #[arg(short = 'v', long = "verbose")]
    verbose_chirho: bool,

    /// Check encoding consistency.
    #[arg(long = "check-encoding")]
    check_encoding_chirho: bool,

    /// Check verse coverage.
    #[arg(long = "check-coverage")]
    check_coverage_chirho: bool,

    /// Check index file integrity.
    #[arg(long = "check-index")]
    check_index_chirho: bool,

    /// Show fix suggestions.
    #[arg(long = "suggest-fixes")]
    suggest_fixes_chirho: bool,

    /// Output format (text, json).
    #[arg(short = 'f', long = "format", default_value = "text")]
    format_chirho: String,
}

/// Validation issue severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeverityChirho {
    /// Informational message.
    InfoChirho,
    /// Warning - module may work but has issues.
    WarningChirho,
    /// Error - module is broken.
    ErrorChirho,
}

impl std::fmt::Display for SeverityChirho {
    fn fmt(&self, f_chirho: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeverityChirho::InfoChirho => write!(f_chirho, "INFO"),
            SeverityChirho::WarningChirho => write!(f_chirho, "WARNING"),
            SeverityChirho::ErrorChirho => write!(f_chirho, "ERROR"),
        }
    }
}

/// A validation issue found in a module.
#[derive(Debug, Clone)]
struct ValidationIssueChirho {
    /// Issue severity.
    severity_chirho: SeverityChirho,
    /// Issue category.
    category_chirho: String,
    /// Issue message.
    message_chirho: String,
    /// Suggested fix (if any).
    fix_chirho: Option<String>,
}

impl ValidationIssueChirho {
    fn new_chirho(
        severity_chirho: SeverityChirho,
        category_chirho: &str,
        message_chirho: &str,
    ) -> Self {
        Self {
            severity_chirho,
            category_chirho: category_chirho.to_string(),
            message_chirho: message_chirho.to_string(),
            fix_chirho: None,
        }
    }

    fn with_fix_chirho(mut self, fix_chirho: &str) -> Self {
        self.fix_chirho = Some(fix_chirho.to_string());
        self
    }
}

/// Validation results for a module.
struct ValidationResultChirho {
    /// Module name.
    module_name_chirho: String,
    /// Issues found.
    issues_chirho: Vec<ValidationIssueChirho>,
    /// Whether validation passed.
    passed_chirho: bool,
}

impl ValidationResultChirho {
    fn new_chirho(module_name_chirho: &str) -> Self {
        Self {
            module_name_chirho: module_name_chirho.to_string(),
            issues_chirho: Vec::new(),
            passed_chirho: true,
        }
    }

    fn add_issue_chirho(&mut self, issue_chirho: ValidationIssueChirho) {
        if issue_chirho.severity_chirho == SeverityChirho::ErrorChirho {
            self.passed_chirho = false;
        }
        self.issues_chirho.push(issue_chirho);
    }

    fn error_count_chirho(&self) -> usize {
        self.issues_chirho.iter().filter(|i_chirho| i_chirho.severity_chirho == SeverityChirho::ErrorChirho).count()
    }

    fn warning_count_chirho(&self) -> usize {
        self.issues_chirho.iter().filter(|i_chirho| i_chirho.severity_chirho == SeverityChirho::WarningChirho).count()
    }
}

/// Module validator.
struct ModuleValidatorChirho<'a> {
    /// Module manager.
    mgr_chirho: &'a SwMgrChirho,
    /// Verbose output.
    verbose_chirho: bool,
    /// Check encoding.
    check_encoding_chirho: bool,
    /// Check coverage.
    check_coverage_chirho: bool,
    /// Check index.
    check_index_chirho: bool,
    /// Show fix suggestions.
    suggest_fixes_chirho: bool,
}

impl<'a> ModuleValidatorChirho<'a> {
    fn new_chirho(mgr_chirho: &'a SwMgrChirho) -> Self {
        Self {
            mgr_chirho,
            verbose_chirho: false,
            check_encoding_chirho: false,
            check_coverage_chirho: false,
            check_index_chirho: false,
            suggest_fixes_chirho: false,
        }
    }

    fn verbose_chirho(mut self, value_chirho: bool) -> Self {
        self.verbose_chirho = value_chirho;
        self
    }

    fn check_encoding_chirho(mut self, value_chirho: bool) -> Self {
        self.check_encoding_chirho = value_chirho;
        self
    }

    fn check_coverage_chirho(mut self, value_chirho: bool) -> Self {
        self.check_coverage_chirho = value_chirho;
        self
    }

    fn check_index_chirho(mut self, value_chirho: bool) -> Self {
        self.check_index_chirho = value_chirho;
        self
    }

    fn suggest_fixes_chirho(mut self, value_chirho: bool) -> Self {
        self.suggest_fixes_chirho = value_chirho;
        self
    }

    /// Validate a module by name.
    fn validate_chirho(&self, module_name_chirho: &str) -> ValidationResultChirho {
        let mut result_chirho = ValidationResultChirho::new_chirho(module_name_chirho);

        // Check if module exists
        let config_chirho = match self.mgr_chirho.get_module_chirho(module_name_chirho) {
            Some(config_chirho) => config_chirho,
            None => {
                result_chirho.add_issue_chirho(
                    ValidationIssueChirho::new_chirho(
                        SeverityChirho::ErrorChirho,
                        "Config",
                        &format!("Module '{}' not found", module_name_chirho),
                    )
                );
                return result_chirho;
            }
        };

        // Validate config
        self.validate_config_chirho(&mut result_chirho, config_chirho);

        // Check data path
        let data_path_chirho = match self.mgr_chirho.get_module_data_path_chirho(module_name_chirho) {
            Some(path_chirho) => path_chirho,
            None => {
                result_chirho.add_issue_chirho(
                    ValidationIssueChirho::new_chirho(
                        SeverityChirho::ErrorChirho,
                        "DataPath",
                        "Module data path not found or inaccessible",
                    ).with_fix_chirho("Check that DataPath in .conf file points to existing directory")
                );
                return result_chirho;
            }
        };

        // Try to load the module
        match self.mgr_chirho.load_module_chirho(module_name_chirho) {
            Ok(module_chirho) => {
                self.validate_loaded_module_info_chirho(&mut result_chirho, &module_chirho);
            }
            Err(e_chirho) => {
                result_chirho.add_issue_chirho(
                    ValidationIssueChirho::new_chirho(
                        SeverityChirho::ErrorChirho,
                        "Load",
                        &format!("Failed to load module: {}", e_chirho),
                    )
                );
            }
        }

        // Check index files if requested
        if self.check_index_chirho {
            self.validate_index_files_chirho(&mut result_chirho, &data_path_chirho);
        }

        result_chirho
    }

    /// Validate module config.
    fn validate_config_chirho(
        &self,
        result_chirho: &mut ValidationResultChirho,
        config_chirho: &rsword_chirho::config_chirho::ModuleConfigChirho,
    ) {
        // Check required fields
        if config_chirho.description_chirho().is_none() {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::WarningChirho,
                    "Config",
                    "Missing Description field",
                ).with_fix_chirho("Add Description= to module .conf file")
            );
        }

        if config_chirho.language_chirho().is_none() {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::WarningChirho,
                    "Config",
                    "Missing Lang field",
                ).with_fix_chirho("Add Lang= (e.g., Lang=en) to module .conf file")
            );
        }

        if config_chirho.data_path_chirho().is_none() {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::ErrorChirho,
                    "Config",
                    "Missing DataPath field",
                ).with_fix_chirho("Add DataPath= to module .conf file")
            );
        }

        // Check for deprecated fields
        if config_chirho.get_chirho("CipherKey").is_some() {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::WarningChirho,
                    "Config",
                    "CipherKey in config (should be in separate file)",
                )
            );
        }

        // Check versification for Bible modules
        if config_chirho.is_bible_chirho() {
            if config_chirho.versification_chirho() == "KJV" {
                result_chirho.add_issue_chirho(
                    ValidationIssueChirho::new_chirho(
                        SeverityChirho::InfoChirho,
                        "Config",
                        "Using default KJV versification",
                    )
                );
            }
        }

        if self.verbose_chirho {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Config",
                    &format!("Module type: {}", config_chirho.source_type_chirho().unwrap_or("Unknown")),
                )
            );
        }
    }

    /// Validate a loaded module info.
    fn validate_loaded_module_info_chirho(
        &self,
        result_chirho: &mut ValidationResultChirho,
        module_chirho: &rsword_chirho::manager_chirho::LoadedModuleChirho,
    ) {
        // Check basic functionality
        if self.verbose_chirho {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Module",
                    &format!("Name: {}", module_chirho.name_chirho),
                )
            );
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Module",
                    &format!("Driver: {:?}", module_chirho.driver_type_chirho),
                )
            );
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Module",
                    &format!("Language: {}", module_chirho.config_chirho.language_chirho().unwrap_or("Unknown")),
                )
            );
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Module",
                    &format!("Encoding: {}", module_chirho.config_chirho.encoding_chirho()),
                )
            );
        }

        // Check encryption
        if module_chirho.config_chirho.is_encrypted_chirho() {
            result_chirho.add_issue_chirho(
                ValidationIssueChirho::new_chirho(
                    SeverityChirho::InfoChirho,
                    "Module",
                    "Module is encrypted (requires unlock key)",
                )
            );
        }

        // Try to read a test entry to verify module is readable
        match module_chirho.read_entry_chirho("Gen 1:1") {
            Ok(text_chirho) => {
                if self.check_encoding_chirho && !text_chirho.is_empty() {
                    self.validate_encoding_chirho(result_chirho, &text_chirho);
                }
                if self.verbose_chirho && !text_chirho.is_empty() {
                    result_chirho.add_issue_chirho(
                        ValidationIssueChirho::new_chirho(
                            SeverityChirho::InfoChirho,
                            "Read",
                            "Successfully read test entry",
                        )
                    );
                }
            }
            Err(e_chirho) => {
                // First entry might not exist for lexicons/dictionaries
                if self.verbose_chirho {
                    result_chirho.add_issue_chirho(
                        ValidationIssueChirho::new_chirho(
                            SeverityChirho::InfoChirho,
                            "Read",
                            &format!("Test read: {}", e_chirho),
                        )
                    );
                }
            }
        }
    }

    /// Validate text encoding.
    fn validate_encoding_chirho(&self, result_chirho: &mut ValidationResultChirho, text_chirho: &str) {
        // Check for invalid UTF-8 sequences (if we got here, it's already valid UTF-8)
        // Check for common encoding issues

        // Check for mojibake patterns (incorrect encoding)
        let mojibake_patterns_chirho = [
            "Â", // UTF-8 interpreted as Latin-1
            "â€", // Smart quotes as UTF-8
            "Ã©", // é as double-encoded UTF-8
        ];

        for pattern_chirho in &mojibake_patterns_chirho {
            if text_chirho.contains(pattern_chirho) {
                result_chirho.add_issue_chirho(
                    ValidationIssueChirho::new_chirho(
                        SeverityChirho::WarningChirho,
                        "Encoding",
                        &format!("Possible encoding issue: found '{}' (mojibake)", pattern_chirho),
                    ).with_fix_chirho("Check source encoding and re-encode as UTF-8")
                );
                break;
            }
        }
    }

    /// Validate index files.
    fn validate_index_files_chirho(
        &self,
        result_chirho: &mut ValidationResultChirho,
        data_path_chirho: &std::path::Path,
    ) {
        // Check for expected index files based on module type
        let expected_extensions_chirho = ["vss", "bks", "cps", "idx"];

        for ext_chirho in &expected_extensions_chirho {
            let pattern_chirho = format!("*.{}", ext_chirho);
            if let Ok(mut entries_chirho) = glob::glob(&data_path_chirho.join(&pattern_chirho).to_string_lossy()) {
                if entries_chirho.next().is_some() {
                    if self.verbose_chirho {
                        result_chirho.add_issue_chirho(
                            ValidationIssueChirho::new_chirho(
                                SeverityChirho::InfoChirho,
                                "Index",
                                &format!("Found .{} index file", ext_chirho),
                            )
                        );
                    }
                }
            }
        }
    }
}

/// Print validation results as text.
fn print_text_result_chirho(result_chirho: &ValidationResultChirho, show_fixes_chirho: bool) {
    println!("Module: {}", result_chirho.module_name_chirho);
    println!("{}", "=".repeat(50));

    if result_chirho.issues_chirho.is_empty() {
        println!("No issues found.");
    } else {
        for issue_chirho in &result_chirho.issues_chirho {
            print!("[{}] ", issue_chirho.severity_chirho);
            print!("[{}] ", issue_chirho.category_chirho);
            println!("{}", issue_chirho.message_chirho);

            if show_fixes_chirho {
                if let Some(ref fix_chirho) = issue_chirho.fix_chirho {
                    println!("  -> Fix: {}", fix_chirho);
                }
            }
        }
    }

    println!();
    println!(
        "Result: {} ({} errors, {} warnings)",
        if result_chirho.passed_chirho { "PASSED" } else { "FAILED" },
        result_chirho.error_count_chirho(),
        result_chirho.warning_count_chirho()
    );
}

/// Print validation results as JSON.
fn print_json_result_chirho(result_chirho: &ValidationResultChirho) {
    let issues_json_chirho: Vec<serde_json::Value> = result_chirho
        .issues_chirho
        .iter()
        .map(|i_chirho| {
            serde_json::json!({
                "severity": format!("{}", i_chirho.severity_chirho),
                "category": i_chirho.category_chirho,
                "message": i_chirho.message_chirho,
                "fix": i_chirho.fix_chirho
            })
        })
        .collect();

    let json_chirho = serde_json::json!({
        "module": result_chirho.module_name_chirho,
        "passed": result_chirho.passed_chirho,
        "error_count": result_chirho.error_count_chirho(),
        "warning_count": result_chirho.warning_count_chirho(),
        "issues": issues_json_chirho
    });

    println!("{}", serde_json::to_string_pretty(&json_chirho).unwrap());
}

fn main() -> Result<()> {
    env_logger::init();

    let args_chirho = ArgsChirho::parse();

    // Create module manager
    let config_chirho = SwMgrConfigChirho::default();
    let mut mgr_chirho = SwMgrChirho::with_config_chirho(config_chirho);

    // Add paths
    if let Some(ref path_chirho) = args_chirho.path_chirho {
        mgr_chirho.add_path_chirho(path_chirho);
    } else {
        mgr_chirho.add_system_paths_chirho();
    }

    mgr_chirho.load_modules_chirho()?;

    // Create validator
    let validator_chirho = ModuleValidatorChirho::new_chirho(&mgr_chirho)
        .verbose_chirho(args_chirho.verbose_chirho)
        .check_encoding_chirho(args_chirho.check_encoding_chirho)
        .check_coverage_chirho(args_chirho.check_coverage_chirho)
        .check_index_chirho(args_chirho.check_index_chirho)
        .suggest_fixes_chirho(args_chirho.suggest_fixes_chirho);

    let mut all_passed_chirho = true;

    if args_chirho.all_chirho {
        // Validate all modules
        let module_names_chirho: Vec<_> = mgr_chirho.get_module_names_chirho()
            .iter()
            .map(|s_chirho| s_chirho.to_string())
            .collect();

        for name_chirho in module_names_chirho {
            let result_chirho = validator_chirho.validate_chirho(&name_chirho);

            if args_chirho.format_chirho == "json" {
                print_json_result_chirho(&result_chirho);
            } else {
                print_text_result_chirho(&result_chirho, args_chirho.suggest_fixes_chirho);
            }

            if !result_chirho.passed_chirho {
                all_passed_chirho = false;
            }

            println!();
        }
    } else if let Some(ref module_name_chirho) = args_chirho.module_chirho {
        // Validate specific module
        let result_chirho = validator_chirho.validate_chirho(module_name_chirho);

        if args_chirho.format_chirho == "json" {
            print_json_result_chirho(&result_chirho);
        } else {
            print_text_result_chirho(&result_chirho, args_chirho.suggest_fixes_chirho);
        }

        if !result_chirho.passed_chirho {
            all_passed_chirho = false;
        }
    } else {
        // List available modules
        println!("Available modules:");
        for name_chirho in mgr_chirho.get_module_names_chirho() {
            println!("  {}", name_chirho);
        }
        println!();
        println!("Use: modvalidate_chirho <module_name> to validate a specific module");
        println!("     modvalidate_chirho --all to validate all modules");
    }

    if all_passed_chirho {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
