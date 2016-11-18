/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[derive(Debug)]
pub struct QualifiedId {
    base_id: String,
    quals: Vec<String>
}

impl QualifiedId {
    pub fn new(base: String) -> QualifiedId {
        QualifiedId { base_id: base, quals: Vec::new() }
    }

    pub fn qualify(mut self, id: String) -> QualifiedId {
        self.quals.push(self.base_id);
        self.base_id = id;
        self
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
    name: String,
    type_spec: TypeSpec,
}

impl Param {
    pub fn new(type_spec: TypeSpec, name: String) -> Param {
        Param { name: name, type_spec: type_spec }
    }
}

#[derive(Debug)]
pub struct StructField {
    type_spec: TypeSpec,
    name: String,
}

impl StructField {
    pub fn new(ty: TypeSpec, name: String) -> StructField {
        StructField { type_spec: ty, name: name }
    }
}

#[derive(Debug)]
pub struct Namespace {
    name: String,
    namespaces: Vec<String>,
}

impl Namespace {
    pub fn new(name: String) -> Namespace {
        Namespace { name: name, namespaces: Vec::new() }
    }

    pub fn add_outer_namespace(&mut self, namespace: &String) {
        self.namespaces.insert(0, namespace.clone());
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

#[derive(Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    In,
    Out,
    InOut,
}

#[derive(Debug)]
pub struct MessageDecl {
    name: String,
    pub send_semantics: SendSemantics,
    pub nesting: Nesting,
    pub prio: Priority,
    pub direction: Option<Direction>,
    in_params: Vec<Param>,
    out_params: Vec<Param>,
    compress: Compress,
    verify: bool,
}

impl MessageDecl {
    pub fn new(name: String) -> MessageDecl {
        MessageDecl {
            name: name,
            send_semantics: SendSemantics::Async,
            nesting: Nesting::None,
            prio: Priority::Normal,
            direction: None,
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
    nesting: Nesting,
    managers: Vec<String>,
    manages: Option<String>,
    messages: Vec<MessageDecl>,
}

impl Protocol {
    pub fn new(send_semantics: SendSemantics, nesting: Nesting,
               managers: Vec<String>, manages: Option<String>, decls: Vec<MessageDecl>) -> Protocol {
        Protocol { send_semantics: send_semantics, nesting: nesting,
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

#[derive(Debug)]
pub enum StructOrUnion {
    Struct(Vec<StructField>),
    Union(Vec<TypeSpec>),
}

#[derive(Debug)]
pub struct TranslationUnit {
    // XXX file_type
    // XXX file_name
    pub cxx_includes: Vec<String>,
    pub includes: Vec<String>,
    // XXX builtin_using
    pub using: Vec<UsingStmt>,
    pub structs_and_unions: Vec<(Namespace, StructOrUnion)>,
    pub protocol: Option<(Namespace, Protocol)>,
}
