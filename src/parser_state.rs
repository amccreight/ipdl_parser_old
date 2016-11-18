/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::cell::Cell;
use std::path::{Path, PathBuf};

use ast::{Direction, Protocol, StructField, TypeSpec, UsingStmt};

pub struct ParserState {
    pub include_dirs: Vec<PathBuf>,
    pub direction: Cell<Option<Direction>>,
}

impl ParserState {
    pub fn new(include_dirs: Vec<PathBuf>) -> ParserState {
        ParserState { include_dirs: include_dirs, direction: Cell::new(None) }
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
