use irgen_model::base;

use crate::ast::*;
use crate::error::Error;
use crate::serialize::serialize_document;
use crate::util::{access_properties, bytes_from_bits, rdl_number, sanitize_string};

pub fn serialize_systemrdl(component: &base::Component) -> Result<String, Error> {
    Ok(serialize_document(&component_to_document(component)?))
}

pub fn component_to_document(component: &base::Component) -> Result<Document, Error> {
    let mut top = Component::new(ComponentKind::AddrMap, component.name());
    top.properties.push(PropertyAssignment::value(
        "name",
        Expression::String(sanitize_string(component.name())),
    ));

    for block in component.blks() {
        let block_component = block_to_addrmap(block)?;
        let mut block_instance = Instance::new(block_component, block.name());
        block_instance.address = Some(rdl_number("block offset", block.offset())?);
        block_instance
            .instance_properties
            .push(PropertyAssignment::value(
                "hdl_path",
                Expression::String(block_hdl_path_macro(block.name())),
            ));
        top.instances.push(block_instance);
    }

    Ok(Document {
        package: None,
        imports: Vec::new(),
        declarations: vec![Declaration::Component(top)],
    })
}

fn block_to_addrmap(block: &base::Block) -> Result<Component, Error> {
    let mut addrmap = Component::new(ComponentKind::AddrMap, block.name());
    addrmap.properties.push(PropertyAssignment::value(
        "name",
        Expression::String(sanitize_string(block.name())),
    ));

    for register in block.regs() {
        addrmap.instances.push(register_instance(register)?);
    }

    for register_file in block.register_files() {
        let mut regfile = Component::new(ComponentKind::RegFile, register_file.name());
        for register in register_file.regs() {
            regfile.instances.push(register_instance(register)?);
        }
        let mut instance = Instance::new(regfile, register_file.name());
        instance.array = Some(Array {
            dimensions: vec![ArrayDimension::Count(Expression::Number(
                register_file.dim().into(),
            ))],
        });
        instance.address = Some(rdl_number("register file offset", register_file.offset())?);
        instance.stride = Some(rdl_number("register file stride", register_file.range())?);
        addrmap.instances.push(instance);
    }

    Ok(addrmap)
}

fn register_instance(register: &base::Register) -> Result<Instance, Error> {
    let mut reg = Component::new(ComponentKind::Reg, register.name());
    reg.properties.push(PropertyAssignment::value(
        "regwidth",
        Expression::Number(register.size().into()),
    ));
    reg.properties.push(PropertyAssignment::value(
        "accesswidth",
        Expression::Number((bytes_from_bits(register.name(), register.size())? * 8).to_string()),
    ));
    reg.properties.push(PropertyAssignment::value(
        "name",
        Expression::String(sanitize_string(register.name())),
    ));

    for field in register.fields() {
        reg.instances.push(field_instance(field)?);
    }

    let mut instance = Instance::new(reg, register.name());
    instance.address = Some(rdl_number("register offset", register.offset())?);
    Ok(instance)
}

fn field_instance(field: &base::Field) -> Result<Instance, Error> {
    let mut field_component = Component::new(ComponentKind::Field, field.name());
    field_component
        .properties
        .extend(access_properties(field.attr())?);
    if let Some(hdl_path) = field_hdl_path(field) {
        field_component.properties.push(PropertyAssignment::value(
            "hdl_path_slice",
            Expression::Array(vec![Expression::String(sanitize_string(&hdl_path))]),
        ));
    }

    let mut instance = Instance::new(field_component, field.name());
    let width = field
        .width()
        .parse::<u64>()
        .map_err(|_| Error::InvalidNumber {
            kind: "field width",
            value: field.width().into(),
        })?;
    let lsb = field
        .offset()
        .parse::<u64>()
        .map_err(|_| Error::InvalidNumber {
            kind: "field bit offset",
            value: field.offset().into(),
        })?;
    let msb = lsb + width - 1;
    instance.range = Some(BitRange {
        msb: Expression::Number(msb.to_string()),
        lsb: Some(Expression::Number(lsb.to_string())),
    });
    instance.reset = Some(rdl_number("field reset", field.reset())?);
    Ok(instance)
}

fn field_hdl_path(field: &base::Field) -> Option<String> {
    if is_reserved_field_name(field.name()) {
        return None;
    }

    field.hdl_path().map(str::to_owned)
}

fn block_hdl_path_macro(block_name: &str) -> String {
    let mut macro_name = String::from("`");
    for ch in block_name.chars() {
        if ch.is_ascii_alphanumeric() {
            macro_name.push(ch.to_ascii_uppercase());
        } else {
            macro_name.push('_');
        }
    }
    macro_name.push_str("_HDL_PATH");
    macro_name
}

fn is_reserved_field_name(field_name: &str) -> bool {
    let lower = field_name.to_ascii_lowercase();
    let suffix = lower
        .strip_prefix("reserved")
        .or_else(|| lower.strip_prefix("rsvd"));

    suffix.is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit()))
}
