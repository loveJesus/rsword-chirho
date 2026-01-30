# rsword_chirho

Pure Rust port of the SWORD Bible software library.

> For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. - John 3:16

## Features

- **Full SWORD compatibility**: Read all standard SWORD module binary formats
- **Multiple module types**: Bibles, commentaries, lexicons, general books, devotionals
- **Compression support**: zlib, bzip2, and xz compression
- **Full-text search**: Tantivy-based indexed search with regex fallback
- **Multiple versifications**: KJV, Catholic, LXX, Synodal, Luther, Vulgate, NRSV, Leningrad, Ethiopian
- **Markup filters**: Convert OSIS, ThML, GBF to HTML or plain text
- **Import tools**: OSIS, TEI, VPL, and IMP format parsers
- **Encrypted modules**: Support for cipher-protected content

## Installation

```toml
[dependencies]
rsword_chirho = "0.1"
```

### Feature Flags

```toml
[dependencies]
rsword_chirho = { version = "0.1", features = ["bzip2", "xz", "search"] }
```

| Feature | Default | Description |
|---------|---------|-------------|
| `bzip2` | Yes | bzip2 compression support |
| `xz` | Yes | xz/lzma compression support |
| `search` | Yes | Tantivy full-text search |
| `parallel` | No | Parallel processing with rayon |

## Quick Start

```rust
use rsword_chirho::{SwMgrChirho, VerseKeyChirho};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load modules from system paths (~/.sword, /usr/share/sword, etc.)
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    // List available modules
    println!("Found {} modules:", mgr_chirho.module_count_chirho());
    for name_chirho in mgr_chirho.get_module_names_chirho() {
        println!("  - {}", name_chirho);
    }

    // Get a Bible module and look up a verse
    if let Some(mut module_chirho) = mgr_chirho.get_module_driver_chirho("KJV")? {
        let key_chirho = VerseKeyChirho::from_str_chirho("John 3:16")?;
        module_chirho.set_key_chirho(&key_chirho)?;
        let text_chirho = module_chirho.get_raw_entry_chirho()?;
        println!("{}: {}", key_chirho, text_chirho);
    }

    Ok(())
}
```

## Verse Navigation

```rust
use rsword_chirho::VerseKeyChirho;

// Parse various reference formats
let key1_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1")?;
let key2_chirho = VerseKeyChirho::from_str_chirho("Gen 1:1")?;      // Abbreviation
let key3_chirho = VerseKeyChirho::from_str_chirho("1 John 3:16")?;  // Numbered book
let key4_chirho = VerseKeyChirho::from_str_chirho("Rev 22:21")?;    // Last verse

// Navigate through verses
let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1")?;
key_chirho.increment_chirho(1);  // Move to Genesis 1:2
key_chirho.increment_chirho(30); // Move to Genesis 2:1 (crosses chapter)

// Get verse components
let book_chirho = key_chirho.get_book_name_chirho();
let chapter_chirho = key_chirho.get_chapter_chirho();
let verse_chirho = key_chirho.get_verse_chirho();
```

## Versification Systems

Different Bible traditions use different verse numbering:

```rust
use rsword_chirho::versification_chirho::{get_versification_chirho, VersificationManagerChirho};

// Available systems
let manager_chirho = VersificationManagerChirho::new_chirho();
for name_chirho in manager_chirho.list_available_chirho() {
    println!("{}", name_chirho);
}
// Output: KJV, Catholic, LXX, Synodal, Luther, Vulgate, NRSV, Leningrad, Ethiopian

// Get a specific versification
let kjv_chirho = get_versification_chirho("KJV").unwrap();
let catholic_chirho = get_versification_chirho("Catholic").unwrap();
```

## Search

```rust
use rsword_chirho::{SearchOptionsChirho, SearchTypeChirho, RegexSearchChirho, SearchEngineChirho};

// Create search options
let options_chirho = SearchOptionsChirho::new_chirho()
    .with_type_chirho(SearchTypeChirho::PhraseChirho)
    .case_insensitive_chirho(true)
    .with_max_results_chirho(50);

// Search using regex engine (no index required)
let search_chirho = RegexSearchChirho::new_chirho(&module_path_chirho);
let results_chirho = search_chirho.search_chirho("for God so loved", &options_chirho)?;

// For better performance, use Tantivy indexed search
let mut tantivy_chirho = TantivySearchChirho::new_chirho(&module_path_chirho, "KJV")?;
if !tantivy_chirho.has_index_chirho() {
    tantivy_chirho.create_index_chirho()?;  // One-time index creation
}
let results_chirho = tantivy_chirho.search_chirho("love", &options_chirho)?;
```

## Filters

Convert module markup to different formats:

```rust
use rsword_chirho::filters_chirho::{FilterChirho, OsisToHtmlFilterChirho, FilterOptionsChirho};

let filter_chirho = OsisToHtmlFilterChirho::new_chirho();
let osis_chirho = r#"<verse osisID="John.3.16">For God so loved...</verse>"#;
let html_chirho = filter_chirho.process_chirho(osis_chirho)?;
```

## Module Types Supported

| Type | Description | Storage Formats |
|------|-------------|-----------------|
| **zText** | Compressed Bible modules | Block-compressed with zlib/bzip2/xz |
| **RawText** | Uncompressed Bible modules | 6-byte or 8-byte index |
| **zCom** | Compressed commentary | Block-compressed |
| **RawCom** | Uncompressed commentary | 6-byte or 8-byte index |
| **zLD** | Compressed lexicon/dictionary | Block-compressed |
| **RawLD** | Uncompressed lexicon | 6-byte or 8-byte index |
| **RawGenBook** | General book modules | Tree-structured |

## Installing Modules

```rust
use rsword_chirho::manager_chirho::InstallMgrChirho;

let mut install_mgr_chirho = InstallMgrChirho::new_chirho();
install_mgr_chirho.init_chirho()?;
install_mgr_chirho.sync_config_chirho()?;

// List available modules from CrossWire
let modules_chirho = install_mgr_chirho.get_remote_modules_chirho("CrossWire")?;
for m in modules_chirho {
    println!("{}: {}", m.name_chirho, m.description_chirho);
}

// Install a module
install_mgr_chirho.install_module_chirho("CrossWire", "KJV")?;
```

## Importing Content

Create modules from source formats:

```rust
use rsword_chirho::import_chirho::{OsisParserChirho, OsisParserConfigChirho};

// Parse OSIS XML
let config_chirho = OsisParserConfigChirho::default();
let mut parser_chirho = OsisParserChirho::new_chirho(config_chirho);
let doc_chirho = parser_chirho.parse_file_chirho(Path::new("bible.osis"))?;

for verse_chirho in &doc_chirho.verses_chirho {
    println!("{}: {}", verse_chirho.osis_id_chirho, verse_chirho.text_chirho);
}
```

## CLI Tools

The `sword_tools_chirho` crate provides CLI tools:

- `diatheke_chirho` - Bible text lookup and search
- `installmgr_chirho` - Module installation management
- `osis2mod_chirho` - Create modules from OSIS XML
- `tei2mod_chirho` - Create lexicons from TEI XML
- `mod2imp_chirho` - Export modules to IMP format

## API Documentation

Run `cargo doc --open` to generate and view the full API documentation.

## License

GPL-2.0-or-later (same as SWORD library)
