# rsword-chirho Architecture

<!-- For God so loved the world that he gave his only begotten Son,
that whoever believes in him should not perish but have eternal life. - John 3:16 -->

## Overview

rsword-chirho is a pure Rust implementation of the SWORD Bible software library, providing full binary format compatibility with existing SWORD modules.

## Crate Structure

```
rsword-chirho/
├── crates/
│   ├── rsword_chirho/          # Core library
│   │   └── src/
│   │       ├── lib.rs          # Library root
│   │       ├── error_chirho.rs # Error types
│   │       ├── keys_chirho/    # Key types (verse, tree, string, date)
│   │       ├── versification_chirho/  # Verse systems
│   │       ├── modules_chirho/ # Module drivers
│   │       ├── storage_chirho/ # Binary format handlers
│   │       ├── compression_chirho/    # Compression support
│   │       ├── filters_chirho/ # Markup filters
│   │       ├── search_chirho/  # Search engines
│   │       ├── manager_chirho/ # Module management
│   │       ├── config_chirho/  # Configuration parsing
│   │       └── locale_chirho/  # Internationalization
│   │
│   └── sword_tools_chirho/     # CLI binaries
│       └── src/bin/
│           ├── diatheke_chirho.rs
│           ├── installmgr_chirho.rs
│           └── ...
```

## Module Loading Pipeline

```
.conf file → ModuleConfigChirho → ModuleFactory → SwModule implementation
                                         ↓
                     storage_chirho (RawVerse/ZVerse/RawStr)
                                         ↓
                     SwModuleChirho trait (read/iterate/search)
```

### Key Types

| Type | Description | Format |
|------|-------------|--------|
| `VerseKeyChirho` | Bible verse references | Book Chapter:Verse |
| `TreeKeyChirho` | General book hierarchies | /Parent/Child/Entry |
| `StrKeyChirho` | Lexicon/dictionary keys | Strong's numbers, words |
| `DateKeyChirho` | Daily devotional keys | MM.DD |
| `ListKeyChirho` | Search result lists | Multiple keys |

### Module Drivers

| Driver | Format | Description |
|--------|--------|-------------|
| `RawTextChirho` | 6-byte index | Uncompressed Bible text |
| `RawText4Chirho` | 8-byte index | Large text entries |
| `ZTextChirho` | Compressed | Block-compressed Bible |
| `ZText4Chirho` | Compressed | Large compressed entries |
| `RawComChirho` | Commentary | Verse-keyed commentary |
| `RawLdChirho` | Lexicon | Key-value dictionary |
| `RawGenBookChirho` | General | Tree-structured books |
| `DailyDevotionalChirho` | Devotional | Date-keyed readings |
| `GlossaryChirho` | Glossary | Bidirectional lookup |
| `ImageModuleChirho` | Images | Binary images with hotspots |

## Filter Chain System

```
Raw module text → Source filter → Option filters → Output filter → Rendered text
                       ↓                ↓               ↓
                  (OSIS/ThML/GBF)  (Strongs/morph)  (HTML/plain)
```

### Available Filters

- **Source Filters**: OSIS, ThML, GBF parsers
- **Option Filters**: Strong's numbers, morphology, footnotes, cross-references, red letter, headings
- **Output Filters**: HTML, plain text, RTF

## Versification System

Supports multiple verse numbering systems:

- KJV (default, Protestant 66-book canon)
- Catholic (includes deuterocanonical books)
- LXX (Septuagint ordering)
- Vulgate (Latin names, alternate ordering)
- Luther (German names)
- Synodal (Russian Orthodox)

### Index Calculation

```rust
// Verse index = testament_offset + book_offset + chapter_offset + verse
let index = v11n.calculate_index_chirho(testament, book, chapter, verse);
```

## Search Architecture

### Search Types

1. **RegexSearchChirho**: Pattern matching with regex
2. **TantivySearchChirho**: Full-text search with ranking
3. **ParallelSearchChirho**: Concurrent search with Rayon

### Query Parser

Supports Boolean operators, wildcards, fuzzy matching:
```
"love AND God"
"grace OR mercy"
"kingdom NOT heaven"
"lov*" (wildcard)
"love~2" (fuzzy, distance 2)
```

## Compression Support

| Type | Description |
|------|-------------|
| ZIP (zlib) | Standard compression |
| BZIP2 | Higher ratio |
| XZ/LZMA | Maximum compression |
| LZSS | Legacy format |

## Installation Manager

```
InstallMgrChirho
    ├── Sources (CrossWire, eBible, custom)
    ├── Remote module lists (mods.d.tar.gz)
    ├── Module download/extract
    └── Version comparison/upgrades
```

## Error Handling

All functions return `ResultChirho<T>` with typed errors:

```rust
pub enum ErrorChirho {
    IoChirho(std::io::Error),
    InvalidVerseReferenceChirho { reference },
    ModuleNotFoundChirho { name },
    InvalidConfigChirho { message },
    // ...
}
```

## Thread Safety

- Module readers are `Send` but not `Sync` (contain file handles)
- VersificationChirho is `Arc<>` wrapped for sharing
- Search indices are thread-safe
- ParallelSearchChirho uses Rayon for concurrent operations

## Naming Convention

All identifiers use the Chirho suffix:
- Functions: `function_name_chirho`
- Structs: `StructNameChirho`
- Constants: `CONSTANT_NAME_CHIRHO`
