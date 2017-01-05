/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ast::*;


const BUILTIN_TYPES: &'static [ &'static str ] = &[
    // C types
    "bool",
    "char",
    "short",
    "int",
    "long",
    "float",
    "double",

    // stdint types
    "int8_t",
    "uint8_t",
    "int16_t",
    "uint16_t",
    "int32_t",
    "uint32_t",
    "int64_t",
    "uint64_t",
    "intptr_t",
    "uintptr_t",

    // stddef types
    "size_t",
    "ssize_t",

    // Mozilla types: "less" standard things we know how serialize/deserialize
    "nsresult",
    "nsString",
    "nsCString",
    "mozilla::ipc::Shmem",
    "mozilla::ipc::FileDescriptor"
];

fn builtin_from_string(tname: &str) -> TypeSpec {
    TypeSpec::new(QualifiedId::new_from_iter(tname.split("::")))
}

// XXX This may not really need to be a function.
fn make_builtin_using() -> Vec<TypeSpec> {
    let mut v = Vec::new();
    for t in BUILTIN_TYPES {
        v.push(builtin_from_string(t));
    };
    v
}

fn gather_decls(tu: &TranslationUnit) -> Result<(), String> {
    // XXX Pass in builtin_using?

    // For a protocol file, the filename should match the
    // protocol. (In the Python IPDL compiler, translation units have
    // a separate "name" field that is checked here, but for protocol
    // files the name is just the name of the protocol, and for
    // non-protocols the name is derived from the file name, so this
    // checking should be equivalent.)
    if let Some(ref p) = tu.protocol {

        let base_file_name = match tu.file_name.file_name() {
            Some(fs) => fs.to_str().unwrap(),
            None => return Err(String::from("File path has no file")),
        };

        let expected_file_name = p.0.name.id.clone() + ".ipdl";

        if base_file_name != expected_file_name {
            return Err(format!("expected file for translation unit `{}' to be named `{}'; instead it's named `{}'.",
                               tu.namespace.name.id, expected_file_name, base_file_name))
        }
    }

    return Ok(())
}



pub fn check(tu: &TranslationUnit) -> bool {
    match gather_decls(tu) {
        Ok(_) => true,
        Err(s) => {
            println!("Error: {}", s);
            false
        },
    }
}
