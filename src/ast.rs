#[derive(Debug)]
pub enum IncludeType {
    Protocol,
    Header,
}

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
    state: Option<String>,
    array: bool,
    nullable: bool,
}

impl TypeSpec {
    pub fn new(spec: QualifiedId) -> TypeSpec {
        TypeSpec { spec: spec, state: None, array: false, nullable: false }
    }

    pub fn add_state(mut self, state: String) -> TypeSpec {
        self.state = Some(state);
        self
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
pub enum CxxTypeKind {
  Struct,
  Class,
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

#[derive(Debug)]
pub enum Direction {
    In,
    Out,
    InOut,
}

#[derive(Debug)]
pub struct MessageDecl {
    name: String,
    send_semantics: SendSemantics,
    nesting: Nesting,
    prio: Priority,
    direction: Option<Direction>,
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
pub enum NamespacedNode {
    StructDecl(Vec<StructField>),
    UnionDecl(Vec<TypeSpec>),
}

#[derive(Debug)]
pub enum Node {
    CxxInclude(String),
    Include(IncludeType, String),
    Namespaced { namespace: Namespace, node: NamespacedNode },
    TypeSpec(TypeSpec),
    Using { cxx_type: TypeSpec, header: String, kind: Option<CxxTypeKind> },
}
