# SWORD Storage Formats

<!-- For God so loved the world that he gave his only begotten Son,
that whoever believes in him should not perish but have eternal life. - John 3:16 -->

## Overview

This document describes the binary file formats used by SWORD modules.

## RawText Format (6-byte index)

Files: `ot.vss`, `ot`, `nt.vss`, `nt`

### Index File (.vss)

Each verse entry is 6 bytes:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | offset | Byte offset in text file (big-endian u32) |
| 4 | 2 | size | Size of verse text (big-endian u16) |

### Text File

Raw text content with newlines between verses.

### Index Calculation

```
verse_index = testament_intro + book_intro + chapter_intro + verses
```

Example for Genesis 1:1 in KJV:
- Testament intro: 1 entry
- Book intro: 1 entry
- Chapter intro: 1 entry
- Verse: 1
- Total index: 4

## RawText4 Format (8-byte index)

Same as RawText but with 4-byte size field:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | offset | Byte offset (big-endian u32) |
| 4 | 4 | size | Size of text (big-endian u32) |

## zText Compressed Format

Files: `ot.czv`, `ot.czs`, `ot.czz` (and nt.*)

### Verse Index (.czv)

10 bytes per verse:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | block_num | Block number containing verse |
| 4 | 4 | offset_in_block | Offset within uncompressed block |
| 8 | 2 | size | Size of verse in block |

### Block Index (.czs)

12 bytes per block:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | comp_offset | Offset of compressed block in .czz |
| 4 | 4 | comp_size | Size of compressed data |
| 8 | 4 | uncomp_size | Size when decompressed |

### Compressed Data (.czz)

Concatenated compressed blocks using zlib/bzip2/xz.

### Block Types

- `VERSEBLOCKS (2)`: One verse per block
- `CHAPTERBLOCKS (3)`: One chapter per block
- `BOOKBLOCKS (4)`: One book per block

## RawLD Lexicon Format

Files: `module.dat`, `module.idx`

### Index File (.idx)

Variable-length records:

```
[4 bytes: offset to data]
[4 bytes: size of entry]
[null-terminated key string]
```

### Data File (.dat)

Raw text content at indexed offsets.

## RawGenBook Format

Files: `module.bdt`, `module.bds`

### Tree Index (.bdt)

Variable-length records per node:

| Offset | Size | Field | Description |
|--------|------|-------|-------------|
| 0 | 4 | offset | Offset in data file |
| 4 | 4 | size | Size of entry |
| 8 | 4 | parent | Parent node index (-1 for root) |
| 12 | var | name | Null-terminated key name |

### Data File (.bds)

Entry content at indexed offsets.

## Encryption

Some modules are encrypted using Sapphire cipher (stream cipher).

### CipherKey

The `.conf` file contains:
```
CipherKey=xxxxx
```

The key is XOR-based with position-dependent transformation.

## Configuration File Format

INI-style format:

```ini
[ModuleName]
Description=Module description
DataPath=./modules/texts/rawtext/module
ModDrv=RawText
Lang=en
Versification=KJV
SourceType=OSIS
Encoding=UTF-8
```

### Required Fields

- `ModDrv`: Driver type (RawText, zText, RawCom, etc.)
- `DataPath`: Path to module data files

### Common Optional Fields

- `Description`: Human-readable description
- `Lang`: ISO language code
- `Versification`: Verse system (KJV, Catholic, etc.)
- `SourceType`: Markup format (OSIS, ThML, GBF)
- `Encoding`: Text encoding (UTF-8, Latin1)
- `CipherKey`: Decryption key (if encrypted)
