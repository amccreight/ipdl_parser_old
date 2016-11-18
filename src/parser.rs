/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::{HashMap, HashSet};
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

fn resolve_include_path(include_dirs: &Vec<PathBuf>, file_path: &Path) -> Option<PathBuf> {
    // XXX The Python parser also checks '' for some reason.
    for d in include_dirs {
        let mut p = d.clone();
        p.push(file_path);

        if let Ok(pb) = p.canonicalize() {
            return Some(pb)
        }
    }
    return None
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
        resolve_include_path(&self.include_dirs, file_path)
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

pub fn parse_file(include_dirs: &Vec<PathBuf>, file_name: &Path) -> TranslationUnit {

    // The file type and name are later enforced by the type checker.
    // This is just a hint to the parser.
    let file_type = FileType::from_file_path(&file_name).unwrap();

    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s = uncomment(&s);

    let parser_state = ParserState::new(include_dirs.clone(), file_type);
    parse_TranslationUnit(&parser_state, &s).unwrap()
}


pub fn parse(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> HashMap<PathBuf, TranslationUnit> {
    let mut work_list : HashSet<PathBuf> = HashSet::new();
    let mut parsed : HashMap<PathBuf, TranslationUnit> = HashMap::new();

    for f in file_names {
        work_list.insert(f);
    }

    while !work_list.is_empty() {
        let mut new_work_list = HashSet::new();
        for curr_file in &work_list {
            println!("Parsing file: {:?}", curr_file);
            let tu = parse_file(&include_dirs, curr_file);

            for i in &tu.includes {
                let p = Path::new(&i);
                if parsed.contains_key(p) || work_list.contains(p) {
                    continue;
                }
                new_work_list.insert(resolve_include_path(include_dirs, &p).unwrap());
            }

            parsed.insert(curr_file.clone(), tu);
        }

        work_list = new_work_list;
    }

    parsed
}
