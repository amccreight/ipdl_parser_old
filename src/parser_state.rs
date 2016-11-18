/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use ast::{Direction, Protocol, StructField, TranslationUnit, TypeSpec, UsingStmt};
use ipdl::parse_TranslationUnit;

use uncommenter::uncomment;

pub enum FileType {
    Protocol,
    Header,
}

impl FileType {
    pub fn from_file_path(file_path: &Path) -> Option<FileType> {
        if let Some(e) = file_path.extension() {
            if e == ".ipdlh" {
                Some(FileType::Header)
            } else {
                Some(FileType::Protocol)
            }
        } else {
            None
        }
    }
}


pub struct ParserState {
    pub include_dirs: Vec<PathBuf>,
    pub file_type: FileType,
    pub direction: Cell<Option<Direction>>,
}

impl ParserState {
    pub fn new(include_dirs: Vec<PathBuf>, file_type: FileType) -> ParserState {
        ParserState {
            include_dirs: include_dirs,
            file_type: file_type,
            direction: Cell::new(None),
        }
    }

    pub fn resolve_include_path(&self, file_path: &Path) -> Option<PathBuf> {
        // XXX The Python parser also checks '' for some reason.
        for d in &self.include_dirs {
            let mut p = d.clone();
            p.push(file_path);

            if let Ok(pb) = p.canonicalize() {
                return Some(pb)
            }
        }
        return None
    }
}

pub enum PreambleStmt {
    CxxInclude(String),
    Include(String),
    Using(UsingStmt),
}

pub enum TopLevelDecl {
    Struct(Vec<StructField>),
    Union(Vec<TypeSpec>),
    Protocol(Protocol),
}

pub fn parse(include_dirs: &Vec<PathBuf>, file_name: &str) -> TranslationUnit {

    // The file type and name are later enforced by the type checker.
    // This is just a hint to the parser.
    let file_path = Path::new(file_name);
    let file_type = FileType::from_file_path(&file_path).unwrap();

    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s = uncomment(&s);

    let parser_state = ParserState::new(include_dirs.clone(), file_type);
    parse_TranslationUnit(&parser_state, &s).unwrap()
}
