use irgen_model::base;

use crate::ast::*;
use crate::error::Error;
use crate::serialize::serialize_document;
use crate::util::{access_from_attr, bytes_from_bits, ralf_number, sanitize_doc};

pub fn serialize_ralf(component: &base::Component) -> Result<String, Error> {
    Ok(serialize_document(&component_to_document(component)?))
}

pub fn component_to_document(component: &base::Component) -> Result<Document, Error> {
    let mut items = Vec::new();
    for block in component.blks() {
        items.push(TopLevelItem::Block(block_from_base(block)?));
    }

    if component.blks().len() > 1 {
        let mut system = System {
            name: component.name().into(),
            body: HierarchyBody {
                bytes: Some("4".into()),
                ..HierarchyBody::default()
            },
            ..System::default()
        };
        for block in component.blks() {
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

fn block_from_base(block: &base::Block) -> Result<Block, Error> {
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
    register_file: &base::RegisterFile,
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
    register: &base::Register,
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
            offset: Some(ralf_number("field bit offset", field.offset())?),
            definition: Some(field_from_base(field)?),
            ..FieldInstance::default()
        });
    }

    Ok(RegisterInstance {
        name: register.name().into(),
        offset: offset
            .map(|value| ralf_number("register offset", value))
            .transpose()?,
        definition: Some(definition),
        ..RegisterInstance::default()
    })
}

fn field_from_base(field: &base::Field) -> Result<Field, Error> {
    Ok(Field {
        name: field.name().into(),
        bits: Some(field.width().into()),
        access: Some(access_from_attr(field.attr())?),
        hard_reset: (!field.reset().trim().is_empty())
            .then(|| ralf_number("field reset", field.reset()))
            .transpose()?,
        doc: (!field.desc().trim().is_empty()).then(|| sanitize_doc(field.desc())),
        ..Field::default()
    })
}
