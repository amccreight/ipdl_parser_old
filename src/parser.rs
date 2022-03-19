/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use ast::{
    Direction, FileType, Location, Protocol, StructField, TUId, TranslationUnit, TypeSpec,
    UsingStmt,
};
use errors::Errors;
use ipdl::TranslationUnitParser;

use uncommenter::uncomment;

pub struct TUIdFileMap {
    next_id: TUId,
    file_ids: HashMap<PathBuf, TUId>,
    id_files: HashMap<TUId, PathBuf>,
}

impl TUIdFileMap {
    fn new() -> TUIdFileMap {
        TUIdFileMap {
            next_id: 0,
            file_ids: HashMap::new(),
            id_files: HashMap::new(),
        }
    }

    fn resolve_file_name(&mut self, pb: &PathBuf) -> TUId {
        if let Some(id) = self.file_ids.get(pb).cloned() {
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;
        self.file_ids.insert(pb.clone(), id);
        self.id_files.insert(id, pb.clone());
        id
    }

    fn id_file_name(&self, tuid: &TUId) -> &PathBuf {
        self.id_files.get(tuid).unwrap()
    }
}

pub struct IncludeResolver {
    include_dirs: Vec<PathBuf>,
    include_files: HashMap<String, PathBuf>,
    id_file_map: TUIdFileMap,
}

impl IncludeResolver {
    fn new(include_dirs: Vec<PathBuf>) -> IncludeResolver {
        IncludeResolver {
            include_dirs: include_dirs,
            include_files: HashMap::new(),
            id_file_map: TUIdFileMap::new(),
        }
    }

    fn resolve_include(&mut self, include: &str) -> Option<TUId> {
        if let Some(ref pb) = self.include_files.get(include) {
            return Some(self.id_file_map.resolve_file_name(&pb));
        }

        // XXX The Python parser also checks '' for some reason.
        let file_path = Path::new(&include);
        for d in &self.include_dirs {
            let mut p = d.clone();
            p.push(file_path);

            if p.exists() {
                if let Ok(pb) = p.canonicalize() {
                    self.include_files.insert(String::from(include), pb.clone());
                    return Some(self.id_file_map.resolve_file_name(&pb));
                }
            }
        }

        None
    }
}

pub struct ParserState<'a> {
    include_resolver: &'a RefCell<IncludeResolver>,
    pub file_type: FileType,
    pub file_name: PathBuf,
    pub direction: Cell<Option<Direction>>,
    errors: RefCell<Errors>,
    newline_offsets: Vec<usize>,
}

impl<'a> ParserState<'a> {
    pub fn new(
        include_resolver: &'a RefCell<IncludeResolver>,
        file_type: FileType,
        file_name: &Path,
        newline_offsets: Vec<usize>,
    ) -> ParserState<'a> {
        ParserState {
            include_resolver: include_resolver,
            file_type: file_type,
            file_name: PathBuf::from(file_name),
            direction: Cell::new(None),
            errors: RefCell::new(Errors::none()),
            newline_offsets: newline_offsets,
        }
    }

    pub fn resolve_include_path(&self, loc: &Location, file: &str) -> TUId {
        if let Some(tuid) = self.include_resolver.borrow_mut().resolve_include(&file) {
            return tuid;
        }

        self.add_error(&loc, &format!("can't locate include file `{}'", &file));
        -1 // Dummy id
    }

    pub fn resolve_location(&self, byte_offset: usize) -> Location {
        match self.newline_offsets.binary_search(&byte_offset) {
            Ok(r) => panic!(
                "Token should not start or end on a newline: {}, {}",
                byte_offset, r
            ),
            Err(index) => {
                let file_name = self.file_name.clone();
                if index == 0 {
                    Location {
                        file_name: file_name,
                        lineno: 1,
                        colno: byte_offset,
                    }
                } else {
                    let line_start_offset = self.newline_offsets[index - 1] + 1;
                    Location {
                        file_name: file_name,
                        lineno: index + 1,
                        colno: byte_offset - line_start_offset,
                    }
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
    Include(TUId),
    Using(UsingStmt),
}

pub enum TopLevelDecl {
    Struct(Vec<StructField>),
    Union(Vec<TypeSpec>),
    Protocol(Protocol),
}

pub fn parse_file(
    include_resolver: &RefCell<IncludeResolver>,
    file_name: &PathBuf,
) -> Result<TranslationUnit, String> {
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

    let parser_state = ParserState::new(&include_resolver, file_type, file_name, newline_offsets);
    TranslationUnitParser::new()
        .parse(&parser_state, &text)
        .map_err(|e| {
            match e {
                ParseError::InvalidToken { location } => {
                    let loc = parser_state.resolve_location(location);
                    format!(":{} Unexpected token.", loc)
                    // XXX This does not include a token, so we can't precisely
                    // match the Python compiler's error.
                }
                ParseError::UnrecognizedToken { token, expected: _ } => {
                    let (start, t, _) = token;
                    let loc = parser_state.resolve_location(start);
                    format!(":{} error: bad syntax near `{}'", loc, t.1)
                    // XXX Can anything useful be reported about |expected|?
                }
                ParseError::UnrecognizedEOF {
                    location: _,
                    expected: _,
                } => {
                    format!("error: bad syntax near `???'")
                }
                ParseError::ExtraToken { token } => {
                    let (start, t, _) = token;
                    let loc = parser_state.resolve_location(start);
                    format!(":{} Error: Extra token `{}'.", loc, t.1)
                }
                ParseError::User { error: _ } => {
                    panic!("Unexpected user error.");
                }
            }
        })
        .and_then(|tu| {
            let ref errors = *&parser_state.errors.borrow();
            errors.to_result().map(|_| tu)
        })
}

fn include_context_to_string(include_context: &Vec<PathBuf>) -> String {
    let mut context = String::new();
    for pb in include_context {
        context.push_str(&format!("  in file included from `{}':\n", pb.display()));
    }
    context
}

fn parse_internal(
    include_dirs: &Vec<PathBuf>,
    file_names: Vec<PathBuf>,
    ignore_errors: bool,
) -> Result<HashMap<TUId, TranslationUnit>, String> {
    let mut work_list: Vec<(PathBuf, Vec<PathBuf>)> = Vec::new();
    let mut parsed = HashMap::new();
    let mut visited = HashSet::new();

    let mut include_resolver = IncludeResolver::new(include_dirs.clone());
    for f in file_names {
        let fc = match f.canonicalize() {
            Ok(fc) => fc,
            Err(_) => {
                if ignore_errors {
                    continue;
                } else {
                    return Err(format!(
                        "error: can't locate file specified on the command line `{}'",
                        f.display()
                    ));
                }
            }
        };

        let fid = include_resolver.id_file_map.resolve_file_name(&fc);
        visited.insert(fid);
        work_list.push((fc, Vec::new()));
    }

    let include_resolver_cell = RefCell::new(include_resolver);

    while !work_list.is_empty() {
        let mut new_work_list = Vec::new();
        for (curr_file, include_context) in work_list {
            // XXX In the long run, we probably don't want to output this.
            println!("Parsing file {}", curr_file.display());
            let tu = match parse_file(&include_resolver_cell, &curr_file) {
                Ok(tu) => tu,
                Err(message) => {
                    if ignore_errors {
                        continue;
                    } else {
                        let mut new_message = include_context_to_string(&include_context);
                        new_message.push_str(&format!("{} {}", curr_file.display(), message));
                        return Err(new_message);
                    }
                }
            };

            for i in &tu.includes {
                if visited.contains(i) {
                    continue;
                }
                let mut new_context = include_context.clone();
                new_context.push(curr_file.clone());
                visited.insert(i.clone());
                new_work_list.push((
                    include_resolver_cell
                        .borrow()
                        .id_file_map
                        .id_file_name(i)
                        .clone(),
                    new_context,
                ));
            }

            let curr_id = include_resolver_cell
                .borrow_mut()
                .id_file_map
                .resolve_file_name(&curr_file);
            parsed.insert(curr_id, tu);
        }

        work_list = new_work_list;
    }

    Ok(parsed)
}

pub fn parse_with_errors(
    include_dirs: &Vec<PathBuf>,
    file_names: Vec<PathBuf>,
) -> Result<HashMap<TUId, TranslationUnit>, String> {
    parse_internal(include_dirs, file_names, /* ignore_errors = */ false)
}

pub fn parse(
    include_dirs: &Vec<PathBuf>,
    file_names: Vec<PathBuf>,
) -> Option<HashMap<TUId, TranslationUnit>> {
    parse_internal(include_dirs, file_names, /* ignore_errors = */ true).ok()
}
