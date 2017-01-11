extern crate ipdl_parser;

use ipdl_parser::ast::TranslationUnit;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const BASE_PATH: [&'static str; 2] = [".", "tests"];
const OK_PATH: &'static str = "ok";
const ERROR_PATH: &'static str = "error";

// error/twoprotocols.ipdl is disabled because of Issue #1.

// The other tests in error/ are disabled because the given checking
// is not enabled yet. Part of the issue is that the smoke tester only
// runs the parser.

fn test_enabled_files(test_file_path: &str, cb: &Fn(Option<HashMap<PathBuf, TranslationUnit>>)) {
    let mut path: PathBuf = BASE_PATH.iter().collect();
    path.push(test_file_path);

    let include_dirs = vec![path.clone()];

    let entries = fs::read_dir(&path).expect("Should have the test file directory");
    for entry in entries {
        if let Ok(entry) = entry {
            if entry.path().extension().unwrap() == "disabled" {
                println!("Skipping {:?}", entry.file_name());
            } else {
                println!("Testing {:?}", entry.file_name());
                let file_names = vec![entry.path()];
                let tus = ipdl_parser::parser::parse(&include_dirs, file_names);
                cb(tus);
            }
        }
    }
}

#[test]
fn ok_tests() {
    fn assert_is_some(tus: Option<HashMap<PathBuf, TranslationUnit>>) {
        assert!(tus.is_some());
    }

    let assert_function = assert_is_some;
    test_enabled_files(OK_PATH, &assert_function);
}

#[test]
fn error_tests() {
    fn assert_is_none(tus: Option<HashMap<PathBuf, TranslationUnit>>) {
        assert!(tus.is_none());
    }

    let assert_function = assert_is_none;
    test_enabled_files(ERROR_PATH, &assert_function);
}
