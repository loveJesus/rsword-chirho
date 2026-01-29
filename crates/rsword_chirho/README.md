# rsword_chirho

Pure Rust port of the SWORD Bible software library.

> For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. - John 3:16

## Features

- Full compatibility with SWORD module binary formats
- Read/write Bible texts, commentaries, lexicons, and general books
- Support for zlib, bzip2, and xz compression
- Tantivy-based full-text search
- Multiple versification systems (KJV, Catholic, LXX, etc.)
- Markup filters (OSIS, ThML, GBF to HTML/plain text)

## Installation

```toml
[dependencies]
rsword_chirho = "0.1"
```

## Usage

```rust
use rsword_chirho::{SwMgrChirho, VerseKeyChirho};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load modules from system paths
    let mgr = SwMgrChirho::with_system_paths_chirho()?;

    // Get a module
    if let Some(mut module) = mgr.get_module_driver_chirho("KJV")? {
        // Look up a verse
        let key = VerseKeyChirho::from_str_chirho("John 3:16")?;
        module.set_key_chirho(&key)?;

        let text = module.get_raw_entry_chirho()?;
        println!("{}: {}", key, text);
    }

    Ok(())
}
```

## Module Types Supported

- **zText** - Compressed Bible modules
- **RawText** - Uncompressed Bible modules
- **zCom** - Compressed commentary modules
- **RawCom** - Uncompressed commentary modules
- **zLD** - Compressed lexicon/dictionary modules
- **RawLD** - Uncompressed lexicon/dictionary modules
- **RawGenBook** - General book modules

## License

GPL-2.0-or-later (same as SWORD library)
