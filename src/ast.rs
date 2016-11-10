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
    spec: QualifiedId
}

impl TypeSpec {
    pub fn new(spec: QualifiedId) -> TypeSpec {
        TypeSpec { spec: spec }
    }
}

#[derive(Debug)]
pub enum Node {
    CxxInclude(String),
    Include(IncludeType, String),
    TypeSpec(TypeSpec),
}
