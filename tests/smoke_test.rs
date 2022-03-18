extern crate ipdl_parser;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

const BASE_PATH: [&'static str; 2] = [".", "tests"];
const OK_PATH: &'static str = "ok";
const ERROR_PATH: &'static str = "error";

// Tests in error/ are disabled because the given checking is not
// enabled yet.

const DISABLED_TESTS: &'static [&'static str] = &[];

// XXX This does not run efficiently. If A includes B, then we end up
// testing A and B two times each. At least for the non-error case we
// should be able to do them all together.

fn test_files(test_file_path: &str, should_pass: bool) {
    let mut path: PathBuf = BASE_PATH.iter().collect();
    path.push(test_file_path);

    let include_dirs = vec![path.clone()];

    let mut disabled_tests = HashSet::new();
    for f in DISABLED_TESTS {
        disabled_tests.insert(OsStr::new(f));
    }

    let entries = fs::read_dir(&path).expect("Should have the test file directory");
    for entry in entries {
        if let Ok(entry) = entry {
            let mut expected_result = should_pass;
            if !should_pass && disabled_tests.contains(entry.path().file_name().unwrap()) {
                println!(
                    "Expecting test to pass when it should fail {:?}",
                    entry.file_name()
                );
                expected_result = true;
            } else {
                println!("Testing {:?}", entry.file_name());
            }

            let file_names = vec![entry.path()];
            let ok = ipdl_parser::compiler::compile(&include_dirs, file_names);
            assert!(expected_result == ok);
        }
    }
}

#[test]
fn ok_tests() {
    test_files(OK_PATH, true);
}

#[test]
fn error_tests() {
    test_files(ERROR_PATH, false);
}
