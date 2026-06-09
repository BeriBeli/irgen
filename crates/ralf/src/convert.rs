use irgen_snapsheet::model as snapsheet_model;

use crate::ast::*;
use crate::error::Error;
use crate::serialize::serialize_document;
use crate::util::{access_from_attr, bytes_from_bits, ralf_number, sanitize_doc};

pub fn serialize_ralf(component: &snapsheet_model::Component) -> Result<String, Error> {
    Ok(serialize_document(&component_to_document(component)?))
}

pub fn component_to_document(component: &snapsheet_model::Component) -> Result<Document, Error> {
    let mut items = Vec::new();
    let blocks = component
        .blks()
        .iter()
        .filter(|block| !is_empty_block(block))
        .collect::<Vec<_>>();

    for block in &blocks {
        items.push(TopLevelItem::Block(block_from_base(block)?));
    }

    if blocks.len() > 1 {
        let mut system = System {
            name: component.name().into(),
            body: HierarchyBody {
                bytes: Some("4".into()),
                ..HierarchyBody::default()
            },
            ..System::default()
        };
        for block in blocks {
            system.body.blocks.push(BlockInstance {
                name: block.name().into(),
                offset: ralf_number("block offset", block.offset())?,
                ..BlockInstance::default()
            });
        }
        items.push(TopLevelItem::System(system));
    }

    Ok(Document { items })
}

fn is_empty_block(block: &snapsheet_model::Block) -> bool {
    block.regs().is_empty() && block.register_files().is_empty()
}

fn block_from_base(block: &snapsheet_model::Block) -> Result<Block, Error> {
    let mut body = AddressableBody {
        bytes: Some(bytes_from_bits(block.name(), block.size())?.to_string()),
        ..AddressableBody::default()
    };

    for register in block.regs() {
        body.registers.push(register_instance_from_base(
            register,
            Some(register.offset()),
        )?);
    }

    for register_file in block.register_files() {
        body.regfiles
            .push(regfile_instance_from_base(register_file)?);
    }

    Ok(Block {
        name: block.name().into(),
        body,
        ..Block::default()
    })
}

fn regfile_instance_from_base(
    register_file: &snapsheet_model::RegisterFile,
) -> Result<RegFileInstance, Error> {
    let mut regfile = RegFile {
        name: register_file.name().into(),
        ..RegFile::default()
    };

    for register in register_file.regs() {
        regfile.registers.push(register_instance_from_base(
            register,
            Some(register.offset()),
        )?);
    }

    Ok(RegFileInstance {
        name: register_file.name().into(),
        array: Some(Array::Count(register_file.dim().into())),
        offset: Some(ralf_number("register file offset", register_file.offset())?),
        increment: Some(ralf_number("register file stride", register_file.range())?),
        definition: Some(regfile),
        ..RegFileInstance::default()
    })
}

fn register_instance_from_base(
    register: &snapsheet_model::Register,
    offset: Option<&str>,
) -> Result<RegisterInstance, Error> {
    let mut definition = Register {
        name: register.name().into(),
        bytes: Some(bytes_from_bits(register.name(), register.size())?.to_string()),
        ..Register::default()
    };

    for field in register.fields() {
        definition.fields.push(FieldInstance {
            name: field.name().into(),
            hdl_path: field_hdl_path(field),
            offset: Some(ralf_number("field bit offset", field.offset())?),
            definition: Some(field_from_base(field)?),
            ..FieldInstance::default()
        });
    }

    Ok(RegisterInstance {
        name: register.name().into(),
        array: register.array().map(|array| {
            Array::Count(
                array
                    .dims()
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>()
                    .join("*"),
            )
        }),
        offset: offset
            .map(|value| ralf_number("register offset", value))
            .transpose()?,
        increment: register
            .array()
            .and_then(|array| array.stride())
            .map(|stride| ralf_number("register stride", stride))
            .transpose()?,
        definition: Some(definition),
        ..RegisterInstance::default()
    })
}

fn field_from_base(field: &snapsheet_model::Field) -> Result<Field, Error> {
    Ok(Field {
        name: field.name().into(),
        bits: Some(field.width().into()),
        access: Some(access_from_attr(field.attr())?),
        hard_reset: has_reset(field.reset())
            .then(|| ralf_number("field reset", field.reset()))
            .transpose()?,
        doc: (!field.desc().trim().is_empty()).then(|| sanitize_doc(field.desc())),
        ..Field::default()
    })
}

fn has_reset(reset: &str) -> bool {
    let reset = reset.trim();
    !reset.is_empty() && reset != "-"
}

fn field_hdl_path(field: &snapsheet_model::Field) -> Option<String> {
    if is_reserved_field_name(field.name()) {
        return None;
    }

    field.hdl_path().map(str::to_owned)
}

fn is_reserved_field_name(field_name: &str) -> bool {
    let lower = field_name.to_ascii_lowercase();
    let suffix = lower
        .strip_prefix("reserved")
        .or_else(|| lower.strip_prefix("rsvd"));

    suffix.is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}
