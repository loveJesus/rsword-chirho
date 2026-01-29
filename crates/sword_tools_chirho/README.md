# sword_tools_chirho

CLI tools for SWORD Bible modules - drop-in replacements for the original SWORD utilities.

> For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. - John 3:16

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
imp2gbs_chirho /path/to/module Book.imp

# Create from verse-per-line
vpl2mod_chirho /path/to/module verses.txt
```

### Module Conversion Tools

```bash
# Export to IMP format
mod2imp_chirho ModName > export.imp

# Export to OSIS XML
mod2osis_chirho ModName > export.xml

# Compress a raw module
mod2zmod_chirho RawMod /path/to/zmod 3

# Create search index
mkfastmod_chirho ModName
```

## License

GPL-2.0-or-later (same as SWORD library)
