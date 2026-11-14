<!-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV) -->

# Changelog

Notable changes to `rsword_chirho` and `sword_tools_chirho`. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow [Semantic Versioning](https://semver.org/).

## [0.3.1] - 2026-11-14

### Fixed

- Compressed lexicons and dictionaries (zLD, zLD4) now open and read through `SwMgrChirho`. The loader resolves a lexicon's data path to its directory, as it already did for general books, and the zStr reader follows SWORD's real layout (`.idx` to `.dat` to `.zdx` to a compressed block) instead of failing with "corrupt deflate stream".
- Uncompressed lexicons (RawLD, RawLD4) read real modules. RawLD uses 6-byte index records and RawLD4 8-byte records, and entries split on the newline SWORD writes. RawLD daily devotionals such as "Daily" no longer fail with "failed to fill whole buffer".
- Module downloads larger than 10 MB complete. `download_chirho` streams the response to disk instead of buffering it, so memory stays flat, and catalog fetches allow up to 64 MB.
- The `InstallMgrChirho` example in the README and module docs matches the real API: `new_chirho(dir)`, `init_chirho()`, `refresh_source_chirho(source)`, then `install_module_chirho`.

### Added

- `InstallMgrChirho::install_from_file_chirho` installs a module from a local `.zip` already on disk.
- `InstallMgrChirho::list_remote_module_infos_chirho` lists a source's modules with name, version, description, language and driver in one call.
- Strong's lookups in lexicons accept `G25`, `25` and `00025` alike, through a zero-padded fallback; word-keyed dictionaries are unchanged.

### Notes

- `sword_tools_chirho` was last published at 0.2.0. Version 0.3.1 is its first release since then; there was no 0.3.0 tools release.

## [0.3.0] - 2026-02-05

`rsword_chirho` 0.3.0 on crates.io. Earlier versions are in the git history and the `v0.1.0` to `v0.2.2` tags.
