/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::path::{Path, PathBuf};
use std::fmt;

#[derive(Debug)]
pub struct QualifiedId {
    pub base_id: Identifier,
    pub quals: Vec<String>,
}

impl QualifiedId {
    pub fn new(base: Identifier) -> QualifiedId {
        QualifiedId { base_id: base, quals: Vec::new() }
    }

    pub fn qualify(mut self, id: Identifier) -> QualifiedId {
        self.quals.push(self.base_id.id);
        self.base_id = id;
        self
    }

    pub fn new_from_iter<'a, I> (mut ids: I) -> QualifiedId
        where I: Iterator<Item=&'a str>
    {
        let loc = Location { lineno: 0, colno: 0 };
        let mut qual_id = QualifiedId::new(Identifier::new(String::from(ids.next().unwrap()), loc.clone()));
        for i in ids {
            qual_id = qual_id.qualify(Identifier::new(String::from(i), loc.clone()));
        }
        qual_id
    }
}

#[derive(Debug)]
pub struct TypeSpec {
    spec: QualifiedId,
    array: bool,
    nullable: bool,
}

impl TypeSpec {
    pub fn new(spec: QualifiedId) -> TypeSpec {
        TypeSpec { spec: spec, array: false, nullable: false }
    }

    pub fn set_array(mut self, is_array: bool) -> TypeSpec {
        self.array = is_array;
        self
    }

    pub fn set_nullable(mut self, is_nullable: bool) -> TypeSpec {
        self.nullable = is_nullable;
        self
    }
}

#[derive(Debug)]
pub struct Param {
    name: Identifier,
    type_spec: TypeSpec,
}

impl Param {
    pub fn new(type_spec: TypeSpec, name: Identifier) -> Param {
        Param { name: name, type_spec: type_spec }
    }
}

#[derive(Debug)]
pub struct StructField {
    type_spec: TypeSpec,
    name: Identifier,
}

impl StructField {
    pub fn new(ty: TypeSpec, name: Identifier) -> StructField {
        StructField { type_spec: ty, name: name }
    }
}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub name: Identifier,
    pub namespaces: Vec<String>,
}

impl Namespace {
    pub fn new(name: Identifier) -> Namespace {
        Namespace { name: name, namespaces: Vec::new() }
    }

    pub fn add_outer_namespace(&mut self, namespace: &str) {
        self.namespaces.insert(0, String::from(namespace));
    }

    pub fn qname(&self) -> QualifiedId {
        QualifiedId { base_id: self.name.clone(), quals: self.namespaces.clone() }
    }
}

#[derive(Debug)]
pub enum Compress {
    None,
    Enabled,
    All,
}

#[derive(Debug)]
pub enum MessageModifier {
    Verify,
    Compress(Compress),
}

#[derive(Debug, PartialEq)]
pub enum SendSemantics {
    Async,
    Sync,
    Intr,
}

#[derive(Debug)]
pub enum Nesting {
    None,
    InsideSync,
    InsideCpow,
}

#[derive(Debug)]
pub enum Priority {
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    ToParent,
    ToChild,
    ToParentOrChild,
}

#[derive(Debug, Clone)]
pub struct Location {
    pub lineno: usize,
    pub colno: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.lineno, self.colno)
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: String,
    pub loc: Location,
}

impl Identifier {
    pub fn new(name: String, loc: Location) -> Identifier {
        Identifier {
            id: name,
            loc: loc,
        }
    }
}

#[derive(Debug)]
pub struct MessageDecl {
    pub name: Identifier,
    pub send_semantics: SendSemantics,
    pub nested: Nesting,
    pub prio: Priority,
    pub direction: Direction,
    in_params: Vec<Param>,
    out_params: Vec<Param>,
    compress: Compress,
    verify: bool,
}

impl MessageDecl {
    pub fn new(name: Identifier) -> MessageDecl {
        MessageDecl {
            name: name,
            send_semantics: SendSemantics::Async,
            nested: Nesting::None,
            prio: Priority::Normal,
            direction: Direction::ToParent,
            in_params: Vec::new(),
            out_params: Vec::new(),
            compress: Compress::None,
            verify: false,
        }
    }

    pub fn add_in_params(&mut self, mut in_params: Vec<Param>) {
        self.in_params.append(&mut in_params);
    }

    pub fn add_out_params(&mut self, mut out_params: Vec<Param>) {
        self.out_params.append(&mut out_params);
    }

    pub fn add_modifiers(&mut self, modifiers: Vec<MessageModifier>) {
        for modifier in modifiers {
            match modifier {
                MessageModifier::Compress(c) => self.compress = c,
                MessageModifier::Verify => self.verify = true,
            }
        }
    }
}

#[derive(Debug)]
pub struct Protocol {
    send_semantics: SendSemantics,
    nested: Nesting,
    pub managers: Vec<Identifier>,
    pub manages: Vec<Identifier>,
    pub messages: Vec<MessageDecl>,
}

impl Protocol {
    pub fn new(send_semantics: SendSemantics, nested: Nesting,
               managers: Vec<Identifier>, manages: Vec<Identifier>, decls: Vec<MessageDecl>) -> Protocol {
        Protocol { send_semantics: send_semantics, nested: nested,
                   managers: managers, manages: manages, messages: decls }
    }
}

#[derive(Debug)]
pub enum CxxTypeKind {
  Struct,
  Class,
}

#[derive(Debug)]
pub struct UsingStmt {
    pub cxx_type: TypeSpec,
    pub header: String,
    pub kind: Option<CxxTypeKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    Protocol,
    Header,
}

impl FileType {
    pub fn from_file_path(file_path: &Path) -> Option<FileType> {
        if let Some(e) = file_path.extension() {
            if e == "ipdlh" {
                Some(FileType::Header)
            } else {
                Some(FileType::Protocol)
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TranslationUnit {
    pub namespace: Namespace,
    pub file_type: FileType,
    pub file_name: PathBuf,
    pub cxx_includes: Vec<String>,
    pub includes: Vec<String>,
    pub using: Vec<UsingStmt>,
    pub structs: Vec<(Namespace, Vec<StructField>)>,
    pub unions: Vec<(Namespace, Vec<TypeSpec>)>,
    pub protocol: Option<(Namespace, Protocol)>,
}
