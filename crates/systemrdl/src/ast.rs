#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    pub package: Option<String>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub path: String,
    pub wildcard: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Enum(EnumDecl),
    Struct(StructDecl),
    Property(PropertyDecl),
    Component(Component),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub ty: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDecl {
    pub name: String,
    pub ty: PropertyType,
    pub component_kinds: Vec<ComponentKind>,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Boolean,
    Number,
    String,
    Ref,
    Enum(String),
    User(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentKind {
    AddrMap,
    RegFile,
    Reg,
    Field,
    Mem,
    Signal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    pub kind: ComponentKind,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub properties: Vec<PropertyAssignment>,
    pub children: Vec<ComponentChild>,
    pub instances: Vec<Instance>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentChild {
    Component(Component),
    Instance(Instance),
    Property(PropertyAssignment),
    Constraint(Constraint),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub component: Box<Component>,
    pub name: String,
    pub array: Option<Array>,
    pub range: Option<BitRange>,
    pub address: Option<Expression>,
    pub stride: Option<Expression>,
    pub reset: Option<Expression>,
    pub instance_properties: Vec<PropertyAssignment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    pub dimensions: Vec<ArrayDimension>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayDimension {
    Count(Expression),
    Range { left: Expression, right: Expression },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitRange {
    pub msb: Expression,
    pub lsb: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyAssignment {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub name: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),
    EnumRef(String),
    Array(Vec<Expression>),
    Struct(Vec<(String, Expression)>),
    Raw(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareAccess {
    Rw,
    R,
    W,
    Rw1,
    W1,
    Na,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareAccess {
    Rw,
    R,
    W,
    Na,
}

impl Component {
    pub fn new(kind: ComponentKind, name: impl Into<String>) -> Self {
        Self {
            kind,
            name: name.into(),
            parameters: Vec::new(),
            properties: Vec::new(),
            children: Vec::new(),
            instances: Vec::new(),
        }
    }
}

impl Instance {
    pub fn new(component: Component, name: impl Into<String>) -> Self {
        Self {
            component: Box::new(component),
            name: name.into(),
            array: None,
            range: None,
            address: None,
            stride: None,
            reset: None,
            instance_properties: Vec::new(),
        }
    }
}

impl PropertyAssignment {
    pub fn bool(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: None,
        }
    }

    pub fn value(name: impl Into<String>, value: Expression) -> Self {
        Self {
            name: name.into(),
            value: Some(value),
        }
    }
}

impl ComponentKind {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::AddrMap => "addrmap",
            Self::RegFile => "regfile",
            Self::Reg => "reg",
            Self::Field => "field",
            Self::Mem => "mem",
            Self::Signal => "signal",
        }
    }
}

impl PropertyType {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
            Self::Ref => "ref",
            Self::Enum(name) | Self::User(name) => name,
        }
    }
}

impl SoftwareAccess {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Rw => "rw",
            Self::R => "r",
            Self::W => "w",
            Self::Rw1 => "rw1",
            Self::W1 => "w1",
            Self::Na => "na",
        }
    }
}

impl HardwareAccess {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Rw => "rw",
            Self::R => "r",
            Self::W => "w",
            Self::Na => "na",
        }
    }
}
