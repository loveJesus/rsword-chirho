# sword_tools_chirho

Command-line tools for SWORD Bible modules, modeled on the SWORD utilities. They are not drop-in replacements: [Compatibility](#compatibility) lists the measured differences.

> For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. - John 3:16

## Compatibility

Measured on 2026-11-14 against SWORD 1.9.0 (diatheke 4.8) with the parity runners in the repository's `tools_chirho/` directory. "Both readers" means SWORD's diatheke and `diatheke_chirho`.

| Tool | Measured state |
|---|---|
| `diatheke_chirho` | Not a drop-in replacement. None of 131 invocations compared with diatheke gave identical output: 72 printed different output, 44 used arguments it rejects (such as a reference given as several words after `-k`), and 15 ended in an error. It reads Bible text from modules SWORD wrote, but it treats every key as a verse reference, so lexicon and general book lookups fail. |
| `osis2mod_chirho` | Writes RawText modules that both readers read. It refuses compression (`-z`) and any versification other than KJV, and ignores `-4`. |
| `imp2vs_chirho` | Writes RawText modules that both readers read. It refuses any versification other than KJV. |
| `vpl2mod_chirho` | Writes RawText modules that both readers read, from lines that start with a reference (`-r`) or from one verse per line. Without `-r` it starts at Genesis 1:1, while SWORD's vpl2mod starts at the module and testament introductions. It refuses any versification other than KJV. |
| `imp2ld_chirho` | Writes RawLD lexicons that SWORD reads. |
| `tei2mod_chirho` | Reads only `<entry n="...">` elements. SWORD's TEI dictionaries use `<entryFree>`, which it rejects without writing anything. It writes RawLD unless `-4` is given (SWORD's tei2mod writes RawLD4), and it ignores `-z`. |
| `imp2gbs_chirho` | Not available in this release. Its output was not a general book either reader could open, so it exits with an error and writes nothing. |
| `mod2zmod_chirho` | Not available in this release. Compression is not implemented, so it exits with an error and writes nothing. |
| `mod2imp_chirho` | Verse text matched SWORD's mod2imp for the module compared, but the files differ: keys are abbreviated (`Gen 50:1`, not `Genesis 50:1`), and empty entries and introductions are left out. |
| `mod2osis_chirho` | The OSIS header differs from SWORD's mod2osis. For a Hindi module it declared `xml:lang="en"`, and it uses the module name as the title instead of the description. |
| `mkfastmod_chirho` | Builds a Tantivy index, which SWORD cannot use. |
| `installmgr_chirho` | Not compared, because a comparison needs the network. |

Every writer exits with an error when it imports nothing.

## Installation

```bash
cargo install sword_tools_chirho
```

## Tools Included

### diatheke_chirho - Bible Text Lookup/Search

```bash
# Look up a verse
diatheke_chirho -b KJV -k "John 3:16"

# Search for text
diatheke_chirho -b KJV -s phrase -k "love"

# Output as HTML
diatheke_chirho -b KJV -f html -k "Gen 1:1-5"
```

### installmgr_chirho - Module Installation

```bash
# Initialize config
installmgr_chirho --init

# List remote sources
installmgr_chirho -s

# Refresh module list from CrossWire
installmgr_chirho -r CrossWire

# List available modules
installmgr_chirho --rl CrossWire

# Install a module
installmgr_chirho --ri CrossWire KJV

# List installed modules
installmgr_chirho -l

# Uninstall a module
installmgr_chirho -u ModuleName
```

### Module Creation Tools

```bash
# Create module from OSIS XML
osis2mod_chirho /path/to/module OSIS.xml

# Create lexicon from TEI XML
tei2mod_chirho /path/to/module TEI.xml

# Create module from IMP format
imp2vs_chirho /path/to/module Bible.imp
imp2ld_chirho /path/to/module Lexicon.imp

# Create from verse-per-line: one verse per line from Genesis 1:1, or lines that start with a reference
vpl2mod_chirho /path/to/module verses.txt
vpl2mod_chirho -r /path/to/module verses-with-references.txt
```

### Module Conversion Tools

```bash
# Export to IMP format
mod2imp_chirho ModName > export.imp

# Export to OSIS XML
mod2osis_chirho ModName > export.xml

# Create search index
mkfastmod_chirho ModName
```

`imp2gbs_chirho` and `mod2zmod_chirho` are not available in this release; see [Compatibility](#compatibility).

## License

GPL-2.0-or-later (same as SWORD library)
