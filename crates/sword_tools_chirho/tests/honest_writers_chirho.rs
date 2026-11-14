// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! The writers refuse instead of leaving a module no reader can open, and importing nothing is a failure
//! without a conf. Each test runs the built binaries in a private scratch directory.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const OSIS_CHIRHO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace"><osisText osisIDWork="Honest" xml:lang="en">
<div type="book" osisID="Gen"><chapter osisID="Gen.1"><verse osisID="Gen.1.1">In the beginning God created the heaven and the earth.</verse></chapter></div>
</osisText></osis>
"#;
const OSIS_NO_VERSES_CHIRHO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace"><osisText osisIDWork="Honest"><div type="book" osisID="Gen"/></osisText></osis>
"#;
const TEI_ENTRY_FREE_CHIRHO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<TEI xmlns="http://www.crosswire.org/2008/TEIOSIS/namespace"><text><body>
<entryFree n="agape"><orth>agape</orth><def>love of God</def></entryFree>
</body></text></TEI>
"#;

/// A private scratch directory, removed when the test ends.
struct ScratchChirho(PathBuf);

impl ScratchChirho {
    fn new_chirho(name_chirho: &str) -> Self {
        let path_chirho = std::env::temp_dir().join(format!("sword-tools-honest-{}-{name_chirho}", std::process::id()));
        let _ = fs::remove_dir_all(&path_chirho);
        fs::create_dir_all(&path_chirho).expect("scratch directory");
        Self(path_chirho)
    }

    fn input_chirho(&self, name_chirho: &str, text_chirho: &str) -> String {
        let path_chirho = self.0.join(name_chirho);
        fs::write(&path_chirho, text_chirho).expect("input file");
        path_chirho.to_string_lossy().into_owned()
    }

    /// An existing output directory, so a failure is never a missing-directory error.
    fn output_chirho(&self, relative_chirho: &str) -> String {
        let path_chirho = self.0.join(relative_chirho);
        fs::create_dir_all(&path_chirho).expect("output directory");
        path_chirho.to_string_lossy().into_owned()
    }

    fn run_chirho(&self, binary_chirho: &str, args_chirho: &[&str]) -> Output {
        std::process::Command::new(binary_chirho)
            .args(args_chirho)
            .env("HOME", &self.0)
            .env_remove("SWORD_PATH")
            .output()
            .expect("binary runs")
    }

    /// Every file under the scratch directory, minus the inputs.
    fn files_chirho(&self, inputs_chirho: &[&str]) -> Vec<PathBuf> {
        fn walk_chirho(dir_chirho: &Path, found_chirho: &mut Vec<PathBuf>) {
            for entry_chirho in fs::read_dir(dir_chirho).expect("readable scratch").flatten() {
                let path_chirho = entry_chirho.path();
                if path_chirho.is_dir() {
                    walk_chirho(&path_chirho, found_chirho);
                } else {
                    found_chirho.push(path_chirho);
                }
            }
        }
        let mut found_chirho = Vec::new();
        walk_chirho(&self.0, &mut found_chirho);
        found_chirho.retain(|path_chirho| !inputs_chirho.iter().any(|input_chirho| path_chirho == Path::new(input_chirho)));
        found_chirho
    }
}

impl Drop for ScratchChirho {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn stderr_chirho(output_chirho: &Output) -> String {
    String::from_utf8_lossy(&output_chirho.stderr).into_owned()
}

#[test]
fn test_imp2gbs_refuses_and_writes_nothing_chirho() {
    let scratch_chirho = ScratchChirho::new_chirho("imp2gbs");
    let imp_chirho = scratch_chirho.input_chirho("book.imp", "$$$/Book One\nThe first book.\n");
    let out_chirho = scratch_chirho.output_chirho("out/book");
    let run_chirho = scratch_chirho.run_chirho(env!("CARGO_BIN_EXE_imp2gbs_chirho"), &[&out_chirho, &imp_chirho]);
    assert!(!run_chirho.status.success());
    assert!(stderr_chirho(&run_chirho).contains("general book"), "{}", stderr_chirho(&run_chirho));
    assert_eq!(scratch_chirho.files_chirho(&[&imp_chirho]), Vec::<PathBuf>::new());
}

#[test]
fn test_mod2zmod_refuses_and_writes_nothing_chirho() {
    let scratch_chirho = ScratchChirho::new_chirho("mod2zmod");
    let out_chirho = scratch_chirho.output_chirho("out/compressed/");
    let run_chirho = scratch_chirho.run_chirho(env!("CARGO_BIN_EXE_mod2zmod_chirho"), &["KJV", &out_chirho, "3"]);
    assert!(!run_chirho.status.success());
    assert!(stderr_chirho(&run_chirho).contains("not implemented"), "{}", stderr_chirho(&run_chirho));
    assert_eq!(scratch_chirho.files_chirho(&[]), Vec::<PathBuf>::new());
}

#[test]
fn test_osis2mod_refuses_compression_and_other_versifications_chirho() {
    let scratch_chirho = ScratchChirho::new_chirho("osis2mod");
    let osis_chirho = scratch_chirho.input_chirho("bible.osis.xml", OSIS_CHIRHO);
    let out_chirho = scratch_chirho.output_chirho("out/parity");
    for refused_chirho in [["-z", "ZIP"], ["-v", "Catholic"]] {
        let run_chirho = scratch_chirho.run_chirho(env!("CARGO_BIN_EXE_osis2mod_chirho"), &[refused_chirho[0], refused_chirho[1], &out_chirho, &osis_chirho]);
        assert!(!run_chirho.status.success(), "{refused_chirho:?} was accepted");
        assert_eq!(scratch_chirho.files_chirho(&[&osis_chirho]), Vec::<PathBuf>::new(), "{refused_chirho:?} wrote files");
    }
}

#[test]
fn test_importing_nothing_fails_without_a_conf_chirho() {
    let scratch_chirho = ScratchChirho::new_chirho("zero");
    let cases_chirho = [
        ("vpl2mod_chirho", env!("CARGO_BIN_EXE_vpl2mod_chirho"), Some("-r"), "lines.vpl", "no reference on this line\n"),
        ("imp2vs_chirho", env!("CARGO_BIN_EXE_imp2vs_chirho"), None, "bible.imp", "$$$not a verse\ntext\n"),
        ("osis2mod_chirho", env!("CARGO_BIN_EXE_osis2mod_chirho"), None, "empty.osis.xml", OSIS_NO_VERSES_CHIRHO),
        ("imp2ld_chirho", env!("CARGO_BIN_EXE_imp2ld_chirho"), None, "empty.imp", ""),
        ("tei2mod_chirho", env!("CARGO_BIN_EXE_tei2mod_chirho"), None, "free.tei.xml", TEI_ENTRY_FREE_CHIRHO),
    ];
    for (name_chirho, binary_chirho, flag_chirho, input_name_chirho, input_text_chirho) in cases_chirho {
        let input_chirho = scratch_chirho.input_chirho(input_name_chirho, input_text_chirho);
        let out_chirho = scratch_chirho.output_chirho(&format!("{name_chirho}/parity/"));
        let mut args_chirho: Vec<&str> = flag_chirho.into_iter().collect();
        args_chirho.extend([out_chirho.as_str(), input_chirho.as_str()]);
        let run_chirho = scratch_chirho.run_chirho(binary_chirho, &args_chirho);
        assert!(!run_chirho.status.success(), "{name_chirho} succeeded with nothing to import");
        let reason_chirho = stderr_chirho(&run_chirho);
        assert!(reason_chirho.contains("No ") && !reason_chirho.contains("No such file"), "{name_chirho}: {reason_chirho}");
        let confs_chirho: Vec<PathBuf> = scratch_chirho.files_chirho(&[]).into_iter().filter(|path_chirho| path_chirho.extension().is_some_and(|ext_chirho| ext_chirho == "conf")).collect();
        assert_eq!(confs_chirho, Vec::<PathBuf>::new(), "{name_chirho} wrote a conf");
    }
}

#[test]
fn test_vpl2mod_imports_referenced_lines_chirho() {
    let scratch_chirho = ScratchChirho::new_chirho("vpl2mod");
    let vpl_chirho = scratch_chirho.input_chirho("bible.vpl", "Genesis 1:1 In the beginning God created the heaven and the earth.\nJohn 3:16 For God so loved the world.\n");
    let out_chirho = scratch_chirho.output_chirho("out/parity/");
    let run_chirho = scratch_chirho.run_chirho(env!("CARGO_BIN_EXE_vpl2mod_chirho"), &["-r", &out_chirho, &vpl_chirho]);
    assert!(run_chirho.status.success(), "{}", stderr_chirho(&run_chirho));
    assert!(String::from_utf8_lossy(&run_chirho.stdout).contains("Imported 2 verses"), "{}", String::from_utf8_lossy(&run_chirho.stdout));
}
