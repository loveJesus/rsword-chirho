# rsword-chirho v0.2.0 Development Plan

> For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life. - John 3:16

## Critical Tasks

### 1. [ ] Implement 4-byte module variants
- RawText4Chirho - 4-byte verse offsets for large Bible modules
- RawCom4Chirho - 4-byte offsets for large commentaries
- RawLd4Chirho - 4-byte offsets for large lexicons
- ZText4Chirho - compressed 4-byte Bible modules
- ZCom4Chirho - compressed 4-byte commentaries
- ZLd4Chirho - compressed 4-byte lexicons

### 2. [ ] Add HTML entity escaping to filters
- Escape `<`, `>`, `&`, `"`, `'` in text content
- Prevent XSS in OSIS HTML filter
- Prevent XSS in ThML HTML filter
- Prevent XSS in GBF HTML filter

### 3. [ ] Implement additional versification systems
- Catholic canon (with Deuterocanonical books: Tobit, Judith, Wisdom, Sirach, Baruch, 1-2 Maccabees)
- Orthodox canon (additional books)
- LXX/Septuagint versification
- Synodal (Russian) versification
- Versification registry for dynamic loading

### 4. [ ] Implement RawGenBook module support
- Tree-key navigation (hierarchical structure)
- Content reading from general book modules
- GenBook entry iteration
- CLI tools for GenBook import/export

### 5. [ ] Add integration tests with real SWORD modules
- Download test modules from CrossWire
- Test KJV module reading
- Test compressed module reading
- Test lexicon lookups
- Test search functionality

## High Value Tasks

### 6. [ ] Complete Tantivy search integration
- Proper index creation/deletion
- Boolean search operators (AND, OR, NOT)
- Phrase proximity searches
- Search result ranking

### 7. [ ] Add LRU caching for verses
- Configurable cache size
- Cache frequently accessed verses
- Cache invalidation on module change

### 8. [ ] Add module info/stats tools
- `diatheke_chirho --info MODULE` for module metadata
- Verse count, size, compression ratio
- Module driver type, encoding, markup

## Medium Value Tasks

### 9. [ ] Add architecture documentation
- Module loading pipeline
- Storage format specifications
- Filter chain system

### 10. [ ] Add red-letter word support
- Detect words of Christ markers
- Wrap in CSS class for styling

### 11. [ ] Add lemma extraction in filters
- Parse lemma attributes from OSIS
- Display Strong's with lemma info

### 12. [ ] Add parallel search with rayon
- Optional feature flag
- Parallelize per-testament search

## Progress Tracking

| Task | Status | Notes |
|------|--------|-------|
| 4-byte modules | Pending | |
| HTML escaping | Pending | |
| Versifications | Pending | |
| RawGenBook | Pending | |
| Integration tests | Pending | |
| Tantivy search | Pending | |
| LRU caching | Pending | |
| Module info | Pending | |
| Architecture docs | Pending | |
| Red-letter | Pending | |
| Lemma extraction | Pending | |
| Parallel search | Pending | |
