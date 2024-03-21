/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ast::*;
use errors::Errors;
use std::collections::{HashMap, HashSet};

// C types
//
// These types don't live in any namespace, so can't be imported with `using`
// statements like normal C++ types.
const BUILTIN_C_TYPES: &'static [&'static str] = &[
    // C types
    "bool", "char", "short", "int", "long", "float", "double",
];

// C++ types
//
// These types must be fully qualified, and will be `typedef`-ed into IPDL
// structs to make them readily available when used.
const BUILTIN_TYPES: &'static [&'static str] = &[
    // stdint types
    "int8_t",
    "uint8_t",
    "int16_t",
    "uint16_t",
    "int32_t",
    "uint32_t",
    "int64_t",
    "uint64_t",
    "intptr_t",
    "uintptr_t",
    // stddef types
    "size_t",
    "ssize_t",
    // Mozilla types: "less" standard things we know how serialize/deserialize
    "nsresult",
    "nsString",
    "nsCString",
    "nsDependentSubstring",
    "nsDependentCSubstring",
    "mozilla::ipc::Shmem",
    "mozilla::ipc::ByteBuf",
    "mozilla::UniquePtr",
    "mozilla::ipc::FileDescriptor",
];

fn builtin_from_string(tname: &str) -> QualifiedId {
    QualifiedId::new_from_iter(tname.split("::"))
}

const DELETE_MESSAGE_NAME: &'static str = "__delete__";
const CONSTRUCTOR_SUFFIX: &'static str = "Constructor";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TypeRef {
    tu: TUId,
    index: usize,
}

impl TypeRef {
    fn new(tu: &TUId, index: usize) -> TypeRef {
        TypeRef {
            tu: tu.clone(),
            index: index,
        }
    }

    fn lookup_struct<'a>(&self, tuts: &'a HashMap<TUId, TranslationUnitType>) -> &'a StructTypeDef {
        &tuts.get(&self.tu).unwrap().structs[self.index]
    }

    fn lookup_union<'a>(&self, tuts: &'a HashMap<TUId, TranslationUnitType>) -> &'a UnionTypeDef {
        &tuts.get(&self.tu).unwrap().unions[self.index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lifetime {
    RefCounted,
    ManualDealloc,
}

// XXX The Python compiler has "Type" and a subclass "IPDLType". I
// don't know how useful it is to split them. Plus my notion of type
// may be different.
#[derive(Debug, Clone)]
enum IPDLType {
    BuiltinCType(&'static str),
    ImportedCxxType(
        QualifiedId,
        Lifetime,
        bool, /* send moveonly */
        bool, /* data moveonly */
    ),
    MessageType(TypeRef),
    ProtocolType(TUId),
    ActorType(TUId),
    StructType(TypeRef),
    UnionType(TypeRef),
    ArrayType(Box<IPDLType>),
    MaybeType(Box<IPDLType>),
    ShmemType(QualifiedId),
    ByteBufType(QualifiedId),
    FDType(QualifiedId),
    EndpointType(QualifiedId),
    ManagedEndpointType(QualifiedId),
    UniquePtrType(Box<IPDLType>),
    NotNullType(Box<IPDLType>),
}

fn get_protocol_type<'a>(
    tuts: &'a HashMap<TUId, TranslationUnitType>,
    tuid: &TUId,
) -> &'a ProtocolTypeDef {
    tuts.get(tuid).unwrap().protocol.as_ref().unwrap()
}

impl IPDLType {
    // XXX This has to be the same as the IPDL Python compiler's class names,
    // to perfectly match the error messages, because Type::typename is defined
    // as self.__class__.__name__.
    fn type_name(&self) -> &'static str {
        match self {
            &IPDLType::BuiltinCType(_) => "BuiltinCType",
            &IPDLType::ImportedCxxType(_, _, _, _) => "ImportedCxxType",
            &IPDLType::MessageType(_) => "MessageType",
            &IPDLType::ProtocolType(_) => "ProtocolType",
            &IPDLType::ActorType(_) => "ActorType",
            &IPDLType::StructType(_) => "StructType",
            &IPDLType::UnionType(_) => "UnionType",
            &IPDLType::ArrayType(_) => "ArrayType",
            &IPDLType::MaybeType(_) => "MaybeType",
            &IPDLType::ShmemType(_) => "ShmemType",
            &IPDLType::ByteBufType(_) => "ByteBufType",
            &IPDLType::FDType(_) => "FDType",
            &IPDLType::EndpointType(_) => "EndpointType",
            &IPDLType::ManagedEndpointType(_) => "ManagedEndpointType",
            &IPDLType::UniquePtrType(_) => "UniquePtrType",
            &IPDLType::NotNullType(_) => "NotNullType",
        }
    }

    fn name(&self, tuts: &HashMap<TUId, TranslationUnitType>) -> String {
        match self {
            &IPDLType::BuiltinCType(name) => name.to_string(),
            &IPDLType::ImportedCxxType(ref qid, _, _, _) => qid.short_name(),
            &IPDLType::MessageType(_) => "???".to_string(),
            &IPDLType::ProtocolType(ref p) => get_protocol_type(&tuts, &p).qname.to_string(),
            &IPDLType::ActorType(ref p) => get_protocol_type(&tuts, &p).qname.to_string(),
            &IPDLType::StructType(ref tr) => tr.lookup_struct(&tuts).qname.to_string(),
            &IPDLType::UnionType(ref tr) => tr.lookup_union(&tuts).qname.to_string(),
            &IPDLType::ArrayType(ref t_inner) => {
                let mut array_name = t_inner.name(&tuts);
                array_name.push_str("[]");
                array_name
            }
            &IPDLType::MaybeType(ref t_inner) => {
                let mut maybe_name = t_inner.name(&tuts);
                maybe_name.push_str("?");
                maybe_name
            }
            &IPDLType::ShmemType(ref qname) => qname.short_name(),
            &IPDLType::ByteBufType(ref qname) => qname.short_name(),
            &IPDLType::FDType(ref qname) => qname.short_name(),
            &IPDLType::EndpointType(ref qname) => qname.short_name(),
            &IPDLType::ManagedEndpointType(ref qname) => qname.short_name(),
            &IPDLType::UniquePtrType(ref t_inner) => {
                let mut up_name = "UniquePtr<".to_string();
                up_name.push_str(&t_inner.name(&tuts));
                up_name.push_str(">");
                up_name
            }
            &IPDLType::NotNullType(ref t_inner) => {
                let mut nn_name = "NotNullPtr<".to_string();
                nn_name.push_str(&t_inner.name(&tuts));
                nn_name.push_str(">");
                nn_name
            }
        }
    }

    fn canonicalize(
        &self,
        tuts: &HashMap<TUId, TranslationUnitType>,
        type_spec: &TypeSpec,
    ) -> (Errors, IPDLType) {
        let mut errors = Errors::none();
        let mut itype = self.clone();

        if type_spec.uniqueptr {
            itype = IPDLType::UniquePtrType(Box::new(itype))
        }

        if let &IPDLType::ProtocolType(ref p) = self {
            itype = IPDLType::ActorType(p.clone())
        }

        match itype {
            // This case covers when supportsNullable() from type.py is true.
            IPDLType::ActorType(_) | IPDLType::ImportedCxxType(_, Lifetime::RefCounted, _, _) => {
                if !type_spec.nullable {
                    itype = IPDLType::NotNullType(Box::new(itype))
                }
            }
            _ => {
                if type_spec.nullable {
                    errors.append_one(
                        type_spec.loc(),
                        &format!(
                            "`nullable' qualifier for type `{}' is unsupported",
                            itype.name(&tuts)
                        ),
                    );
                }
            }
        }

        if type_spec.array {
            itype = IPDLType::ArrayType(Box::new(itype))
        }

        if type_spec.maybe {
            itype = IPDLType::MaybeType(Box::new(itype))
        }

        (errors, itype)
    }

    fn is_refcounted(&self) -> bool {
        match self {
            &IPDLType::ImportedCxxType(_, Lifetime::RefCounted, _, _) => true,
            _ => false,
        }
    }

    fn is_send_moveonly(&self) -> bool {
        match self {
            &IPDLType::ImportedCxxType(_, _, send_moveonly, _) => send_moveonly,
            _ => false,
        }
    }

    fn is_data_moveonly(&self) -> bool {
        match self {
            &IPDLType::ImportedCxxType(_, _, _, data_moveonly) => data_moveonly,
            _ => false,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct StructTypeDef {
    qname: QualifiedId,
    comparable: bool,
    fields: Vec<IPDLType>,
}

impl StructTypeDef {
    fn new(ns: &Namespace, comparable: bool) -> StructTypeDef {
        StructTypeDef {
            qname: ns.qname(),
            comparable: comparable,
            fields: Vec::new(),
        }
    }

    fn append_field(&mut self, field_type: IPDLType) {
        self.fields.push(field_type)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct UnionTypeDef {
    qname: QualifiedId,
    comparable: bool,
    components: Vec<IPDLType>,
}

impl UnionTypeDef {
    fn new(ns: &Namespace, comparable: bool) -> UnionTypeDef {
        UnionTypeDef {
            qname: ns.qname(),
            comparable: comparable,
            components: Vec::new(),
        }
    }

    fn append_component(&mut self, component_type: IPDLType) {
        self.components.push(component_type)
    }
}

#[derive(Debug, Clone)]
enum MessageType {
    Ctor(TUId),
    Dtor(TUId),
    Other,
}

impl MessageType {
    fn is_ctor(&self) -> bool {
        match self {
            &MessageType::Ctor(_) => true,
            _ => false,
        }
    }

    fn constructed_type(&self) -> &TUId {
        match self {
            &MessageType::Ctor(ref tuid) => tuid,
            _ => panic!("Tried to get constructed type on non-Ctor"),
        }
    }

    fn is_dtor(&self) -> bool {
        match self {
            &MessageType::Dtor(_) => true,
            _ => false,
        }
    }
}

struct MessageStrength {
    send_semantics: SendSemantics,
    nested_min: Nesting,
    nested_max: Nesting,
}

impl MessageStrength {
    fn converts_to(&self, other: &MessageStrength) -> bool {
        if self.nested_min < other.nested_min {
            return false;
        }

        if self.nested_max > other.nested_max {
            return false;
        }

        // Protocols that use intr semantics are not allowed to use
        // message nesting.
        if other.send_semantics.is_intr()
            && (!self.nested_min.is_none() || !self.nested_max.is_none())
        {
            return false;
        }

        if self.send_semantics.is_async() {
            return true;
        } else if self.send_semantics.is_sync() && !other.send_semantics.is_async() {
            return true;
        } else if other.send_semantics.is_intr() {
            return true;
        }

        return false;
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ParamTypeDef {
    name: Identifier,
    param_type: IPDLType,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct MessageTypeDef {
    name: Identifier,
    send_semantics: SendSemantics,
    nested: Nesting,
    prio: Priority,
    direction: Direction,
    params: Vec<ParamTypeDef>,
    returns: Vec<ParamTypeDef>,
    mtype: MessageType,
    compress: Compress,
}
// XXX Need to add LegacyIntr, Tainted.

fn get_attribute_value<A: Clone>(
    attributes: &Attributes,
    key: &str,
    not_present: A,
    no_value: A,
    identifier_map: HashMap<&'static str, A>,
    string_map: fn(&String) -> A,
) -> A {
    attributes
        .get(key)
        .map(|(_, v)| match v {
            AttributeValue::Identifier(i) => {
                (*identifier_map.get(&i.id as &str).unwrap_or(&not_present)).clone()
            }
            AttributeValue::String(s) => string_map(&s),
            AttributeValue::None => no_value,
        })
        .unwrap_or(not_present)
}

fn get_nested(attributes: &Attributes, key: &str) -> Nesting {
    get_attribute_value(
        attributes,
        key,
        Nesting::None,
        Nesting::None,
        HashMap::from([
            ("not", Nesting::None),
            ("inside_sync", Nesting::InsideSync),
            ("inside_cpow", Nesting::InsideCpow),
        ]),
        |_| Nesting::None,
    )
}

fn get_prio(attributes: &Attributes) -> Priority {
    get_attribute_value(
        attributes,
        "Priority",
        Priority::Normal,
        Priority::Normal,
        HashMap::from([
            ("normal", Priority::Normal),
            ("input", Priority::Input),
            ("vsync", Priority::Vsync),
            ("mediumhigh", Priority::Mediumhigh),
            ("control", Priority::Control),
        ]),
        |_| Priority::Normal,
    )
}

fn get_compress(attributes: &Attributes) -> Compress {
    get_attribute_value(
        attributes,
        "Compress",
        Compress::None,
        Compress::Enabled,
        HashMap::from([("all", Compress::All)]),
        |_| Compress::None,
    )
}

impl MessageTypeDef {
    fn new(md: &MessageDecl, name: &str, mtype: MessageType) -> MessageTypeDef {
        assert!(!mtype.is_ctor() || name.ends_with(CONSTRUCTOR_SUFFIX));
        MessageTypeDef {
            name: Identifier::new(String::from(name), md.name.loc.clone()),
            send_semantics: md.send_semantics,
            nested: get_nested(&md.attributes, "Nested"),
            prio: get_prio(&md.attributes),
            direction: md.direction,
            params: Vec::new(),
            returns: Vec::new(),
            mtype: mtype,
            compress: get_compress(&md.attributes),
        }
    }

    fn is_ctor(&self) -> bool {
        self.mtype.is_ctor()
    }

    fn constructed_type(&self) -> &TUId {
        self.mtype.constructed_type()
    }

    fn is_dtor(&self) -> bool {
        self.mtype.is_dtor()
    }

    fn message_strength(&self) -> MessageStrength {
        MessageStrength {
            send_semantics: self.send_semantics,
            nested_min: self.nested,
            nested_max: self.nested,
        }
    }

    fn converts_to(&self, protocol: &ProtocolTypeDef) -> bool {
        self.message_strength()
            .converts_to(&protocol.message_strength())
    }

    pub fn is_async(&self) -> bool {
        self.send_semantics.is_async()
    }

    pub fn is_sync(&self) -> bool {
        self.send_semantics.is_sync()
    }
}

#[derive(Debug, Clone)]
struct ProtocolTypeDef {
    qname: QualifiedId,
    send_semantics: SendSemantics,
    nested: Nesting,
    managers: Vec<TUId>,
    manages: Vec<TUId>,
    messages: Vec<MessageTypeDef>,
    has_delete: bool,
    #[allow(dead_code)]
    lifetime: Lifetime,
    needs_other_pid: bool,
}

impl ProtocolTypeDef {
    fn new(&(ref ns, ref p): &(Namespace, Protocol)) -> ProtocolTypeDef {
        ProtocolTypeDef {
            qname: ns.qname(),
            send_semantics: p.send_semantics,
            nested: get_nested(&p.attributes, "NestedUpTo"),
            managers: Vec::new(),
            manages: Vec::new(),
            messages: Vec::new(),
            has_delete: false,
            lifetime: if p.attributes.contains_key("ManualDealloc") {
                Lifetime::RefCounted
            } else {
                Lifetime::ManualDealloc
            },
            needs_other_pid: p.attributes.contains_key("NeedsOtherPid"),
        }
    }

    fn is_top_level(&self) -> bool {
        self.managers.len() == 0
    }

    fn message_strength(&self) -> MessageStrength {
        MessageStrength {
            send_semantics: self.send_semantics,
            nested_min: Nesting::None,
            nested_max: self.nested,
        }
    }

    fn converts_to(&self, other: &ProtocolTypeDef) -> bool {
        self.message_strength()
            .converts_to(&other.message_strength())
    }
}

#[derive(Debug, Clone)]
struct Decl {
    loc: Location,
    decl_type: IPDLType,
    short_name: String,
    full_name: Option<String>,
}

// The Python version also has a "progname" field, but I don't see any
// reason to keep that separate from the short_name field.

impl Decl {
    fn new(loc: &Location, decl_type: IPDLType, short_name: String) -> Decl {
        Decl {
            loc: loc.clone(),
            decl_type: decl_type,
            short_name: short_name,
            full_name: None,
        }
    }

    fn new_from_qid(qid: &QualifiedId, decl_type: IPDLType) -> Decl {
        Decl {
            loc: qid.loc().clone(),
            decl_type: decl_type,
            short_name: qid.short_name(),
            full_name: qid.full_name(),
        }
    }
}

struct SymbolTable {
    scopes: Vec<HashMap<String, Decl>>,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            scopes: vec![HashMap::new()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn exit_scope(&mut self) {
        self.scopes.pop().unwrap();
        ()
    }

    // XXX Should/can this return a reference?
    fn lookup(&self, sym: &str) -> Option<Decl> {
        for s in &self.scopes {
            if let Some(e) = s.get(sym).clone() {
                return Some(e.clone());
            }
        }
        None
    }

    fn declare_inner(&mut self, name: &str, decl: Decl) -> Errors {
        if let Some(old_decl) = self.lookup(name) {
            return Errors::one(
                &decl.loc,
                &format!(
                    "redeclaration of symbol `{}', first declared at {}",
                    name, old_decl.loc
                ),
            );
        }

        let old_binding = self
            .scopes
            .last_mut()
            .unwrap()
            .insert(String::from(name), decl);
        assert!(old_binding.is_none());
        Errors::none()
    }

    fn declare(&mut self, decl: Decl) -> Errors {
        let mut errors = self.declare_inner(&decl.short_name, decl.clone());
        if let Some(ref full_name) = decl.full_name {
            errors.append(self.declare_inner(full_name, decl.clone()));
        }
        errors
    }
}

fn declare_cxx_type(
    sym_tab: &mut SymbolTable,
    spec: &QualifiedId,
    refcounted: Lifetime,
    send_moveonly: bool,
    data_moveonly: bool,
) -> Errors {
    let ipdl_type = match spec.full_name() {
        Some(ref n) if n == "::mozilla::ipc::Shmem" => IPDLType::ShmemType(spec.clone()),
        Some(ref n) if n == "::mozilla::ipc::ByteBuf" => IPDLType::ByteBufType(spec.clone()),
        Some(ref n) if n == "::mozilla::ipc::FileDescriptor" => IPDLType::FDType(spec.clone()),
        _ => {
            let ipdl_type =
                IPDLType::ImportedCxxType(spec.clone(), refcounted, send_moveonly, data_moveonly);
            let full_name = format!("{}", spec);
            // ??? What to do here for UniquePtr?
            if let Some(decl) = sym_tab.lookup(&full_name) {
                if let Some(existing_type) = decl.full_name {
                    if existing_type == full_name {
                        if (refcounted == Lifetime::RefCounted) != decl.decl_type.is_refcounted() {
                            return Errors::one(&spec.loc(),
                                               &format!("inconsistent refcounted status of type `{}`, first declared at {}",
                                                        full_name, decl.loc));
                        }
                        if send_moveonly != decl.decl_type.is_send_moveonly()
                            || data_moveonly != decl.decl_type.is_data_moveonly()
                        {
                            return Errors::one(&spec.loc(),
                                                   &format!("inconsistent moveonly status of type `{}`, first declared at {}",
                                                            full_name, decl.loc));
                        }

                        // This type has already been added, so don't do anything.
                        return Errors::none();
                    }
                };
            };
            ipdl_type
        }
    };
    sym_tab.declare(Decl::new_from_qid(&spec, ipdl_type))
}

#[derive(Clone)]
struct TranslationUnitType {
    pub structs: Vec<StructTypeDef>,
    pub unions: Vec<UnionTypeDef>,
    pub protocol: Option<ProtocolTypeDef>,
}

impl TranslationUnitType {
    fn new(maybe_protocol: &Option<(Namespace, Protocol)>) -> TranslationUnitType {
        let protocol = maybe_protocol
            .as_ref()
            .map(|ref p| ProtocolTypeDef::new(&p));
        TranslationUnitType {
            structs: Vec::new(),
            unions: Vec::new(),
            protocol: protocol,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AttributeSpecValue {
    Valueless,
    StringLiteral,
    Keyword(&'static str),
}

impl AttributeSpecValue {
    pub fn check(&self, v: &AttributeValue) -> bool {
        match (v, &self) {
            (AttributeValue::Identifier(id1), AttributeSpecValue::Keyword(id2)) => &id1.id == id2,
            (AttributeValue::String(_), AttributeSpecValue::StringLiteral) => true,
            (AttributeValue::None, AttributeSpecValue::Valueless) => true,
            (_, _) => false,
        }
    }
}

type AttributeSpec = HashMap<&'static str, Vec<AttributeSpecValue>>;

fn check_attributes(attributes: &Attributes, specs: &AttributeSpec) -> Errors {
    let mut errors = Errors::none();

    for (name, (loc, value)) in attributes {
        let spec = match specs.get(name as &str) {
            Some(s) => s,
            None => {
                errors.append_one(&loc, &format!("unknown attribute `{}'", name));
                continue;
            }
        };

        if spec.len() == 0 {
            if value != &AttributeValue::None {
                errors.append_one(
                    &loc,
                    &format!("unexpected value for valueless attribute `{}'", name),
                );
            }
            continue;
        }

        if !spec.iter().any(|s| s.check(&value)) {
            let options = spec
                .iter()
                .map(|f| match f {
                    AttributeSpecValue::Valueless => "None",
                    AttributeSpecValue::StringLiteral => "StringLiteral",
                    AttributeSpecValue::Keyword(k) => *k,
                })
                .collect::<Vec<_>>()
                .join(", ");
            errors.append_one(
                &loc,
                &format!("invalid value for attribute `{name}', expected one of: {options}",),
            );
        }
    }

    errors
}

fn declare_protocol(
    sym_tab: &mut SymbolTable,
    tuid: &TUId,
    ns: &Namespace,
    p: &Protocol,
) -> Errors {
    let mut errors = Errors::none();

    let protocol_attributes: AttributeSpec = HashMap::from([
        ("ManualDealloc", Vec::new()),
        (
            "NestedUpTo",
            Vec::from([
                AttributeSpecValue::Keyword("not"),
                AttributeSpecValue::Keyword("inside_sync"),
                AttributeSpecValue::Keyword("inside_cpow"),
            ]),
        ),
        ("NeedsOtherPid", Vec::new()),
        (
            "ChildImpl",
            Vec::from([
                AttributeSpecValue::Keyword("virtual"),
                AttributeSpecValue::StringLiteral,
            ]),
        ),
        (
            "ParentImpl",
            Vec::from([
                AttributeSpecValue::Keyword("virtual"),
                AttributeSpecValue::StringLiteral,
            ]),
        ),
    ]);
    errors.append(check_attributes(&p.attributes, &protocol_attributes));

    let p_type = IPDLType::ProtocolType(tuid.clone());
    errors.append(sym_tab.declare(Decl::new_from_qid(&ns.qname(), p_type)));

    let ref loc = ns.name.loc;
    let mut declare_endpoint = |is_managed: bool, side: &str| {
        let endpoint_str = if is_managed {
            "ManagedEndpoint"
        } else {
            "Endpoint"
        };
        let full_id = Identifier::new(
            format!("{}<{}{}>", endpoint_str, ns.qname(), side),
            loc.clone(),
        );
        let namespaces = vec!["mozilla".to_string(), "ipc".to_string()];
        let full_qid = QualifiedId {
            base_id: full_id,
            quals: namespaces,
        };
        let endpoint_type = if is_managed {
            IPDLType::ManagedEndpointType(full_qid)
        } else {
            IPDLType::EndpointType(full_qid)
        };
        let short_name = format!("{}<{}{}>", endpoint_str, ns.name.id, side);
        sym_tab.declare(Decl::new(loc, endpoint_type, short_name))
    };
    errors.append(declare_endpoint(true, "Parent"));
    errors.append(declare_endpoint(true, "Child"));
    errors.append(declare_endpoint(false, "Parent"));
    errors.append(declare_endpoint(false, "Child"));

    errors
}

fn declare_usings(mut sym_tab: &mut SymbolTable, tu: &TranslationUnit) -> Errors {
    let mut errors = Errors::none();

    let using_attributes: AttributeSpec = HashMap::from([
        (
            "MoveOnly",
            Vec::from([
                AttributeSpecValue::Valueless,
                AttributeSpecValue::Keyword("send"),
                AttributeSpecValue::Keyword("data"),
            ]),
        ),
        ("RefCounted", Vec::new()),
    ]);

    for u in &tu.using {
        errors.append(check_attributes(&u.attributes, &using_attributes));

        let (send, data) = u
            .attributes
            .get("MoveOnly")
            .map(|(_, v)| match v {
                AttributeValue::None => (true, true),
                AttributeValue::Identifier(i) => match i.id.as_str() {
                    "send" => (true, false),
                    "data" => (false, true),
                    _ => (false, false),
                },
                _ => (false, false),
            })
            .unwrap_or((false, false));

        errors.append(declare_cxx_type(
            &mut sym_tab,
            &u.cxx_type,
            if u.attributes.contains_key("RefCounted") {
                Lifetime::RefCounted
            } else {
                Lifetime::ManualDealloc
            },
            send,
            data,
        ));
    }
    errors
}

fn declare_structs_and_unions(
    sym_tab: &mut SymbolTable,
    tuid: &TUId,
    tu: &TranslationUnit,
) -> Errors {
    let mut errors = Errors::none();
    let mut index = 0;

    for s in &tu.structs {
        let s_type = IPDLType::StructType(TypeRef::new(&tuid, index));
        errors.append(sym_tab.declare(Decl::new_from_qid(&s.0.qname(), s_type)));
        index += 1;
    }

    index = 0;
    for u in &tu.unions {
        let u_type = IPDLType::UnionType(TypeRef::new(&tuid, index));
        errors.append(sym_tab.declare(Decl::new_from_qid(&u.0.qname(), u_type)));
        index += 1;
    }

    errors
}

fn gather_decls_struct(
    sym_tab: &mut SymbolTable,
    tuts: &HashMap<TUId, TranslationUnitType>,
    &(ref ns, _, ref sd): &(Namespace, Attributes, Vec<StructField>),
    sdef: &mut StructTypeDef,
) -> Errors {
    let mut errors = Errors::none();

    sym_tab.enter_scope();

    for f in sd {
        let fty_string = f.type_spec.spec.to_string();
        let fty_decl = sym_tab.lookup(&fty_string);
        if fty_decl.is_none() {
            errors.append_one(
                &f.name.loc,
                &format!(
                    "field `{}' of struct `{}' has unknown type `{}'",
                    f.name,
                    ns.qname().short_name(),
                    fty_string
                ),
            );
            continue;
        }
        let (errors2, f_type) = fty_decl
            .unwrap()
            .decl_type
            .canonicalize(&tuts, &f.type_spec);
        errors.append(errors2);

        errors.append(sym_tab.declare(Decl::new(&f.name.loc, f_type.clone(), f.name.id.clone())));
        sdef.append_field(f_type);
    }

    sym_tab.exit_scope();

    errors
}

fn gather_decls_union(
    sym_tab: &mut SymbolTable,
    tuts: &HashMap<TUId, TranslationUnitType>,
    &(ref ns, _, ref ud): &(Namespace, Attributes, Vec<TypeSpec>),
    udef: &mut UnionTypeDef,
) -> Errors {
    let mut errors = Errors::none();

    for c in ud {
        let c_string = c.spec.to_string();
        let c_decl = sym_tab.lookup(&c_string);
        if c_decl.is_none() {
            errors.append_one(
                c.loc(),
                &format!(
                    "unknown component type `{}' of union `{}'",
                    c_string,
                    ns.qname().short_name()
                ),
            );
            continue;
        }
        let (errors2, c_ty) = c_decl.unwrap().decl_type.canonicalize(&tuts, &c);
        errors.append(errors2);
        udef.append_component(c_ty);
    }

    errors
}

fn gather_decls_manager(
    sym_tab: &mut SymbolTable,
    managee: &(Namespace, Protocol),
    managee_type: &mut ProtocolTypeDef,
    manager: &Identifier,
) -> Errors {
    let manager_decl = match sym_tab.lookup(&manager.id) {
        Some(decl) => decl,
        None => {
            return Errors::one(
                &manager.loc,
                &format!(
                    "protocol `{}' referenced as |manager| of `{}' has not been declared",
                    manager.id,
                    managee.0.qname().short_name()
                ),
            )
        }
    };

    if let &IPDLType::ProtocolType(ref pt) = &manager_decl.decl_type {
        managee_type.managers.push(pt.clone());
        return Errors::none();
    }

    return Errors::one(&manager.loc,
                       &format!("entity `{}' referenced as |manager| of `{}' is not of `protocol' type; instead it is a {}",
                                manager.id, managee.0.qname().short_name(),
                                manager_decl.decl_type.type_name()));
}

fn gather_decls_manages(
    sym_tab: &mut SymbolTable,
    manager: &(Namespace, Protocol),
    manager_type: &mut ProtocolTypeDef,
    managee: &Identifier,
) -> Errors {
    let managee_decl = match sym_tab.lookup(&managee.id) {
        Some(decl) => decl,
        None => {
            return Errors::one(
                &managee.loc,
                &format!(
                    "protocol `{}', managed by `{}', has not been declared",
                    managee.id,
                    manager.0.qname().short_name()
                ),
            )
        }
    };

    if let &IPDLType::ProtocolType(ref pt) = &managee_decl.decl_type {
        manager_type.manages.push(pt.clone());
        return Errors::none();
    }

    return Errors::one(
        &managee.loc,
        &format!(
            "{} declares itself managing a non-`protocol' entity `{}' that is a {}",
            manager.0.qname().short_name(),
            managee.id,
            managee_decl.decl_type.type_name()
        ),
    );
}

fn gather_decls_message(
    sym_tab: &mut SymbolTable,
    tuts: &HashMap<TUId, TranslationUnitType>,
    tuid: &TUId,
    protocol_type: &mut ProtocolTypeDef,
    md: &MessageDecl,
) -> Errors {
    let mut errors = Errors::none();
    let mut message_name = md.name.id.clone();
    let mut mtype = MessageType::Other;

    if let Some(ref decl) = sym_tab.lookup(&message_name) {
        if let IPDLType::ProtocolType(ref pt) = decl.decl_type {
            // Probably a ctor. We'll check validity later.
            message_name += CONSTRUCTOR_SUFFIX;
            mtype = MessageType::Ctor(pt.clone());
        } else {
            errors.append_one(
                &md.name.loc,
                &format!(
                    "message name `{}' already declared as `{}'",
                    md.name,
                    decl.decl_type.type_name()
                ),
            );
            // If we error here, no big deal; move on to find more.
        }
    }

    if DELETE_MESSAGE_NAME == message_name {
        mtype = MessageType::Dtor(tuid.clone());
    }

    sym_tab.enter_scope();

    let message_attributes: AttributeSpec = HashMap::from([
        ("Tainted", Vec::new()),
        (
            "Compress",
            Vec::from([
                AttributeSpecValue::Valueless,
                AttributeSpecValue::Keyword("all"),
            ]),
        ),
        (
            "Priority",
            Vec::from([
                AttributeSpecValue::Keyword("normal"),
                AttributeSpecValue::Keyword("input"),
                AttributeSpecValue::Keyword("vsync"),
                AttributeSpecValue::Keyword("mediumhigh"),
                AttributeSpecValue::Keyword("control"),
            ]),
        ),
        (
            "Nested",
            Vec::from([
                AttributeSpecValue::Keyword("not"),
                AttributeSpecValue::Keyword("inside_sync"),
                AttributeSpecValue::Keyword("inside_cpow"),
            ]),
        ),
        ("LegacyIntr", Vec::new()),
    ]);
    errors.append(check_attributes(&md.attributes, &message_attributes));

    let mut msg_type = MessageTypeDef::new(&md, &message_name, mtype);

    {
        // The Python version adds the parameter, just with a dummy
        // type. Here I choose to be consistent with how we handle struct
        // fields with invalid types and simply omit the parameter.
        let mut param_to_decl = |param: &Param| {
            let param_attributes: AttributeSpec = HashMap::from([(
                "NoTaint",
                Vec::from([
                    AttributeSpecValue::Keyword("passback"),
                    AttributeSpecValue::Keyword("allvalid"),
                ]),
            )]);
            errors.append(check_attributes(&param.attributes, &param_attributes));

            let pt_name = param.type_spec.spec.to_string();

            if param.attributes.contains_key("NoTaint") && !md.attributes.contains_key("Tainted") {
                errors.append_one(param.type_spec.loc(),
                                  &format!("argument typename `{}' of message `{}' has a NoTaint attribute, but the message lacks the Tainted attribute",
                                           &pt_name, &message_name));
            }

            match sym_tab.lookup(&pt_name) {
                Some(p_type) => {
                    let (errors2, t) = p_type.decl_type.canonicalize(&tuts, &param.type_spec);
                    errors.append(errors2);
                    let decl = Decl::new(param.type_spec.loc(), t.clone(), param.name.id.clone());
                    errors.append(sym_tab.declare(decl));
                    Some(ParamTypeDef {
                        name: param.name.clone(),
                        param_type: t,
                    })
                }
                None => {
                    errors.append_one(
                        param.type_spec.loc(),
                        &format!(
                            "argument typename `{}' of message `{}' has not been declared",
                            &pt_name, message_name
                        ),
                    );
                    None
                }
            }
        };

        for in_param in &md.in_params {
            if let Some(t) = param_to_decl(&in_param) {
                msg_type.params.push(t);
            }
        }

        for out_param in &md.out_params {
            if let Some(t) = param_to_decl(&out_param) {
                msg_type.returns.push(t);
            }
        }
    }

    sym_tab.exit_scope();

    let index = protocol_type.messages.len();
    protocol_type.messages.push(msg_type);

    let mt = IPDLType::MessageType(TypeRef::new(&tuid, index));
    errors.append(sym_tab.declare(Decl::new(&md.name.loc, mt, message_name)));

    errors
}

fn gather_decls_protocol(
    mut sym_tab: &mut SymbolTable,
    tuts: &HashMap<TUId, TranslationUnitType>,
    tuid: &TUId,
    p: &(Namespace, Protocol),
    mut p_type: &mut ProtocolTypeDef,
) -> Errors {
    let mut errors = Errors::none();

    sym_tab.enter_scope();

    {
        let mut seen_managers = HashSet::new();
        for manager in &p.1.managers {
            if seen_managers.contains(&manager.id) {
                errors.append_one(
                    &manager.loc,
                    &format!("manager `{}' appears multiple times", manager.id),
                );
                continue;
            }

            seen_managers.insert(manager.id.clone());

            errors.append(gather_decls_manager(
                &mut sym_tab,
                &p,
                &mut p_type,
                &manager,
            ));
        }
    }

    for managee in &p.1.manages {
        errors.append(gather_decls_manages(
            &mut sym_tab,
            &p,
            &mut p_type,
            &managee,
        ));
    }

    for md in &p.1.messages {
        errors.append(gather_decls_message(
            &mut sym_tab,
            &tuts,
            &tuid,
            &mut p_type,
            &md,
        ));
    }

    let delete_type = sym_tab.lookup(DELETE_MESSAGE_NAME);
    p_type.has_delete = delete_type.is_some();
    if !(p_type.has_delete || p_type.is_top_level()) {
        errors.append_one(
            &p.0.name.loc,
            &format!(
                "destructor declaration `{}(...)' required for managed protocol `{}'",
                DELETE_MESSAGE_NAME,
                p.0.qname().short_name()
            ),
        );
    }

    if !p_type.is_top_level() && p_type.needs_other_pid {
        errors.append_one(
            &p.0.name.loc,
            &format!("[NeedsOtherPid] only applies to toplevel protocols"),
        );
    }

    // FIXME/cjones Declare all the little C++ thingies that will
    // be generated. They're not relevant to IPDL itself, but
    // those ("invisible") symbols can clash with others in the
    // IPDL spec, and we'd like to catch those before C++ compilers
    // are allowed to obfuscate the error.

    sym_tab.exit_scope();

    errors
}

fn gather_decls_tu(
    tus: &HashMap<TUId, TranslationUnit>,
    tuts: &mut HashMap<TUId, TranslationUnitType>,
    tuid: &TUId,
    tu: &TranslationUnit,
) -> Errors {
    let mut errors = Errors::none();
    let mut sym_tab = SymbolTable::new();

    if let &Some(ref p) = &tu.protocol {
        errors.append(declare_protocol(&mut sym_tab, &tuid, &p.0, &p.1));
    }

    // Add the declarations from all the IPDL files we include.
    for include_tuid in &tu.includes {
        let include_tu = tus.get(include_tuid).unwrap();
        match include_tu.protocol {
            Some(ref p) => errors.append(declare_protocol(&mut sym_tab, &include_tuid, &p.0, &p.1)),
            None => {
                // This is a header.  Import its "exported" globals into our scope.
                errors.append(declare_usings(&mut sym_tab, &include_tu));
                errors.append(declare_structs_and_unions(
                    &mut sym_tab,
                    &include_tuid,
                    &include_tu,
                ));
            }
        }
    }

    // Declare builtin C types.
    let builtin = Location::builtin();
    for &t in BUILTIN_C_TYPES {
        errors.append(sym_tab.declare(Decl::new(
            &builtin,
            IPDLType::BuiltinCType(t),
            t.to_owned(),
        )));
    }

    // Declare builtin C++ types.
    for t in BUILTIN_TYPES {
        let cxx_type = builtin_from_string(t);
        errors.append(declare_cxx_type(
            &mut sym_tab,
            &cxx_type,
            Lifetime::ManualDealloc, /* refcounted */
            false,                   /* send moveonly */
            false,                   /* data moveonly */
        ));
    }

    // Declare imported C++ types.
    errors.append(declare_usings(&mut sym_tab, &tu));

    // Get a copy of the translation unit type so that we can still
    // use |tuts| to look up things for error messages. An alternative
    // would be to extract some kind of mapping from tuids to the name
    // of protocols, structs and unions and use that.
    let mut tut = (*tuts.get(tuid).unwrap()).clone();

    let struct_union_attributes: AttributeSpec = HashMap::from([("Comparable", Vec::new())]);

    // Create stubs for top level struct and union decls.
    for s in &tu.structs {
        errors.append(check_attributes(&s.1, &struct_union_attributes));
        tut.structs
            .push(StructTypeDef::new(&s.0, s.1.contains_key("Comparable")));
    }
    for u in &tu.unions {
        errors.append(check_attributes(&u.1, &struct_union_attributes));
        tut.unions
            .push(UnionTypeDef::new(&u.0, u.1.contains_key("Comparable")));
    }

    // Forward declare all structs and unions in order to support
    // recursive definitions.
    errors.append(declare_structs_and_unions(&mut sym_tab, &tuid, &tu));

    // Check definitions of structs and unions.
    // XXX It might be cleaner to do a zip iteration over {tu,tut}.structs
    let mut index = 0;
    for su in &tu.structs {
        errors.append(gather_decls_struct(
            &mut sym_tab,
            &tuts,
            &su,
            &mut tut.structs[index],
        ));
        index += 1;
    }
    index = 0;
    for u in &tu.unions {
        errors.append(gather_decls_union(
            &mut sym_tab,
            &tuts,
            &u,
            &mut tut.unions[index],
        ));
        index += 1;
    }

    // The Python version type checks every struct and union included
    // from an ipdlh file here, but I don't think that makes any
    // sense.

    if let &Some(ref p) = &tu.protocol {
        errors.append(gather_decls_protocol(
            &mut sym_tab,
            &tuts,
            &tuid,
            &p,
            &mut tut.protocol.as_mut().unwrap(),
        ));
    }

    // Now that we've updated |tut|, replace it in |tuts|.
    tuts.insert(tuid.clone(), tut);

    errors
}

enum FullyDefinedState {
    Visiting,
    Defined(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CompoundType {
    Struct,
    Union,
}

/* The rules for "full definition" of a type are
    defined(atom)             := true
    defined(array basetype)   := defined(basetype)
    defined(struct f1 f2...)  := defined(f1) and defined(f2) and ...
    defined(union c1 c2 ...)  := defined(c1) or defined(c2) or ...
*/
fn fully_defined(
    tuts: &HashMap<TUId, TranslationUnitType>,
    mut defined: &mut HashMap<(CompoundType, TypeRef), FullyDefinedState>,
    t: &IPDLType,
) -> bool {
    let key = match t {
        &IPDLType::StructType(ref tr) => (CompoundType::Struct, tr.clone()),
        &IPDLType::UnionType(ref tr) => (CompoundType::Union, tr.clone()),
        &IPDLType::ArrayType(ref t_inner) => return fully_defined(&tuts, &mut defined, &t_inner),
        &IPDLType::MaybeType(ref t_inner) => return fully_defined(&tuts, &mut defined, &t_inner),
        &IPDLType::UniquePtrType(ref t_inner) => {
            return fully_defined(&tuts, &mut defined, &t_inner)
        }
        &IPDLType::NotNullType(ref t_inner) => return fully_defined(&tuts, &mut defined, &t_inner),

        &IPDLType::BuiltinCType(_) => return true,
        &IPDLType::ImportedCxxType(_, _, _, _) => return true,
        &IPDLType::MessageType(_) => return true,
        &IPDLType::ProtocolType(_) => return true,
        &IPDLType::ActorType(_) => return true,
        &IPDLType::ShmemType(_) => return true,
        &IPDLType::ByteBufType(_) => return true,
        &IPDLType::FDType(_) => return true,
        &IPDLType::EndpointType(_) => return true,
        &IPDLType::ManagedEndpointType(_) => return true,
    };

    // The Python version would repeatedly visit a type that was found
    // to be not defined. I think that's unnecessary. Not doing it
    // might save some time in the case of an error.

    if let Some(state) = defined.get(&key) {
        return match state {
            &FullyDefinedState::Visiting => false,
            &FullyDefinedState::Defined(is_defined) => is_defined,
        };
    }

    defined.insert(key.clone(), FullyDefinedState::Visiting);

    let mut is_defined;
    match key.0 {
        CompoundType::Struct => {
            is_defined = true;
            for f in &key.1.lookup_struct(&tuts).fields {
                if !fully_defined(&tuts, &mut defined, f) {
                    is_defined = false;
                    break;
                }
            }
        }
        CompoundType::Union => {
            is_defined = false;
            for f in &key.1.lookup_union(&tuts).components {
                if fully_defined(&tuts, &mut defined, f) {
                    is_defined = true;
                    break;
                }
            }
        }
    }

    // XXX Don't need to insert here. get_mut should work.
    defined.insert(key, FullyDefinedState::Defined(is_defined));

    return is_defined;
}

enum ManagerCycleState {
    Visiting,
    Acyclic,
}

fn protocol_managers_cycles(
    tuts: &HashMap<TUId, TranslationUnitType>,
    mut visited: &mut HashMap<TUId, ManagerCycleState>,
    mut stack: &mut Vec<TUId>,
    tuid: &TUId,
) -> Vec<String> {
    if let Some(state) = visited.get(tuid) {
        return match state {
            &ManagerCycleState::Visiting => {
                let cycle_names: Vec<String> = stack
                    .iter()
                    .chain([tuid.clone()].iter())
                    .map(|p| get_protocol_type(&tuts, &p).qname.short_name())
                    .collect::<Vec<String>>();
                vec![format!("`{}'", cycle_names.join(" -> "))]
            }
            &ManagerCycleState::Acyclic => Vec::new(),
        };
    }

    let mut cycles = Vec::new();

    visited.insert(tuid.clone(), ManagerCycleState::Visiting);

    stack.push(tuid.clone());

    let pt = get_protocol_type(&tuts, &tuid);
    for managee in &pt.manages {
        // Self-managed protocols are allowed, except at the top level.
        // The top level case is checked in protocol_managers_cycles.
        if managee == tuid {
            continue;
        }

        cycles.append(&mut protocol_managers_cycles(
            &tuts,
            &mut visited,
            &mut stack,
            &managee,
        ));
    }

    stack.pop();

    *visited.get_mut(tuid).unwrap() = ManagerCycleState::Acyclic;

    cycles
}

fn protocols_managers_acyclic(tuts: &HashMap<TUId, TranslationUnitType>) -> Errors {
    let mut errors = Errors::none();

    for (tuid, tut) in tuts {
        if tut.protocol.is_none() {
            continue;
        }
        let pt = get_protocol_type(&tuts, &tuid);

        // To match the behavior of the Python IPDL compiler in error cases, reset the
        // visited stack after each protocol.
        let mut visited = HashMap::new();
        let mut stack = Vec::new();

        let cycles = protocol_managers_cycles(&tuts, &mut visited, &mut stack, &tuid);
        if cycles.len() > 0 {
            errors.append_one(
                pt.qname.loc(),
                &format!(
                    "cycle(s) detected in manager/manages hierarchy: {}",
                    cycles.join(", ")
                ),
            );
        }

        if pt.managers.len() == 1 && &pt.managers[0] == tuid {
            errors.append_one(
                pt.qname.loc(),
                &format!(
                    "top-level protocol `{}' cannot manage itself",
                    pt.qname.short_name()
                ),
            );
        }
    }
    errors
}

fn check_types_message(ptype: &ProtocolTypeDef, mtype: &MessageTypeDef) -> Errors {
    let mut errors = Errors::none();
    let mname = &mtype.name.id;

    if mtype.nested.inside_sync() && !mtype.is_sync() {
        errors.append_one(
            &mtype.name.loc,
            &format!(
                "inside_sync nested messages must be sync (here, message `{}' in protocol `{}')",
                mname,
                ptype.qname.short_name()
            ),
        );
    }

    if mtype.nested.inside_cpow() && mtype.direction.is_to_child() {
        errors.append_one(&mtype.name.loc,
                          &format!("inside_cpow nested parent-to-child messages are verboten (here, message `{}' in protocol `{}')",
                                   mname, ptype.qname.short_name()));
    }

    // We allow inside_sync messages that are themselves sync to be sent from the
    // parent. Normal and inside_cpow nested messages that are sync can only come from
    // the child.
    if mtype.is_sync() && mtype.nested.is_none() && mtype.direction.is_to_child() {
        errors.append_one(
            &mtype.name.loc,
            &format!(
                "sync parent-to-child messages are verboten (here, message `{}' in protocol `{}')",
                mname,
                ptype.qname.short_name()
            ),
        );
    }

    if !mtype.converts_to(&ptype) {
        errors.append_one(&mtype.name.loc,
                          &format!("message `{}' requires more powerful send semantics than its protocol `{}' provides",
                                   mname, ptype.qname.short_name()));
    }

    if (mtype.is_ctor() || mtype.is_dtor()) && mtype.is_async() && mtype.returns.len() > 0 {
        errors.append_one(
            &mtype.name.loc,
            &format!(
                "asynchronous ctor/dtor message `{}' declares return values",
                mname
            ),
        );
    }

    if mtype.compress != Compress::None && (!mtype.is_async() || mtype.is_ctor() || mtype.is_dtor())
    {
        let pname = ptype.qname.short_name();
        let message;

        if mtype.is_ctor() || mtype.is_dtor() {
            let message_type = if mtype.is_ctor() {
                "constructor"
            } else {
                "destructor"
            };
            message = format!(
                "{} messages can't use compression (here, in protocol `{}')",
                message_type, pname
            );
        } else {
            message = format!(
                "message `{}' in protocol `{}' requests compression but is not async",
                mname, pname
            );
        }

        errors.append_one(&mtype.name.loc, &message);
    }

    if mtype.is_ctor() && !ptype.manages.contains(mtype.constructed_type()) {
        let ctor_protocol_len = mname.len() - CONSTRUCTOR_SUFFIX.len();
        errors.append_one(
            &mtype.name.loc,
            &format!(
                "ctor for protocol `{}', which is not managed by protocol `{}'",
                &mname[0..ctor_protocol_len],
                ptype.qname.short_name()
            ),
        );
    }

    errors
}

fn check_types_protocol(
    tuts: &HashMap<TUId, TranslationUnitType>,
    tuid: &TUId,
    ptype: &ProtocolTypeDef,
) -> Errors {
    let mut errors = Errors::none();

    // XXX Why does this check every translation unit and not just the current one???
    errors.append(protocols_managers_acyclic(&tuts));

    for manager in &ptype.managers {
        let manager_type = get_protocol_type(&tuts, &manager);
        if !ptype.converts_to(&manager_type) {
            errors.append_one(&ptype.qname.loc(),
                              &format!("protocol `{}' requires more powerful send semantics than its manager `{}' provides",
                                       ptype.qname.short_name(), manager_type.qname.short_name()));
        }

        if !manager_type.manages.contains(&tuid) {
            errors.append_one(&manager_type.qname.loc(),
                              &format!("|manager| declaration in protocol `{}' does not match any |manages| declaration in protocol `{}'",
                                       ptype.qname.short_name(), manager_type.qname.short_name()));
        }
    }

    for managee in &ptype.manages {
        let managee_type = get_protocol_type(&tuts, &managee);

        if !managee_type.managers.contains(&tuid) {
            errors.append_one(&managee_type.qname.loc(),
                              &format!("|manages| declaration in protocol `{}' does not match any |manager| declaration in protocol `{}'",
                                       ptype.qname.short_name(), managee_type.qname.short_name()));
        }
    }

    if ptype.send_semantics.is_intr() && ptype.nested != Nesting::None {
        errors.append_one(
            ptype.qname.loc(),
            &format!(
                "intr protocol `{}' cannot specify [NestedUpTo]",
                ptype.qname.short_name()
            ),
        );
    }

    for mtype in &ptype.messages {
        errors.append(check_types_message(&ptype, &mtype));
    }

    errors
}

fn check_types_tu(
    tus: &HashMap<TUId, TranslationUnit>,
    tuts: &HashMap<TUId, TranslationUnitType>,
    mut defined: &mut HashMap<(CompoundType, TypeRef), FullyDefinedState>,
    tuid: &TUId,
    tut: &TranslationUnitType,
) -> Errors {
    let mut errors = Errors::none();

    let tu = tus.get(tuid).unwrap();

    for i in 0..tut.structs.len() {
        if !fully_defined(
            &tuts,
            &mut defined,
            &IPDLType::StructType(TypeRef::new(&tuid, i)),
        ) {
            errors.append_one(
                &tu.structs[i].0.name.loc,
                &format!(
                    "struct `{}' is only partially defined",
                    &tu.structs[i].0.name.id
                ),
            );
        }
    }

    for i in 0..tut.unions.len() {
        if !fully_defined(
            &tuts,
            &mut defined,
            &IPDLType::UnionType(TypeRef::new(&tuid, i)),
        ) {
            errors.append_one(
                &tu.unions[i].0.name.loc,
                &format!(
                    "union `{}' is only partially defined",
                    &tu.unions[i].0.name.id
                ),
            );
        }
    }

    if let &Some(ref pt) = &tut.protocol {
        errors.append(check_types_protocol(&tuts, &tuid, &pt));
    }

    // XXX We don't need to track visited because we will visited all
    // translation units at the top level.

    // XXX What is "ptype"? In Python, it is set to None at the top of this method.

    // XXX The Python checker calls visitIncludes on tu.includes,
    // which checks any included protocols. I don't know why that
    // would be useful.

    errors
}

// Basic checking that doesn't relate to types specifically.
pub fn check_translation_unit(tu: &TranslationUnit) -> Errors {
    if let &Some((ref ns, _)) = &tu.protocol {
        // For a protocol file, the filename should match the
        // protocol. (In the Python IPDL compiler, translation units have
        // a separate "name" field that is checked here, but for protocol
        // files the name is just the name of the protocol, and for
        // non-protocols the name is derived from the file name, so this
        // checking should be equivalent.)
        let base_file_name = tu
            .file_name
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let expected_file_name = ns.name.id.clone() + ".ipdl";
        if base_file_name != expected_file_name {
            return Errors::one(&tu.namespace.name.loc,
                               &format!("expected file for translation unit `{}' to be named `{}'; instead it's named `{}'.",
                                        tu.namespace.name.id, expected_file_name, base_file_name));
        }
    }

    Errors::none()
}

pub fn check(tus: &HashMap<TUId, TranslationUnit>) -> Result<(), String> {
    let mut tuts = HashMap::new();

    // XXX This ordering should be deterministic. I could sort by the
    // TUId.

    let tus_vec = tus.iter().collect::<Vec<_>>();
    let mut errors = Errors::none();

    // XXX Should we get all errors first? Probably...
    for &(tuid, tu) in &tus_vec {
        errors.append(check_translation_unit(&tu));

        // Create top-level type decl for all protocols.
        let old_entry = tuts.insert(tuid.clone(), TranslationUnitType::new(&tu.protocol));
        assert!(old_entry.is_none());
    }

    // Bail out here if we have errors.
    //errors.to_result()?;

    for &(tuid, tu) in &tus_vec {
        errors.append(gather_decls_tu(&tus, &mut tuts, &tuid, &tu));
    }

    let tuts_vec = tuts.iter().collect::<Vec<_>>();
    let mut defined = HashMap::new();
    for &(tuid, tut) in &tuts_vec {
        errors.append(check_types_tu(&tus, &tuts, &mut defined, &tuid, &tut));
    }

    errors.to_result()
}
