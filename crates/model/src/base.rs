use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    vendor: String,
    library: String,
    name: String,
    version: String,
    blks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: String,
    offset: String,
    range: String,
    size: String,
    regs: Vec<Register>,
    register_files: Vec<RegisterFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    name: String,
    offset: String,
    size: String,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    csr_setting: Option<String>,
    fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterFile {
    name: String,
    offset: String,
    range: String,
    dim: String,
    regs: Vec<Register>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    offset: String,
    width: String,
    attr: String,
    reset: String,
    desc: String,
    #[serde(default)]
    hdl_path: Option<String>,
}

impl Component {
    pub fn new(
        vendor: String,
        library: String,
        name: String,
        version: String,
        blks: Vec<Block>,
    ) -> Self {
        Self {
            vendor,
            library,
            name,
            version,
            blks,
        }
    }

    pub fn vendor(&self) -> &str {
        &self.vendor
    }
    pub fn library(&self) -> &str {
        &self.library
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn blks(&self) -> &[Block] {
        &self.blks
    }
}

impl Block {
    pub fn new(
        name: String,
        offset: String,
        range: String,
        size: String,
        regs: Vec<Register>,
    ) -> Self {
        Self {
            name,
            offset,
            range,
            size,
            regs,
            register_files: Vec::new(),
        }
    }

    pub fn new_with_register_files(
        name: String,
        offset: String,
        range: String,
        size: String,
        regs: Vec<Register>,
        register_files: Vec<RegisterFile>,
    ) -> Self {
        Self {
            name,
            offset,
            range,
            size,
            regs,
            register_files,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn range(&self) -> &str {
        &self.range
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn regs(&self) -> &[Register] {
        &self.regs
    }
    pub fn register_files(&self) -> &[RegisterFile] {
        &self.register_files
    }
}

impl Register {
    pub fn new(name: String, offset: String, size: String, fields: Vec<Field>) -> Self {
        Self::new_with_description(name, offset, size, String::new(), fields)
    }

    pub fn new_with_description(
        name: String,
        offset: String,
        size: String,
        desc: String,
        fields: Vec<Field>,
    ) -> Self {
        Self::new_with_description_and_csr_setting(name, offset, size, desc, None, fields)
    }

    pub fn new_with_description_and_csr_setting(
        name: String,
        offset: String,
        size: String,
        desc: String,
        csr_setting: Option<String>,
        fields: Vec<Field>,
    ) -> Self {
        Self {
            name,
            offset,
            size,
            desc,
            csr_setting,
            fields,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn desc(&self) -> &str {
        &self.desc
    }
    pub fn csr_setting(&self) -> Option<&str> {
        self.csr_setting.as_deref()
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}

impl RegisterFile {
    pub fn new(
        name: String,
        offset: String,
        range: String,
        dim: String,
        regs: Vec<Register>,
    ) -> Self {
        Self {
            name,
            offset,
            range,
            dim,
            regs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn range(&self) -> &str {
        &self.range
    }
    pub fn dim(&self) -> &str {
        &self.dim
    }
    pub fn regs(&self) -> &[Register] {
        &self.regs
    }
}

impl Field {
    pub fn new(
        name: String,
        offset: String,
        width: String,
        attr: String,
        reset: String,
        desc: String,
    ) -> Self {
        Self::new_with_hdl_path(name.clone(), offset, width, attr, reset, desc, Some(name))
    }

    pub fn new_with_hdl_path(
        name: String,
        offset: String,
        width: String,
        attr: String,
        reset: String,
        desc: String,
        hdl_path: Option<String>,
    ) -> Self {
        Self {
            name,
            offset,
            width,
            attr,
            reset,
            desc,
            hdl_path,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn width(&self) -> &str {
        &self.width
    }
    pub fn attr(&self) -> &str {
        &self.attr
    }
    pub fn reset(&self) -> &str {
        &self.reset
    }
    pub fn desc(&self) -> &str {
        &self.desc
    }
    pub fn hdl_path(&self) -> Option<&str> {
        self.hdl_path.as_deref()
    }
}
