use irgen_model::base::{Block as BaseBlock, Component, Field, Register, RegisterFile};

use crate::anchor::anchor_id;
use crate::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct DocumentView<'a> {
    pub(crate) component: &'a Component,
    pub(crate) blocks: Vec<BlockView<'a>>,
}

#[derive(Debug, Clone)]
pub(crate) struct BlockView<'a> {
    pub(crate) block: &'a BaseBlock,
    pub(crate) anchor: String,
    pub(crate) registers: Vec<RegisterView<'a>>,
}

#[derive(Debug, Clone)]
pub(crate) struct RegisterView<'a> {
    pub(crate) register: &'a Register,
    pub(crate) anchor: String,
    pub(crate) display_name: String,
    pub(crate) display_offset: String,
    pub(crate) source: RegisterSource,
    pub(crate) fields: Vec<FieldView<'a>>,
}

#[derive(Debug, Clone)]
pub(crate) enum RegisterSource {
    Direct,
    RegisterFile {
        dim: String,
        stride: String,
        base_offset: String,
        child_offset: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct FieldView<'a> {
    pub(crate) field: &'a Field,
    pub(crate) anchor: String,
    lsb: u64,
    msb: u64,
}

impl<'a> DocumentView<'a> {
    pub(crate) fn new(component: &'a Component) -> Result<Self, Error> {
        let mut blocks = Vec::new();
        for block in component.blks() {
            blocks.push(BlockView::new(block)?);
        }
        Ok(Self { component, blocks })
    }
}

impl<'a> BlockView<'a> {
    fn new(block: &'a BaseBlock) -> Result<Self, Error> {
        let mut registers = block
            .regs()
            .iter()
            .map(|register| RegisterView::new_direct(block.name(), register))
            .collect::<Result<Vec<_>, _>>()?;

        for register_file in block.register_files() {
            registers.extend(expand_register_file(block.name(), register_file)?);
        }

        Ok(Self {
            block,
            anchor: anchor_id(&["block", block.name()]),
            registers,
        })
    }
}

impl<'a> RegisterView<'a> {
    fn new_direct(block_name: &str, register: &'a Register) -> Result<Self, Error> {
        Self::new(
            &["register", block_name],
            register,
            register.name().into(),
            register.offset().into(),
            RegisterSource::Direct,
        )
    }

    fn new(
        anchor_prefix: &[&str],
        register: &'a Register,
        display_name: String,
        display_offset: String,
        source: RegisterSource,
    ) -> Result<Self, Error> {
        let mut register_anchor = anchor_prefix.to_vec();
        register_anchor.push(&display_name);
        let anchor = anchor_id(&register_anchor);

        let fields = field_views(register, &register_anchor)?;
        Ok(Self {
            register,
            anchor,
            display_name,
            display_offset,
            source,
            fields,
        })
    }
}

impl FieldView<'_> {
    pub(crate) fn bits(&self) -> String {
        if self.msb == self.lsb {
            self.lsb.to_string()
        } else {
            format!("{}:{}", self.msb, self.lsb)
        }
    }

    pub(crate) fn bit_width(&self) -> u64 {
        self.msb - self.lsb + 1
    }
}

fn field_views<'a>(
    register: &'a Register,
    register_anchor: &[&str],
) -> Result<Vec<FieldView<'a>>, Error> {
    let register_size = parse_u64("register size", register.size())?;
    let mut fields = Vec::new();
    for field in register.fields() {
        let lsb = parse_u64("field bit offset", field.offset())?;
        let width = parse_u64("field width", field.width())?;
        let msb = lsb + width.saturating_sub(1);
        if width == 0 || msb >= register_size {
            return Err(Error::FieldOutOfRange {
                register: register.name().into(),
                field: field.name().into(),
                msb,
                lsb,
                size: register_size,
            });
        }

        let mut field_anchor = register_anchor.to_vec();
        field_anchor[0] = "field";
        field_anchor.push(field.name());
        fields.push(FieldView {
            field,
            anchor: anchor_id(&field_anchor),
            lsb,
            msb,
        });
    }

    fields.sort_by(|left, right| {
        right
            .msb
            .cmp(&left.msb)
            .then_with(|| right.lsb.cmp(&left.lsb))
            .then_with(|| left.field.name().cmp(right.field.name()))
    });
    Ok(fields)
}

fn expand_register_file<'a>(
    block_name: &str,
    register_file: &'a RegisterFile,
) -> Result<Vec<RegisterView<'a>>, Error> {
    let base_offset = parse_literal("register file offset", register_file.offset())?;
    let dim = parse_literal("register file dimension", register_file.dim())?;
    let mut registers = Vec::new();

    for register in register_file.regs() {
        let child_offset = parse_literal("register offset", register.offset())?;
        let display_name = unexpanded_register_file_name(register_file, register, dim);
        let display_offset_value =
            base_offset
                .checked_add(child_offset)
                .ok_or_else(|| Error::AddressOverflow {
                    kind: "register file register offset",
                    name: display_name.clone(),
                })?;
        let display_offset = format_hex(display_offset_value);
        registers.push(RegisterView::new(
            &["register", block_name],
            register,
            display_name,
            display_offset,
            RegisterSource::RegisterFile {
                dim: register_file.dim().into(),
                stride: register_file.range().into(),
                base_offset: register_file.offset().into(),
                child_offset: register.offset().into(),
            },
        )?);
    }

    Ok(registers)
}

fn unexpanded_register_file_name(
    register_file: &RegisterFile,
    register: &Register,
    dim: u64,
) -> String {
    if register_file.regs().len() == 1 && register.name() == register_file.name() {
        format!("{}[{dim}]", register_file.name())
    } else {
        format!("{}[{dim}].{}", register_file.name(), register.name())
    }
}

fn parse_u64(kind: &'static str, value: &str) -> Result<u64, Error> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|_| Error::InvalidNumber {
            kind,
            value: value.into(),
        })
}

fn parse_literal(kind: &'static str, value: &str) -> Result<u64, Error> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return u64::from_str_radix(hex, 16).map_err(|_| Error::InvalidNumber {
            kind,
            value: value.into(),
        });
    }

    parse_u64(kind, trimmed)
}

fn format_hex(value: u64) -> String {
    format!("0x{value:X}")
}
