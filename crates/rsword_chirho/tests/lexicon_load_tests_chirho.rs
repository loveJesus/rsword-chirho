// For God so loved the world, that he gave his only begotten Son,
// that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! End-to-end loading of a compressed lexicon (zLD) through the public `SwMgr`
//! API. This covers the path that was previously broken: a lexicon `DataPath`
//! ends in a file basename (`.../dict`), so the module directory must be resolved
//! from the parent, and the zStr storage must be decoded in SWORD's real format.
//!
//! The module is synthesized on disk in the genuine on-disk layout (no network),
//! mirroring how a real CrossWire module such as StrongsGreek is stored.

use std::fs;
use std::path::Path;

use rsword_chirho::compression_chirho::{CompressorChirho, ZipCompressorChirho};
use rsword_chirho::manager_chirho::SwMgrChirho;

/// Write a single-block zLD module (`<base>/modules/lexdict/zld/<name>/dict.*`)
/// plus its `mods.d/<name>.conf`, in the real SWORD zStr format.
fn write_zld_module_chirho(base_chirho: &Path, name_chirho: &str, entries_chirho: &[(&str, &str)]) {
    let data_dir_chirho = base_chirho.join("modules/lexdict/zld").join(name_chirho.to_lowercase());
    fs::create_dir_all(&data_dir_chirho).unwrap();
    fs::create_dir_all(base_chirho.join("mods.d")).unwrap();

    // Decompressed block: u32 count + (u32 offset, u32 size) * count + entry data.
    let table_len_chirho = 4 + entries_chirho.len() * 8;
    let mut table_chirho: Vec<u8> = (entries_chirho.len() as u32).to_le_bytes().to_vec();
    let mut data_chirho: Vec<u8> = Vec::new();
    for (_k_chirho, v_chirho) in entries_chirho {
        let off_chirho = (table_len_chirho + data_chirho.len()) as u32;
        table_chirho.extend_from_slice(&off_chirho.to_le_bytes());
        table_chirho.extend_from_slice(&(v_chirho.len() as u32).to_le_bytes());
        data_chirho.extend_from_slice(v_chirho.as_bytes());
    }
    let mut block_chirho = table_chirho;
    block_chirho.extend_from_slice(&data_chirho);
    let compressed_chirho = ZipCompressorChirho::new_chirho().compress_chirho(&block_chirho).unwrap();

    fs::write(data_dir_chirho.join("dict.zdt"), &compressed_chirho).unwrap();

    // .zdx: one block (offset 0, compressed size).
    let mut zdx_chirho = 0u32.to_le_bytes().to_vec();
    zdx_chirho.extend_from_slice(&(compressed_chirho.len() as u32).to_le_bytes());
    fs::write(data_dir_chirho.join("dict.zdx"), &zdx_chirho).unwrap();

    // .dat key records + .idx (offset, size) into .dat.
    let mut dat_chirho: Vec<u8> = Vec::new();
    let mut idx_chirho: Vec<u8> = Vec::new();
    for (i_chirho, (k_chirho, _v_chirho)) in entries_chirho.iter().enumerate() {
        let rec_off_chirho = dat_chirho.len() as u32;
        let mut rec_chirho = k_chirho.as_bytes().to_vec();
        rec_chirho.extend_from_slice(b"\r\n");
        rec_chirho.extend_from_slice(&0u32.to_le_bytes()); // block 0
        rec_chirho.extend_from_slice(&(i_chirho as u32).to_le_bytes()); // index in block
        idx_chirho.extend_from_slice(&rec_off_chirho.to_le_bytes());
        idx_chirho.extend_from_slice(&(rec_chirho.len() as u32).to_le_bytes());
        dat_chirho.extend_from_slice(&rec_chirho);
        dat_chirho.extend_from_slice(b"\r\n");
    }
    fs::write(data_dir_chirho.join("dict.dat"), &dat_chirho).unwrap();
    fs::write(data_dir_chirho.join("dict.idx"), &idx_chirho).unwrap();

    let conf_chirho = format!(
        "[{name}]\nDataPath=./modules/lexdict/zld/{lower}/dict\nModDrv=zLD\n\
         CompressType=ZIP\nLang=grc\nEncoding=UTF-8\nSourceType=Plain\n\
         Description=Test Greek Lexicon\n",
        name = name_chirho,
        lower = name_chirho.to_lowercase(),
    );
    fs::write(base_chirho.join("mods.d").join(format!("{}.conf", name_chirho.to_lowercase())), conf_chirho).unwrap();
}

#[test]
fn test_swmgr_loads_and_reads_zld_lexicon_chirho() {
    let temp_chirho = tempfile::TempDir::new().unwrap();
    let base_chirho = temp_chirho.path();

    // Zero-padded numeric keys, sorted, exactly like a real Strong's lexicon.
    let entries_chirho = [
        ("00026", "agape - love"),
        ("00059", "agorazo - to buy"),
        ("02316", "theos - God"),
    ];
    write_zld_module_chirho(base_chirho, "TestStrongs", &entries_chirho);

    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(base_chirho);
    mgr_chirho.load_modules_chirho().unwrap();

    // The module is discovered and recognized as a lexicon.
    assert!(mgr_chirho.get_module_names_chirho().contains(&"TestStrongs"));

    // Loading must succeed even though DataPath ends in the "dict" file prefix.
    let module_chirho = mgr_chirho
        .load_module_chirho("TestStrongs")
        .expect("zLD lexicon should load via SwMgr");
    assert!(module_chirho.driver_type_chirho.is_lexicon_chirho());

    // Exact native key reads the decompressed entry.
    assert!(module_chirho.read_entry_chirho("00026").unwrap().contains("agape"));
    // Strong's-style keys resolve through the zero-pad fallback.
    assert!(module_chirho.read_entry_chirho("G26").unwrap().contains("agape"));
    assert!(module_chirho.read_entry_chirho("26").unwrap().contains("agape"));
    assert!(module_chirho.read_entry_chirho("G2316").unwrap().contains("theos"));
}
