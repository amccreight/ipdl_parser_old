extern crate ipdl_parser;

use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;
use std::ffi::OsStr;

const BASE_PATH: [&'static str; 2] = [".", "tests"];
const OK_PATH: &'static str = "ok";
const ERROR_PATH: &'static str = "error";

// Tests in error/ are disabled because the given checking is not
// enabled yet.

const DISABLED_TESTS: &'static [ &'static str ] = &[
    "compressCtor.ipdl",
    "compressCtorManagee.ipdl",
    "conflictProtocolMsg.ipdl",
    "cyclecheck_Child.ipdl",
    "cyclecheck_Parent.ipdl",
    "cyclecheck_Grandchild.ipdl",
    "intrMessageCompress.ipdl",
    "managedNoCtor.ipdl",
    "managedNoDtor.ipdl",
    "managerNoCtor.ipdl",
    "managerNoDtor.ipdl",
    "manageSelfToplevel.ipdl",
    "messageNoDirection.ipdl",
    "multimanDupMgrs.ipdl",
    "multimanDupMgrsMgr.ipdl",
    "multimanNonexistentMgrs.ipdl",
    "mutualRecStruct.ipdl",
    "mutualRecStructUnion.ipdl",
    "noEmptyToplevel.ipdl",
    "Nullable2.ipdl",
    "Nullable.ipdl",
    "redeclMessage.ipdl",
    "redeclParamReturn.ipdl",
    "shmem.ipdl",
    "structRedecl.ipdl",
    "structUnknownField.ipdl",
    "syncMessageCompress.ipdl",
    "syncParentToChild.ipdl",
    "tooWeakIntrAsync.ipdl",
    "tooWeakIntrSync.ipdl",
    "tooWeakSyncAsync.ipdl",
    "undeclParamType.ipdl",
    "undeclProtocol.ipdl",
    "undeclReturnType.ipdl",
    "undefMutualRecStruct.ipdl",
    "undefMutualRecStructUnion.ipdl",
    "undefMutualRecUnion.ipdl",
    "undefSelfRecStruct.ipdl",
    "undefSelfRecUnion.ipdl",
];

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

            if !should_pass && disabled_tests.contains(entry.path().file_name().unwrap()) {
                println!("Skipping {:?}", entry.file_name());
                // XXX What should happen here is that instead of
                // continuing, we check to make sure that the test
                // passes. That way, if somebody fixes the IPDL
                // compiler, we'll get an error.
                continue;
            } else {
                println!("Testing {:?}", entry.file_name());
            }

            let file_names = vec![entry.path()];
            let ok = ipdl_parser::compiler::compile(&include_dirs, file_names);
            assert!(should_pass == ok);
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
