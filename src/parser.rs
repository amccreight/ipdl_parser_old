/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as ParseError;

use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::{Cell, RefCell};
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};

use ast::{Direction, FileType, Protocol, StructField, TranslationUnit, TypeSpec, UsingStmt, Location};
use ipdl::parse_TranslationUnit;
use errors::Errors;
use utils::resolve_include_path;

use uncommenter::uncomment;


pub struct ParserState {
    pub include_dirs: Vec<PathBuf>,
    pub file_type: FileType,
    pub file_name: PathBuf,
    pub direction: Cell<Option<Direction>>,
    pub errors: RefCell<Errors>,
    newline_offsets: Vec<usize>,
}

impl ParserState {
    pub fn new(include_dirs: Vec<PathBuf>, file_type: FileType, file_name: &Path, newline_offsets: Vec<usize>) -> ParserState {
        ParserState {
            include_dirs: include_dirs,
            file_type: file_type,
            file_name: PathBuf::from(file_name),
            direction: Cell::new(None),
            errors: RefCell::new(Errors::none()),
            newline_offsets: newline_offsets,
        }
    }

    pub fn resolve_location(&self, byte_offset: usize) -> Location {
        match self.newline_offsets.binary_search(&byte_offset) {
            Ok(r) => panic!("Token should not start or end on a newline: {}, {}", byte_offset, r),
            Err(index) => {
                let file_name = self.file_name.clone();
                if index == 0 {
                    Location { file_name: file_name, lineno: 1, colno: byte_offset }
                } else {
                    let line_start_offset = self.newline_offsets[index - 1] + 1;
                    Location { file_name: file_name, lineno: index + 1, colno: byte_offset - line_start_offset }
                }
            }
        }
    }

    pub fn add_error(&self, loc: &Location, error: &str) {
        self.errors.borrow_mut().append_one(&loc, error);
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

pub fn parse_file(include_dirs: &Vec<PathBuf>, file_name: &Path) -> Result<TranslationUnit, String> {

    // The file type and name are later enforced by the type checker.
    // This is just a hint to the parser.
    let file_type = FileType::from_file_path(&file_name).unwrap();

    let mut f = File::open(file_name).unwrap();
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    text = uncomment(&text);

    // Create a vector of byte offsets of all the newlines in the input.
    // We'll use this to resolve (lineno, colno) pairs.
    let mut newline_offsets = Vec::new();
    let mut offset = 0;
    for line in text.split('\n') {
        offset += line.len();
        newline_offsets.push(offset);
        offset += 1;
    }

    let parser_state = ParserState::new(include_dirs.clone(), file_type, file_name, newline_offsets);
    parse_TranslationUnit(&parser_state, &text)
        .map_err(|e| {
            match e {
                ParseError::InvalidToken { location } => {
                    let loc = parser_state.resolve_location(location);
                    format!(":{} Unexpected token.", loc)
                },
                ParseError::UnrecognizedToken { token, expected: _ } => {
                    match token {
                        Some((start, t, _)) => {
                            let loc = parser_state.resolve_location(start);
                            format!(":{} Error: Unrecognized token `{}'.",
                                    loc, t.1)
                        },
                        None => String::from(" Unexpected EOL."),
                    }
                    // XXX Can anything useful be reported about |expected|?
                },
                ParseError::ExtraToken{ token } => {
                    let (start, t, _) = token;
                    let loc = parser_state.resolve_location(start);
                    format!(":{} Error: Extra token `{}'.",
                            loc, t.1)
                },
                ParseError::User{ error: _ } => {
                    panic!("Unexpected user error.");
                },
            }})
        .and_then(|tu| {
            let ref errors = *&parser_state.errors.borrow();
            errors.to_result().map(|_| tu)
        })
}


fn print_include_context(include_context: &Vec<PathBuf>) {
    for i in include_context {
        println!("  in file included from `{}':", i.display());
    }
}

pub fn parse(include_dirs: &Vec<PathBuf>, file_names: Vec<PathBuf>) -> Option<HashMap<PathBuf, TranslationUnit>> {
    let mut work_list : Vec<(PathBuf, Vec<PathBuf>)> = Vec::new();
    let mut parsed = HashMap::new();
    let mut visited = HashSet::new();

    // XXX For error reporting purposes, we should track the include
    // context of every file in the work list.

    for f in file_names {
        let fc = match resolve_include_path(&vec![PathBuf::from("")], &f) {
            Some(fc) => fc,
            None => {
                println!("Error: can't locate file specified on the command line `{}'", f.display());
                return None
            },
        };
        visited.insert(fc.clone());
        work_list.push((fc, Vec::new()));
    }

    while !work_list.is_empty() {
        let mut new_work_list = Vec::new();
        for (curr_file, include_context) in work_list {
            // XXX In the long run, we probably don't want to output this.
            println!("Parsing file {}", curr_file.display());
            let tu = match parse_file(&include_dirs, &curr_file) {
                Ok(tu) => tu,
                Err(message) => {
                    print_include_context(&include_context);
                    println!("{} {}", curr_file.display(), message);
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
                if visited.contains(&p) {
                    continue;
                }
                let mut new_context = include_context.clone();
                new_context.push(curr_file.clone());
                visited.insert(p.clone());
                new_work_list.push((p, new_context));
            }

            parsed.insert(curr_file.clone(), tu);
        }

        work_list = new_work_list;
    }

    Some(parsed)
}
