// Test module for checking verse indices

#[test]
fn check_indices_chirho() {
    use crate::versification_chirho::{kjv_chirho, TestamentChirho};

    let kjv_chirho = kjv_chirho();

    // Genesis 1:1
    let idx_chirho = kjv_chirho.calculate_index_chirho(TestamentChirho::OldChirho, 0, 1, 1).unwrap();
    println!("Genesis 1:1 index: {}", idx_chirho);

    // Genesis 1:2
    let idx_chirho = kjv_chirho.calculate_index_chirho(TestamentChirho::OldChirho, 0, 1, 2).unwrap();
    println!("Genesis 1:2 index: {}", idx_chirho);

    // John 3:16 (John is book index 3 in NT)
    let idx_chirho = kjv_chirho.calculate_index_chirho(TestamentChirho::NewChirho, 3, 3, 16).unwrap();
    println!("John 3:16 index: {}", idx_chirho);

    // Check totals
    println!("OT total: {}", kjv_chirho.testament_size_chirho(TestamentChirho::OldChirho));
    println!("NT total: {}", kjv_chirho.testament_size_chirho(TestamentChirho::NewChirho));
}

#[test]
fn debug_gbtspa_ot_chirho() {
    use std::path::Path;
    use crate::compression_chirho::create_compressor_chirho;
    use crate::storage_chirho::z_verse_chirho::ZVerseChirho;
    use crate::CompressionTypeChirho;

    let path_chirho = Path::new("/Users/hallelujah/.sword/modules/texts/ztext/gbtspa");

    if !path_chirho.exists() {
        println!("GBTSPA module not found, skipping test");
        return;
    }

    let comp_type_chirho = CompressionTypeChirho::ZipChirho;
    let compressor_chirho = create_compressor_chirho(comp_type_chirho);

    let mut storage_chirho = ZVerseChirho::open_with_comp_type_chirho(
        path_chirho,
        compressor_chirho,
        comp_type_chirho,
    ).expect("Failed to open storage");

    // Try to read OT verse at index 3 (Genesis 1:1)
    println!("Reading OT index 3 (Genesis 1:1)...");
    let verse_idx_chirho = storage_chirho.find_verse_index_chirho(1, 3);
    println!("Verse index result: {:?}", verse_idx_chirho);

    if let Ok(idx_chirho) = verse_idx_chirho {
        println!("  block_num: {}", idx_chirho.block_num_chirho);
        println!("  offset: {}", idx_chirho.offset_in_block_chirho);
        println!("  size: {}", idx_chirho.size_chirho);

        if !idx_chirho.is_empty_chirho() {
            // Try to read the block
            let block_result_chirho = storage_chirho.read_block_chirho(1, idx_chirho.block_num_chirho);
            println!("Block read result: {:?}", block_result_chirho.as_ref().map(|v| v.len()));

            if let Ok(block_chirho) = &block_result_chirho {
                let start_chirho = idx_chirho.offset_in_block_chirho as usize;
                let end_chirho = start_chirho + idx_chirho.size_chirho as usize;
                if end_chirho <= block_chirho.len() {
                    let text_chirho = String::from_utf8_lossy(&block_chirho[start_chirho..end_chirho]);
                    println!("  Text: {}", text_chirho);
                } else {
                    println!("  ERROR: end {} > block len {}", end_chirho, block_chirho.len());
                }
            }
        }
    }

    // Check indices 0-10 to understand the pattern
    println!("\nChecking indices 0-10:");
    for i_chirho in 0..10u32 {
        if let Ok(idx_chirho) = storage_chirho.find_verse_index_chirho(1, i_chirho) {
            if idx_chirho.size_chirho > 0 {
                if let Ok(block_chirho) = storage_chirho.read_block_chirho(1, idx_chirho.block_num_chirho) {
                    let start_chirho = idx_chirho.offset_in_block_chirho as usize;
                    let end_chirho = (start_chirho + idx_chirho.size_chirho as usize).min(start_chirho + 60);
                    let end_chirho = end_chirho.min(block_chirho.len());
                    let text_chirho = String::from_utf8_lossy(&block_chirho[start_chirho..end_chirho]);
                    // Safely truncate at char boundary
                    let display_text_chirho: String = text_chirho.chars().take(50).collect();
                    println!("Index {}: block={}, offset={}, size={} => {:?}...",
                        i_chirho, idx_chirho.block_num_chirho, idx_chirho.offset_in_block_chirho,
                        idx_chirho.size_chirho, display_text_chirho);
                }
            } else {
                println!("Index {}: empty", i_chirho);
            }
        }
    }

    // Also try NT for comparison
    println!("\nReading NT index 3067 (John 3:16)...");
    let verse_idx_chirho = storage_chirho.find_verse_index_chirho(2, 3067);
    println!("Verse index result: {:?}", verse_idx_chirho);

    if let Ok(idx_chirho) = verse_idx_chirho {
        println!("  block_num: {}", idx_chirho.block_num_chirho);
        println!("  offset: {}", idx_chirho.offset_in_block_chirho);
        println!("  size: {}", idx_chirho.size_chirho);
        if !idx_chirho.is_empty_chirho() {
            if let Ok(block_chirho) = storage_chirho.read_block_chirho(2, idx_chirho.block_num_chirho) {
                let start_chirho = idx_chirho.offset_in_block_chirho as usize;
                let end_chirho = start_chirho + idx_chirho.size_chirho as usize;
                if end_chirho <= block_chirho.len() {
                    let text_chirho = String::from_utf8_lossy(&block_chirho[start_chirho..end_chirho]);
                    println!("  Text: {}", text_chirho);
                }
            }
        }
    }
}
