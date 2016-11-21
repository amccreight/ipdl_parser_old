/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as ParseError;

use std::collections::HashMap;
use std::cell::Cell;
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use ast::{Direction, FileType, Protocol, StructField, TranslationUnit, TypeSpec, UsingStmt};
use ipdl::parse_TranslationUnit;

use uncommenter::uncomment;


pub struct ParserState {
    pub include_dirs: Vec<PathBuf>,
    pub file_type: FileType,
    pub file_name: PathBuf,
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
    pub fn new(include_dirs: Vec<PathBuf>, file_type: FileType, file_name: &Path) -> ParserState {
        ParserState {
            include_dirs: include_dirs,
            file_type: file_type,
            file_name: PathBuf::from(file_name),
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

// Line numbering starts at 1, column numbering starts at 0.
fn location_from_char_offsets(file: &str, offsets: Vec<usize>) -> Vec<(usize, usize)>
{
    let mut curr_start = 0;
    let mut line_number = 1;
    let mut offsets_iter = offsets.iter();
    let mut curr_offset = *offsets_iter.next().unwrap();
    let mut locations = Vec::new();

    for l in file.lines() {
        assert!(curr_offset >= curr_start);
        let new_start = curr_start + l.len() + 1;
        while curr_offset < new_start {
            locations.push((line_number, curr_offset - curr_start));
            match offsets_iter.next() {
                Some(new_offset) => {
                    assert!(*new_offset >= curr_offset);
                    curr_offset = *new_offset;
                },
                None => return locations
            }
        }
        line_number += 1;
        curr_start = new_start;
    }

    panic!("Failed to find char offset");
}

pub fn parse_file(include_dirs: &Vec<PathBuf>, file_name: &Path) -> Result<TranslationUnit, String> {

    // The file type and name are later enforced by the type checker.
    // This is just a hint to the parser.
    let file_type = FileType::from_file_path(&file_name).unwrap();

    let mut f = File::open(file_name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s = uncomment(&s);

    let parser_state = ParserState::new(include_dirs.clone(), file_type, file_name);
    parse_TranslationUnit(&parser_state, &s)
        .map_err(|e| {
            match e {
                ParseError::InvalidToken { location } => {
                    let (line, col) = location_from_char_offsets(&s, vec!(location))[0];
                    format!(":{}:{} Unexpected token.", line, col)
                },
                ParseError::UnrecognizedToken { token, expected: _ } => {
                    match token {
                        Some((start, t, _)) => {
                            let start_line = location_from_char_offsets(&s, vec!(start))[0].0;
                            format!(":{} Error: Unrecognized token `{}'.",
                                    start_line, t.1)
                        },
                        None => String::from(" Unexpected EOL."),
                    }
                    // XXX Can anything useful be reported about |expected|?
                },
                ParseError::ExtraToken{ token } => {
                    let (start, t, _) = token;
                    let start_line = location_from_char_offsets(&s, vec!(start))[0].0;
                    format!(":{} Error: Extra token `{}'.",
                            start_line, t.1)
                },
                ParseError::User{ error: _ } => {
                    panic!("Unexpected user error.");
                },
            }})
}


fn print_include_context(include_context: &Vec<PathBuf>) {
    for i in include_context {
        println!("  in file included from `{}':", i.display());
    }
}

pub fn parse(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> Option<HashMap<PathBuf, TranslationUnit>> {
    let mut work_list : HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let mut parsed = HashMap::new();

    // XXX For error reporting purposes, we should track the include
    // context of every file in the work list.

    for f in file_names {
        let fc = match resolve_include_path(&vec![PathBuf::from("")], &f) {
            Some(fc) => fc,
            None => {
                println!("Error: can't locate file specified on the comamnd line `{}'", f.display());
                return None
            },
        };
        work_list.insert(fc, Vec::new());
    }

    while !work_list.is_empty() {
        let mut new_work_list = HashMap::new();
        for (curr_file, include_context) in &work_list {
            // XXX In the long run, we probably don't want to output this.
            println!("Parsing file {}", curr_file.display());
            let tu = match parse_file(&include_dirs, curr_file) {
                Ok(tu) => tu,
                Err(message) => {
                    print_include_context(&include_context);
                    println!("{}{}", curr_file.display(), message);
                    return None
                }
            };

            for i in &tu.includes {
                let p = match resolve_include_path(include_dirs, Path::new(&i)) {
                    Some(p) => p,
                    None => {
                        print_include_context(&include_context);
                        println!("Error: can't locate include file `{}'", i);
                        return None
                    },
                };
                if parsed.contains_key(&p) || work_list.contains_key(&p) {
                    continue;
                }
                let mut new_context = include_context.clone();
                new_context.push(curr_file.clone());
                new_work_list.insert(p, new_context);
            }

            parsed.insert(curr_file.clone(), tu);
        }

        work_list = new_work_list;
    }

    Some(parsed)
}
