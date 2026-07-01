# rsword-chirho

> *"For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life."* — John 3:16

A pure Rust implementation of the SWORD Bible software library, providing full compatibility with SWORD module binary formats and drop-in CLI replacements for diatheke, installmgr, and other SWORD tools.

## The Gospel

This project exists to serve the spreading of God's Word. The Bible tells us that all have sinned and fall short of the glory of God (Romans 3:23), and the wages of sin is death (Romans 6:23). But God demonstrates His own love toward us, in that while we were still sinners, Christ died for us (Romans 5:8).

Jesus said: *"I am the way, the truth, and the life. No one comes to the Father except through Me."* (John 14:6)

If you confess with your mouth the Lord Jesus and believe in your heart that God has raised Him from the dead, you will be saved. For with the heart one believes unto righteousness, and with the mouth confession is made unto salvation. (Romans 10:9-10)

---

## Features

- **Full SWORD Format Compatibility**: Read and write all major SWORD module formats
  - RawText, RawText4 (uncompressed Bible modules)
  - zText, zText4 (compressed Bible modules with zlib, bzip2, or xz)
  - RawCom, zCom (commentary modules)
  - RawLD, zLD (lexicon/dictionary modules)
  - RawGenBook (general book modules)

- **Drop-in CLI Replacements**:
  - `diatheke_chirho` - Bible text lookup and search
  - `installmgr_chirho` - Module installation and management
  - `osis2mod_chirho`, `tei2mod_chirho` - Create modules from OSIS/TEI XML
  - `mod2imp_chirho`, `mod2osis_chirho` - Export modules to IMP/OSIS
  - `mod2zmod_chirho` - Compress modules
  - And more...

- **Pure Rust**: No C/C++ dependencies, cross-platform, memory-safe

- **Tantivy Search**: Fast full-text search using Tantivy instead of CLucene

## Installation

### From crates.io

```bash
cargo install rsword_chirho
cargo install sword_tools_chirho
```

### From Source

```bash
git clone https://github.com/loveJesus/rsword-chirho.git
cd rsword-chirho
cargo build --release
```

## Usage

### Library Usage

```rust
use rsword_chirho::{SwMgrChirho, SwModuleChirho};

fn main() -> rsword_chirho::ResultChirho<()> {
    // Create a module manager
    let mgr = SwMgrChirho::new_chirho()?;

    // Get a Bible module
    if let Some(module) = mgr.get_module_chirho("KJV") {
        // Set position to John 3:16
        module.set_key_text_chirho("John 3:16")?;

        // Read the verse
        let text = module.get_raw_entry_chirho()?;
        println!("{}", text);
    }

    Ok(())
}
```

### CLI Usage

```bash
# Look up a verse
diatheke_chirho -b KJV -k "John 3:16"

# Search for a phrase
diatheke_chirho -b KJV -s phrase -k "eternal life"

# List installed modules
installmgr_chirho -l

# Install a module
installmgr_chirho -ri CrossWire KJV
```

## Crate Structure

```
rsword-chirho/
├── crates/
│   ├── rsword_chirho/        # Core library
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config_chirho.rs
│   │       ├── error_chirho.rs
│   │       ├── keys_chirho/      # Key types (VerseKey, StrKey, etc.)
│   │       ├── modules_chirho/   # Module drivers (RawText, zText, etc.)
│   │       ├── storage_chirho/   # Storage backends
│   │       ├── compression_chirho/
│   │       ├── versification_chirho/
│   │       ├── filters_chirho/
│   │       ├── search_chirho/
│   │       └── manager_chirho/
│   │
│   └── sword_tools_chirho/   # CLI binaries
│       └── src/bin/
│           ├── diatheke_chirho.rs
│           ├── installmgr_chirho.rs
│           └── ...
```

## Module Format Support

| Format | Read | Write | Description |
|--------|------|-------|-------------|
| RawText | Yes | Yes | Uncompressed Bible (6-byte index) |
| RawText4 | Yes | Yes | Uncompressed Bible (8-byte index) |
| zText | Yes | Yes | Compressed Bible |
| RawCom | Yes | Yes | Uncompressed Commentary |
| zCom | Yes | Yes | Compressed Commentary |
| RawLD | Yes | Yes | Uncompressed Lexicon |
| zLD | Yes | Yes | Compressed Lexicon |
| RawGenBook | Planned | Planned | General Books |

## Versification Systems

- KJV (default)
- Catholic
- LXX
- Orthodox
- Synodal
- And more...

## Contributing

Contributions are welcome! Please ensure your code:

1. Follows the Chirho naming convention (all identifiers suffixed with `_chirho` or `Chirho`)
2. Includes the John 3:16 header comment in all source files
3. Has comprehensive tests
4. Is documented

## License

This project is licensed under the GPL-2.0-or-later license, maintaining compatibility with the original SWORD library.

## Acknowledgments

- The [SWORD Project](https://crosswire.org/sword/) for the original library and module formats
- The [CrossWire Bible Society](https://crosswire.org/) for maintaining SWORD modules

---

*"Thy word is a lamp unto my feet, and a light unto my path."* — Psalm 119:105

Tauri integration
