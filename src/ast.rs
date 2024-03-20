/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct QualifiedId {
    pub base_id: Identifier,
    pub quals: Vec<String>,
}

impl QualifiedId {
    pub fn new(base: Identifier) -> QualifiedId {
        QualifiedId {
            base_id: base,
            quals: Vec::new(),
        }
    }

    pub fn qualify(mut self, id: Identifier) -> QualifiedId {
        self.quals.push(self.base_id.id);
        self.base_id = id;
        self
    }

    pub fn new_from_iter<'a, I>(mut ids: I) -> QualifiedId
    where
        I: Iterator<Item = &'a str>,
    {
        let loc = Location {
            file_name: PathBuf::from("<builtin>"),
            lineno: 0,
            colno: 0,
        };
        let mut qual_id = QualifiedId::new(Identifier::new(
            String::from(ids.next().unwrap()),
            loc.clone(),
        ));
        for i in ids {
            qual_id = qual_id.qualify(Identifier::new(String::from(i), loc.clone()));
        }
        qual_id
    }

    pub fn short_name(&self) -> String {
        self.base_id.to_string()
    }

    pub fn full_name(&self) -> Option<String> {
        if self.quals.is_empty() {
            None
        } else {
            Some(self.to_string())
        }
    }

    pub fn loc(&self) -> &Location {
        &self.base_id.loc
    }
}

impl fmt::Display for QualifiedId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for q in &self.quals {
            write!(f, "{}::", q)?;
        }
        write!(f, "{}", self.base_id)
    }
}

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    Identifier(Identifier),
    String(String),
    None,
}

pub type Attributes = HashMap<String, (Location, AttributeValue)>;

#[derive(Debug)]
pub struct TypeSpec {
    pub spec: QualifiedId,
    pub array: bool,
    pub maybe: bool,
    pub nullable: bool,
    pub uniqueptr: bool,
}

impl TypeSpec {
    pub fn new(spec: QualifiedId) -> TypeSpec {
        TypeSpec {
            spec: spec,
            array: false,
            maybe: false,
            nullable: false,
            uniqueptr: false,
        }
    }

    // XXX Get rid of these setters if the fields are just public anyways?

    pub fn set_array(mut self, is_array: bool) -> TypeSpec {
        self.array = is_array;
        self
    }

    pub fn set_maybe(mut self, is_maybe: bool) -> TypeSpec {
        self.maybe = is_maybe;
        self
    }

    pub fn set_nullable(mut self, is_nullable: bool) -> TypeSpec {
        self.nullable = is_nullable;
        self
    }

    pub fn set_uniqueptr(mut self, is_uniqueptr: bool) -> TypeSpec {
        self.uniqueptr = is_uniqueptr;
        self
    }

    pub fn loc(&self) -> &Location {
        self.spec.loc()
    }
}

#[derive(Debug)]
pub struct Param {
    pub attributes: Attributes,
    pub name: Identifier,
    pub type_spec: TypeSpec,
}

impl Param {
    pub fn new(attributes: Attributes, type_spec: TypeSpec, name: Identifier) -> Param {
        Param {
            attributes: attributes,
            name: name,
            type_spec: type_spec,
        }
    }
}

#[derive(Debug)]
pub struct StructField {
    pub type_spec: TypeSpec,
    pub name: Identifier,
}

impl StructField {
    pub fn new(ty: TypeSpec, name: Identifier) -> StructField {
        StructField {
            type_spec: ty,
            name: name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub name: Identifier,
    pub namespaces: Vec<String>,
}

impl Namespace {
    pub fn new(name: Identifier) -> Namespace {
        Namespace {
            name: name,
            namespaces: Vec::new(),
        }
    }

    pub fn add_outer_namespace(&mut self, namespace: &str) {
        self.namespaces.insert(0, String::from(namespace));
    }

    pub fn qname(&self) -> QualifiedId {
        QualifiedId {
            base_id: self.name.clone(),
            quals: self.namespaces.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SendSemantics {
    Async,
    Sync,
    Intr,
}

impl SendSemantics {
    pub fn is_async(&self) -> bool {
        self == &SendSemantics::Async
    }

    pub fn is_sync(&self) -> bool {
        self == &SendSemantics::Sync
    }

    pub fn is_intr(&self) -> bool {
        self == &SendSemantics::Intr
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Nesting {
    None = 1,
    InsideSync = 2,
    InsideCpow = 3,
}

impl Nesting {
    pub fn is_none(&self) -> bool {
        self == &Nesting::None
    }

    pub fn inside_sync(&self) -> bool {
        self == &Nesting::InsideSync
    }

    pub fn inside_cpow(&self) -> bool {
        self == &Nesting::InsideCpow
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Priority {
    Normal,
    Input,
    Vsync,
    Mediumhigh,
    Control,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProtocolSide {
    Parent,
    Child,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    To(ProtocolSide),
    Both,
}

impl Direction {
    pub fn new_parent() -> Direction {
        Direction::To(ProtocolSide::Parent)
    }

    pub fn new_child() -> Direction {
        Direction::To(ProtocolSide::Child)
    }

    pub fn new_both() -> Direction {
        Direction::Both
    }

    pub fn is_to_parent(&self) -> bool {
        self == &Direction::Both || self == &Direction::To(ProtocolSide::Parent)
    }

    pub fn is_to_child(&self) -> bool {
        self == &Direction::Both || self == &Direction::To(ProtocolSide::Child)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Location {
    pub file_name: PathBuf,
    pub lineno: usize,
    pub colno: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.file_name.display(),
            self.lineno,
            self.colno
        )
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier {
    pub id: String,
    pub loc: Location,
}

impl Identifier {
    pub fn new(name: String, loc: Location) -> Identifier {
        Identifier { id: name, loc: loc }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug)]
pub struct MessageDecl {
    pub name: Identifier,
    pub attributes: Attributes,
    pub send_semantics: SendSemantics,
    pub direction: Direction,
    pub in_params: Vec<Param>,
    pub out_params: Vec<Param>,
}

impl MessageDecl {
    pub fn new(
        name: Identifier,
        attributes: Attributes,
        send_semantics: SendSemantics,
        direction: Direction,
        in_params: Vec<Param>,
        out_params: Vec<Param>,
    ) -> MessageDecl {
        MessageDecl {
            name: name,
            attributes: attributes,
            send_semantics: send_semantics,
            direction: direction,
            in_params: in_params,
            out_params: out_params,
        }
    }
}

#[derive(Debug)]
pub struct Protocol {
    pub attributes: Attributes,
    pub send_semantics: SendSemantics,
    pub managers: Vec<Identifier>,
    pub manages: Vec<Identifier>,
    pub messages: Vec<MessageDecl>,
}

impl Protocol {
    pub fn new(
        attributes: Attributes,
        send_semantics: SendSemantics,
        managers: Vec<Identifier>,
        manages: Vec<Identifier>,
        decls: Vec<MessageDecl>,
    ) -> Protocol {
        Protocol {
            attributes: attributes,
            send_semantics: send_semantics,
            managers: managers,
            manages: manages,
            messages: decls,
        }
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
    pub attributes: Attributes,
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

// Translation unit identifier.
pub type TUId = i32;

#[derive(Debug)]
pub struct TranslationUnit {
    pub namespace: Namespace,
    pub file_type: FileType,
    pub file_name: PathBuf,
    pub cxx_includes: Vec<String>,
    pub includes: Vec<TUId>,
    pub using: Vec<UsingStmt>,
    pub structs: Vec<(Namespace, Attributes, Vec<StructField>)>,
    pub unions: Vec<(Namespace, Attributes, Vec<TypeSpec>)>,
    pub protocol: Option<(Namespace, Protocol)>,
}
