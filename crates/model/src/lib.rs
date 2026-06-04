pub mod attr;
pub mod base;
pub mod error;

// use regex::Regex;

use attr::{extract_access_value, extract_modified_write_value, extract_read_action_value};
use error::Error;
use ip_xact::v2009::types as ipxact2009;
use ip_xact::v2014::types as ipxact;
use ip_xact::v2022::types as ipxact2022;

pub fn serialize_ipxact_xml(base: &base::Component) -> Result<String, Error> {
    Ok(quick_xml::se::to_string(&ipxact::Component::try_from(
        base,
    )?)?)
}

pub fn serialize_ipxact_2009_xml(base: &base::Component) -> Result<String, Error> {
    Ok(quick_xml::se::to_string(&ipxact2009::Component::try_from(
        base,
    )?)?)
}

pub fn serialize_ipxact_2022_xml(base: &base::Component) -> Result<String, Error> {
    Ok(quick_xml::se::to_string(&ipxact2022::Component::try_from(
        base,
    )?)?)
}

impl TryFrom<&base::Component> for ipxact2009::Component {
    type Error = Error;

    fn try_from(base: &base::Component) -> Result<Self, Error> {
        let mut memory_map = ipxact2009::MemoryMap::new(base.name());
        for block in base.blks() {
            memory_map.address_block.push(block_to_ipxact_2009(block)?);
        }

        let mut memory_maps = ipxact2009::MemoryMaps::default();
        memory_maps.memory_map.push(memory_map);

        let mut component =
            ipxact2009::Component::new(base.vendor(), base.library(), base.name(), base.version());
        component.memory_maps = Some(memory_maps);

        Ok(component)
    }
}

fn block_to_ipxact_2009(block: &base::Block) -> Result<ipxact2009::AddressBlock, Error> {
    let mut address_block =
        ipxact2009::AddressBlock::new(block.name(), block.offset(), block.range(), block.size());

    for register in block.regs() {
        address_block
            .register
            .push(register_to_ipxact_2009(register)?);
    }

    for register_file in block.register_files() {
        address_block
            .register_file
            .push(register_file_to_ipxact_2009(register_file)?);
    }

    Ok(address_block)
}

fn register_file_to_ipxact_2009(
    register_file: &base::RegisterFile,
) -> Result<ipxact2009::RegisterFile, Error> {
    let mut ipxact_register_file = ipxact2009::RegisterFile::new(
        register_file.name(),
        register_file.offset(),
        register_file.range(),
    );
    ipxact_register_file
        .dim
        .push(register_file.dim().to_owned());

    for register in register_file.regs() {
        ipxact_register_file
            .register
            .push(register_to_ipxact_2009(register)?);
    }

    Ok(ipxact_register_file)
}

fn register_to_ipxact_2009(register: &base::Register) -> Result<ipxact2009::Register, Error> {
    let mut ipxact_register =
        ipxact2009::Register::new(register.name(), register.offset(), register.size());

    for field in register.fields() {
        ipxact_register.field.push(field_to_ipxact_2009(field)?);
    }

    Ok(ipxact_register)
}

fn field_to_ipxact_2009(field: &base::Field) -> Result<ipxact2009::Field, Error> {
    let mut ipxact_field = ipxact2009::Field::new(field.name(), field.offset(), field.width());
    ipxact_field.description = non_empty_string(field.desc());
    ipxact_field.access = Some(access_value(field)?);
    ipxact_field.modified_write_value = modified_write_value(field)?;
    ipxact_field.read_action = read_action_value(field)?;
    Ok(ipxact_field)
}

impl TryFrom<&base::Component> for ipxact2022::Component {
    type Error = Error;

    fn try_from(base: &base::Component) -> Result<Self, Error> {
        let mut memory_map = ipxact2022::MemoryMap::new(base.name());
        for block in base.blks() {
            memory_map.address_block.push(block_to_ipxact_2022(block)?);
        }

        let mut memory_maps = ipxact2022::MemoryMaps::default();
        memory_maps.memory_map.push(memory_map);

        let mut component =
            ipxact2022::Component::new(base.vendor(), base.library(), base.name(), base.version());
        component.memory_maps = Some(memory_maps);

        Ok(component)
    }
}

fn block_to_ipxact_2022(block: &base::Block) -> Result<ipxact2022::AddressBlock, Error> {
    let mut address_block =
        ipxact2022::AddressBlock::new(block.name(), block.offset(), block.range(), block.size());

    for register in block.regs() {
        address_block
            .register
            .push(register_to_ipxact_2022(register)?);
    }

    for register_file in block.register_files() {
        address_block
            .register_file
            .push(register_file_to_ipxact_2022(register_file)?);
    }

    Ok(address_block)
}

fn register_file_to_ipxact_2022(
    register_file: &base::RegisterFile,
) -> Result<ipxact2022::RegisterFile, Error> {
    let mut ipxact_register_file = ipxact2022::RegisterFile::new(
        register_file.name(),
        register_file.offset(),
        register_file.range(),
        register_file.dim(),
    );

    for register in register_file.regs() {
        ipxact_register_file
            .register
            .push(register_to_ipxact_2022(register)?);
    }

    Ok(ipxact_register_file)
}

fn register_to_ipxact_2022(register: &base::Register) -> Result<ipxact2022::Register, Error> {
    let mut ipxact_register =
        ipxact2022::Register::new(register.name(), register.offset(), register.size());

    for field in register.fields() {
        ipxact_register.field.push(field_to_ipxact_2022(field)?);
    }

    Ok(ipxact_register)
}

fn field_to_ipxact_2022(field: &base::Field) -> Result<ipxact2022::Field, Error> {
    let mut ipxact_field = ipxact2022::Field::new(field.name(), field.offset(), field.width());
    ipxact_field.description = non_empty_string(field.desc());
    if !field.reset().is_empty() {
        ipxact_field.resets = Some(ipxact2022::Resets {
            reset: vec![ipxact2022::Reset::new(field.reset())],
        });
    }

    let mut access_policy = ipxact2022::FieldAccessPolicy::new();
    access_policy.access = Some(access_value(field)?);
    access_policy.modified_write_value = modified_write_value(field)?;
    access_policy.read_action = read_action_value(field)?;
    ipxact_field.field_access_policies = Some(ipxact2022::FieldAccessPolicies {
        field_access_policy: vec![access_policy],
    });

    Ok(ipxact_field)
}

impl TryFrom<&base::Component> for ipxact::Component {
    type Error = Error;

    fn try_from(base: &base::Component) -> Result<Self, Error> {
        // not considering removing reserved registers (some reserved's access is rw)
        // let re = Regex::new(r"^(rsvd|reserved)\d*$")?;

        let mut memory_map = ipxact::MemoryMap::new(base.name());
        for blk in base.blks() {
            let mut address_block =
                ipxact::AddressBlock::new(blk.name(), blk.offset(), blk.range(), blk.size());

            for reg in blk.regs() {
                address_block.add_register(register_to_ipxact(reg)?);
            }

            for register_file in blk.register_files() {
                let mut ipxact_register_file = ipxact::RegisterFile::new(
                    register_file.name(),
                    register_file.offset(),
                    register_file.range(),
                );
                ipxact_register_file
                    .dim
                    .push(ipxact::RegisterDim::new(register_file.dim()));

                for reg in register_file.regs() {
                    ipxact_register_file.add_register(register_to_ipxact(reg)?);
                }

                address_block
                    .register_data
                    .push(ipxact::RegisterData::RegisterFile(ipxact_register_file));
            }

            memory_map.add_address_block(address_block);
        }

        let mut memory_maps = ipxact::MemoryMaps::default();
        memory_maps.add(memory_map);

        let mut component =
            ipxact::Component::new(base.vendor(), base.library(), base.name(), base.version());
        component.memory_maps = Some(memory_maps);

        Ok(component)
    }
}

fn register_to_ipxact(reg: &base::Register) -> Result<ipxact::Register, Error> {
    let mut register = ipxact::Register::new(reg.name(), reg.offset(), reg.size());

    for field in reg.fields() {
        // .filter(|field| {
        //     !re.is_match(field.name())
        // })
        let mut ipxact_field = ipxact::Field::new(field.name(), field.offset(), field.width());
        ipxact_field.description = non_empty_string(field.desc());
        ipxact_field.access = Some(access_kind(&extract_access_value(field.attr())?)?);
        ipxact_field.modified_write_value = extract_modified_write_value(field.attr())?
            .map(|value| modified_write_value_kind(&value).map(ipxact::ModifiedWriteValue::new))
            .transpose()?;
        ipxact_field.read_action = extract_read_action_value(field.attr())?
            .map(|value| read_action_kind(&value).map(ipxact::ReadAction::new))
            .transpose()?;
        ipxact_field.resets = Some(ipxact::Resets {
            reset: vec![ipxact::Reset::new(field.reset())],
        });

        register.add_field(ipxact_field);
    }

    Ok(register)
}

fn non_empty_string(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

fn access_kind(value: &str) -> Result<ipxact::Access, Error> {
    match value {
        "read-only" => Ok(ipxact::Access::ReadOnly),
        "write-only" => Ok(ipxact::Access::WriteOnly),
        "read-write" => Ok(ipxact::Access::ReadWrite),
        "writeOnce" => Ok(ipxact::Access::WriteOnce),
        "read-writeOnce" => Ok(ipxact::Access::ReadWriteOnce),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn access_value(field: &base::Field) -> Result<String, Error> {
    let value = extract_access_value(field.attr())?;
    access_kind(&value)?;
    Ok(value)
}

fn modified_write_value_kind(value: &str) -> Result<ipxact::ModifiedWriteValueKind, Error> {
    match value {
        "oneToClear" => Ok(ipxact::ModifiedWriteValueKind::OneToClear),
        "oneToSet" => Ok(ipxact::ModifiedWriteValueKind::OneToSet),
        "oneToToggle" => Ok(ipxact::ModifiedWriteValueKind::OneToToggle),
        "zeroToClear" => Ok(ipxact::ModifiedWriteValueKind::ZeroToClear),
        "zeroToSet" => Ok(ipxact::ModifiedWriteValueKind::ZeroToSet),
        "zeroToToggle" => Ok(ipxact::ModifiedWriteValueKind::ZeroToToggle),
        "clear" => Ok(ipxact::ModifiedWriteValueKind::Clear),
        "set" => Ok(ipxact::ModifiedWriteValueKind::Set),
        "modify" => Ok(ipxact::ModifiedWriteValueKind::Modify),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn modified_write_value(field: &base::Field) -> Result<Option<String>, Error> {
    extract_modified_write_value(field.attr())?
        .map(|value| modified_write_value_kind(&value).map(|_| value))
        .transpose()
}

fn read_action_kind(value: &str) -> Result<ipxact::ReadActionKind, Error> {
    match value {
        "clear" => Ok(ipxact::ReadActionKind::Clear),
        "set" => Ok(ipxact::ReadActionKind::Set),
        "modify" => Ok(ipxact::ReadActionKind::Modify),
        _ => Err(Error::InvalidAttribute {
            attribute: value.into(),
        }),
    }
}

fn read_action_value(field: &base::Field) -> Result<Option<String>, Error> {
    extract_read_action_value(field.attr())?
        .map(|value| read_action_kind(&value).map(|_| value))
        .transpose()
}
