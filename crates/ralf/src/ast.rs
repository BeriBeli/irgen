#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Document {
    pub items: Vec<TopLevelItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevelItem {
    Source(String),
    Field(Field),
    Register(Register),
    RegFile(RegFile),
    Memory(Memory),
    VirtualRegister(VirtualRegister),
    Block(Block),
    System(System),
    RegisterCallback(CallbackClass),
    FieldCallback(CallbackClass),
    Raw(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Attributes {
    pub entries: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Access {
    Rw,
    Ro,
    Wo,
    W1,
    W1c,
    Rc,
    Rs,
    Wrc,
    Wrs,
    Wc,
    Ws,
    Wsrc,
    Wcrs,
    W1s,
    W1t,
    W0c,
    W0s,
    W0t,
    W1src,
    W1crs,
    W0src,
    W0crs,
    Woc,
    Wos,
    Wo1,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
    FifoLs,
    FifoMs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceAccess {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Noise {
    Ro,
    Rw,
    No,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Initial {
    X,
    Zero,
    One,
    Address,
    Literal { value: String, step: Option<Step> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverKind {
    Address,
    Bits,
    FieldValues,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverDirective {
    pub include: bool,
    pub kind: CoverKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    pub name: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValue {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverpoint {
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cross {
    pub items: Vec<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCode {
    pub lang: Option<String>,
    pub scope: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallbackClass {
    pub name: String,
    pub var_declarations: Option<String>,
    pub new_method: Option<CallbackNewMethod>,
    pub pre_write_method: Option<String>,
    pub post_write_method: Option<String>,
    pub pre_read_method: Option<String>,
    pub post_read_method: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallbackNewMethod {
    pub args: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddRegCallback {
    pub target: Option<String>,
    pub callback_class: String,
    pub args: Option<String>,
    pub external_cb_class: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub bits: Option<String>,
    pub access: Option<Access>,
    pub hard_reset: Option<String>,
    pub soft_reset: Option<String>,
    pub volatile: Option<bool>,
    pub constraints: Vec<Constraint>,
    pub enum_values: Vec<EnumValue>,
    pub cover: Vec<CoverDirective>,
    pub coverpoints: Vec<Coverpoint>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FieldInstance {
    pub name: String,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub hdl_path: Option<String>,
    pub offset: Option<String>,
    pub increment: Option<String>,
    pub definition: Option<Field>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub bytes: Option<String>,
    pub left_to_right: bool,
    pub fields: Vec<FieldInstance>,
    pub constraints: Vec<Constraint>,
    pub noise: Option<Noise>,
    pub shared: Option<Option<String>>,
    pub cover: Vec<CoverDirective>,
    pub crosses: Vec<Cross>,
    pub user_codes: Vec<UserCode>,
    pub add_reg_callbacks: Vec<AddRegCallback>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegisterInstance {
    pub name: String,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub hdl_path: Option<String>,
    pub offset: Option<String>,
    pub increment: Option<String>,
    pub access: Option<InstanceAccess>,
    pub definition: Option<Register>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegFile {
    pub name: String,
    pub registers: Vec<RegisterInstance>,
    pub constraints: Vec<Constraint>,
    pub shared: Option<Option<String>>,
    pub cover: Vec<CoverDirective>,
    pub user_codes: Vec<UserCode>,
    pub add_reg_callbacks: Vec<AddRegCallback>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegFileInstance {
    pub name: String,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub hdl_path: Option<String>,
    pub offset: Option<String>,
    pub increment: Option<String>,
    pub access: Option<InstanceAccess>,
    pub definition: Option<RegFile>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Memory {
    pub name: String,
    pub size: Option<String>,
    pub bits: Option<String>,
    pub access: Option<Access>,
    pub initial: Option<Initial>,
    pub shared: Option<Option<String>>,
    pub cover: Vec<CoverDirective>,
    pub user_codes: Vec<UserCode>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryInstance {
    pub name: String,
    pub rename: Option<String>,
    pub hdl_path: Option<String>,
    pub offset: Option<String>,
    pub access: Option<InstanceAccess>,
    pub definition: Option<Memory>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VirtualRegister {
    pub name: String,
    pub bytes: Option<String>,
    pub left_to_right: bool,
    pub fields: Vec<VirtualFieldInstance>,
    pub user_codes: Vec<UserCode>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VirtualFieldInstance {
    pub name: String,
    pub rename: Option<String>,
    pub offset: Option<String>,
    pub bits: Option<String>,
    pub definition: Option<Field>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VirtualRegisterInstance {
    pub name: String,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub memory: Option<String>,
    pub memory_offset: Option<String>,
    pub increment: Option<String>,
    pub definition: Option<VirtualRegister>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Block {
    pub name: String,
    pub body: AddressableBody,
    pub domains: Vec<Domain<AddressableBody>>,
    pub default_map_name: Option<String>,
    pub user_codes: Vec<UserCode>,
    pub add_reg_callbacks: Vec<AddRegCallback>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct System {
    pub name: String,
    pub body: HierarchyBody,
    pub domains: Vec<Domain<HierarchyBody>>,
    pub user_codes: Vec<UserCode>,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Domain<T> {
    pub name: String,
    pub body: T,
    pub attributes: Attributes,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AddressableBody {
    pub bytes: Option<String>,
    pub endian: Option<Endian>,
    pub registers: Vec<RegisterInstance>,
    pub regfiles: Vec<RegFileInstance>,
    pub memories: Vec<MemoryInstance>,
    pub virtual_registers: Vec<VirtualRegisterInstance>,
    pub blocks: Vec<BlockInstance>,
    pub constraints: Vec<Constraint>,
    pub cover: Vec<CoverDirective>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HierarchyBody {
    pub bytes: Option<String>,
    pub endian: Option<Endian>,
    pub blocks: Vec<BlockInstance>,
    pub systems: Vec<SystemInstance>,
    pub constraints: Vec<Constraint>,
    pub cover: Vec<CoverDirective>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlockInstance {
    pub name: String,
    pub domain: Option<String>,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub hdl_path: Option<String>,
    pub offset: String,
    pub increment: Option<String>,
    pub definition: Option<Block>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SystemInstance {
    pub name: String,
    pub domain: Option<String>,
    pub rename: Option<String>,
    pub array: Option<Array>,
    pub hdl_path: Option<String>,
    pub offset: String,
    pub increment: Option<String>,
    pub definition: Option<System>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Array {
    Count(String),
    Range { msb: String, lsb: String },
}

impl Access {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Rw => "rw",
            Self::Ro => "ro",
            Self::Wo => "wo",
            Self::W1 => "w1",
            Self::W1c => "w1c",
            Self::Rc => "rc",
            Self::Rs => "rs",
            Self::Wrc => "wrc",
            Self::Wrs => "wrs",
            Self::Wc => "wc",
            Self::Ws => "ws",
            Self::Wsrc => "wsrc",
            Self::Wcrs => "wcrs",
            Self::W1s => "w1s",
            Self::W1t => "w1t",
            Self::W0c => "w0c",
            Self::W0s => "w0s",
            Self::W0t => "w0t",
            Self::W1src => "w1src",
            Self::W1crs => "w1crs",
            Self::W0src => "w0src",
            Self::W0crs => "w0crs",
            Self::Woc => "woc",
            Self::Wos => "wos",
            Self::Wo1 => "wo1",
            Self::Other(value) => value,
        }
    }
}

impl Endian {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Little => "little",
            Self::Big => "big",
            Self::FifoLs => "fifo_ls",
            Self::FifoMs => "fifo_ms",
        }
    }
}

impl InstanceAccess {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }
}

impl Noise {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Ro => "ro",
            Self::Rw => "rw",
            Self::No => "no",
        }
    }
}

impl Initial {
    pub(crate) fn as_str(&self) -> String {
        match self {
            Self::X => "x".into(),
            Self::Zero => "0".into(),
            Self::One => "1".into(),
            Self::Address => "addr".into(),
            Self::Literal { value, step } => {
                let mut output = value.clone();
                if let Some(step) = step {
                    output.push_str(step.as_str());
                }
                output
            }
        }
    }
}

impl Step {
    fn as_str(&self) -> &str {
        match self {
            Self::Increment => "++",
            Self::Decrement => "--",
        }
    }
}

impl CoverDirective {
    pub(crate) fn as_str(&self) -> String {
        let sign = if self.include { "+" } else { "-" };
        format!("{sign}{}", self.kind.as_str())
    }
}

impl CoverKind {
    fn as_str(&self) -> &str {
        match self {
            Self::Address => "a",
            Self::Bits => "b",
            Self::FieldValues => "f",
        }
    }
}

impl Array {
    pub(crate) fn as_str(&self) -> String {
        match self {
            Self::Count(count) => format!("[{count}]"),
            Self::Range { msb, lsb } => format!("[{msb}:{lsb}]"),
        }
    }
}
